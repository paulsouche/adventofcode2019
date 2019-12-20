#[macro_use]
extern crate lazy_static;
use regex::Regex;
use std::collections::HashMap;
use std::{fs::read_to_string, io};

fn main() -> io::Result<()> {
    let maze: Vec<Vec<char>> = read_to_string("src/bin/day18-many-worlds-interpretation/input.txt")?
        .trim()
        .split('\n')
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect();

    println!("{}", part1(&maze));

    Ok(())
}

fn part1(maze: &Vec<Vec<char>>) -> usize {
    let mut min_steps = usize::max_value();
    let ParsedMaze(start, keys_map) = parse_maze(maze);
    let paths_map = find_all_paths(maze, &keys_map, start);
    let mut walkers = vec![Walker::new('@', 0, None)];
    let number_of_keys = keys_map.keys().len();

    loop {
        match walkers.pop() {
            Some(walker) => {
                let possible_paths: Vec<&Path> = paths_map
                    .values()
                    .filter(|p| {
                        p.from_key == walker.key
                            && !walker.keys.contains(&p.to_key)
                            && p.keys_needed.iter().all(|k| walker.keys.contains(k))
                    })
                    .collect();

                for possible_path in possible_paths.iter() {
                    let new_distance = walker.distance + possible_path.distance;
                    if new_distance > 4000 {
                        continue;
                    }

                    let new_key = possible_path.to_key;
                    let mut new_keys = walker.keys.clone();
                    new_keys.push(new_key);
                    let new_walker = Walker::new(new_key, new_distance, Some(new_keys));

                    if new_walker.keys.len() < number_of_keys {
                        walkers.push(new_walker);
                        continue;
                    }

                    if min_steps > new_walker.distance {
                        min_steps = new_walker.distance
                    }
                }
            }
            _ => break,
        }
    }

    min_steps
}

fn parse_maze(maze: &Vec<Vec<char>>) -> ParsedMaze {
    let mut start: Option<Coordinate> = None;
    let mut keys_map: HashMap<char, Coordinate> = HashMap::new();
    for y in 0..maze.len() {
        for x in 0..maze[y].len() {
            let character = maze[y][x];
            if is_key(&character) {
                keys_map.insert(character, Coordinate::new(x, y, Some(character)));
            }

            if character == '@' {
                start = Some(Coordinate::new(x, y, Some('@')))
            }
        }
    }
    ParsedMaze(start.unwrap(), keys_map)
}

fn find_all_paths(
    maze: &Vec<Vec<char>>,
    keys_map: &HashMap<char, Coordinate>,
    start: Coordinate,
) -> HashMap<String, Path> {
    let mut paths: HashMap<String, Path> = HashMap::new();

    let mut coordinates = vec![start];
    for coordinate in keys_map.values() {
        coordinates.push(coordinate.clone());
    }

    for coordinate in coordinates.iter() {
        find_paths_between_keys(maze, &mut paths, coordinate);
    }

    paths
}

fn find_paths_between_keys(
    maze: &Vec<Vec<char>>,
    paths_map: &mut HashMap<String, Path>,
    start: &Coordinate,
) -> () {
    let mut walk: Vec<WalkStep> = vec![WalkStep::new(start.clone(), None)];
    let mut distances: HashMap<Coordinate, usize> = HashMap::new();
    distances.insert(start.clone(), usize::min_value());

    loop {
        match walk.pop() {
            Some(step) => {
                let distance = &(distances.get(&step.coordinate).unwrap() + 1);
                let directions = Coordinate::get_cardinal_coordinates(&step.coordinate);
                for direction in directions.iter() {
                    if distances.contains_key(direction)
                        && distances.get(direction).unwrap() <= distance
                    {
                        continue;
                    }

                    let mut keys_needed = step.copy_keys_needed();
                    match maze[direction.y][direction.x] {
                        '#' => continue,
                        character => {
                            if is_key(&character) {
                                paths_map.insert(
                                    Path::to_path_map_key(start.character.unwrap(), character),
                                    Path::new(
                                        start.character.unwrap(),
                                        character,
                                        distance.clone(),
                                        Some(step.copy_keys_needed()),
                                    ),
                                );
                            }

                            if is_door(&character) {
                                keys_needed.push(character.to_ascii_lowercase())
                            }

                            distances.insert(direction.clone(), distance.clone());
                            walk.push(WalkStep::new(direction.clone(), Some(keys_needed)));
                        }
                    }
                }
            }
            _ => break,
        }
    }
}

fn is_key(character: &char) -> bool {
    lazy_static! {
        static ref KEY_REG: Regex = Regex::new("[a-z]").unwrap();
    }
    KEY_REG.is_match(&character.to_string())
}

fn is_door(character: &char) -> bool {
    lazy_static! {
        static ref DOOR_REG: Regex = Regex::new("[A-Z]").unwrap();
    }
    DOOR_REG.is_match(&character.to_string())
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct ParsedMaze(Coordinate, HashMap<char, Coordinate>);

#[derive(Clone, Copy, Debug, PartialEq, Hash, Eq)]
struct Coordinate {
    x: usize,
    y: usize,
    character: Option<char>,
}

impl Coordinate {
    fn get_cardinal_coordinates(coordinate: &Coordinate) -> Vec<Coordinate> {
        vec![
            Coordinate::new(coordinate.x, coordinate.y - 1, coordinate.character),
            Coordinate::new(coordinate.x + 1, coordinate.y, coordinate.character),
            Coordinate::new(coordinate.x, coordinate.y + 1, coordinate.character),
            Coordinate::new(coordinate.x - 1, coordinate.y, coordinate.character),
        ]
    }

    fn new(x: usize, y: usize, character: Option<char>) -> Coordinate {
        Coordinate {
            x: x,
            y: y,
            character: character,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Hash, Eq)]
struct Path {
    from_key: char,
    to_key: char,
    distance: usize,
    keys_needed: Vec<char>,
}

impl Path {
    fn to_path_map_key(char_start: char, char_to: char) -> String {
        String::from(char_start.to_string() + "=>" + &char_to.to_string())
    }

    fn new(from_key: char, to_key: char, distance: usize, keys_needed: Option<Vec<char>>) -> Path {
        Path {
            from_key: from_key,
            to_key: to_key,
            distance: distance,
            keys_needed: match keys_needed {
                Some(k) => k,
                _ => vec![],
            },
        }
    }
}

#[derive(Clone, Debug, PartialEq, Hash, Eq)]
struct WalkStep {
    coordinate: Coordinate,
    keys_needed: Vec<char>,
}

impl WalkStep {
    fn new(coordinate: Coordinate, keys_needed: Option<Vec<char>>) -> WalkStep {
        WalkStep {
            coordinate: coordinate,
            keys_needed: match keys_needed {
                Some(k) => k,
                _ => vec![],
            },
        }
    }

    fn copy_keys_needed(&self) -> Vec<char> {
        self.keys_needed.iter().map(|k| k.clone()).collect()
    }
}

#[derive(Clone, Debug, PartialEq, Hash, Eq)]
struct Walker {
    key: char,
    distance: usize,
    keys: Vec<char>,
}

impl Walker {
    fn new(key: char, distance: usize, keys: Option<Vec<char>>) -> Walker {
        Walker {
            key: key,
            distance: distance,
            keys: match keys {
                Some(k) => k,
                _ => vec![],
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::part1;

    fn to_maze(string: String) -> Vec<Vec<char>> {
        string
            .trim()
            .split('\n')
            .map(|line| line.trim().chars().collect())
            .collect()
    }

    #[test]
    fn it_should_find_the_shortest_path_1() {
        assert_eq!(
            part1(&to_maze(String::from(
                "
                #########
                #b.A.@.a#
                #########"
            ))),
            8
        );
    }

    #[test]
    fn it_should_find_the_shortest_path_2() {
        assert_eq!(
            part1(&to_maze(String::from(
                "
                ########################
                #f.D.E.e.C.b.A.@.a.B.c.#
                ######################.#
                #d.....................#
                ########################"
            ))),
            86
        );
    }

    #[test]
    fn it_should_find_the_shortest_path_3() {
        assert_eq!(
            part1(&to_maze(String::from(
                "
                ########################
                #...............b.C.D.f#
                #.######################
                #.....@.a.B.c.d.A.e.F.g#
                ########################"
            ))),
            132
        );
    }

    #[test]
    fn it_should_find_the_shortest_path_4() {
        assert_eq!(
            part1(&to_maze(String::from(
                "
                #################
                #i.G..c...e..H.p#
                ########.########
                #j.A..b...f..D.o#
                ########@########
                #k.E..a...g..B.n#
                ########.########
                #l.F..d...h..C.m#
                #################"
            ))),
            136
        );
    }

    #[test]
    fn it_should_find_the_shortest_path_5() {
        assert_eq!(
            part1(&to_maze(String::from(
                "
                ########################
                #@..............ac.GI.b#
                ###d#e#f################
                ###A#B#C################
                ###g#h#i################
                ########################"
            ))),
            81
        );
    }
}
