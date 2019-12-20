use std::collections::HashMap;
use std::{fs::read_to_string, io};

fn main() -> io::Result<()> {
    let file = read_to_string("src/bin/day03-crossed-wires/input.txt")?;
    let input: Vec<Vec<&str>> = file
        .trim()
        .split('\n')
        .map(|line| line.split(",").collect())
        .collect();

    let tuple = walk_panel(input);

    println!("{}", tuple.0);
    println!("{}", tuple.1);

    Ok(())
}

fn get_coordinates(coordinates: &Vec<i32>) -> String {
    coordinates[0].to_string() + "," + &coordinates[1].to_string()
}

fn walk_panel(wires: Vec<Vec<&str>>) -> (i32, i32) {
    let mut panel: HashMap<String, i32> = HashMap::new();
    let mut steps: Vec<HashMap<String, i32>> = vec![];
    let mut wire_index: i32 = 0;
    let mut step_index: usize = 0;

    for wire in wires.iter() {
        wire_index += 1;
        let mut coords: Vec<i32> = vec![0, 0];
        let mut step: i32 = 0;
        steps.push(HashMap::new());

        for movement in wire.iter() {
            let direction = &movement[0..1];
            let times = &movement[1..].parse::<i32>().unwrap() + 1;
            let coord: usize;
            let walk: i32;
            match direction {
                "R" => {
                    coord = 0;
                    walk = 1;
                }
                "L" => {
                    coord = 0;
                    walk = -1;
                }
                "U" => {
                    coord = 1;
                    walk = 1;
                }
                "D" => {
                    coord = 1;
                    walk = -1;
                }
                n => panic!("unknown direction: {}", n),
            }

            for _ in 1..times {
                coords[coord] += walk;
                step += 1;
                let times_was_here: i32;
                match panel.get(&get_coordinates(&coords)) {
                    Some(&number) => {
                        if number != wire_index {
                            times_was_here = wire_index + number;
                        } else {
                            times_was_here = wire_index;
                        }
                    }
                    _ => times_was_here = wire_index,
                }
                panel.insert(get_coordinates(&coords), times_was_here);
                steps[step_index].insert(get_coordinates(&coords), step);
            }
        }
        step_index += 1;
    }

    let mut min_manhattan_distance = i32::max_value();
    let mut min_steps_sum = i32::max_value();

    for (coords, times_crossed) in panel.iter() {
        if times_crossed > &wire_index {
            let manhattan_distance = coords
                .split(",")
                .map(|c| c.parse::<i32>().unwrap().abs())
                .sum();
            let steps_sum: i32 = steps
                .iter()
                .map(|hashmap| match hashmap.get(coords) {
                    Some(&number) => number,
                    _ => panic!("No step for crossing: {}", coords),
                })
                .sum();
            if manhattan_distance < min_manhattan_distance {
                min_manhattan_distance = manhattan_distance;
            }

            if steps_sum < min_steps_sum {
                min_steps_sum = steps_sum;
            }
        }
    }
    (min_manhattan_distance, min_steps_sum)
}

#[cfg(test)]
mod tests {
    use super::walk_panel;

    #[test]
    fn it_should_find_lowest_manhattan_distance_1() {
        let wires: Vec<Vec<&str>> =
            vec![vec!["R8", "U5", "L5", "D3"], vec!["U7", "R6", "D4", "L4"]];
        assert_eq!(walk_panel(wires).0, 6);
    }

    #[test]
    fn it_should_find_lowest_manhattan_distance_2() {
        let wires: Vec<Vec<&str>> = vec![
            vec!["R75", "D30", "R83", "U83", "L12", "D49", "R71", "U7", "L72"],
            vec!["U62", "R66", "U55", "R34", "D71", "R55", "D58", "R83"],
        ];
        assert_eq!(walk_panel(wires).0, 159);
    }

    #[test]
    fn it_should_find_lowest_manhattan_distance_3() {
        let wires: Vec<Vec<&str>> = vec![
            vec![
                "R98", "U47", "R26", "D63", "R33", "U87", "L62", "D20", "R33", "U53", "R51",
            ],
            vec![
                "U98", "R91", "D20", "R16", "D67", "R40", "U7", "R15", "U6", "R7",
            ],
        ];
        assert_eq!(walk_panel(wires).0, 135);
    }

    #[test]
    fn it_should_find_lowest_steps_1() {
        let wires: Vec<Vec<&str>> =
            vec![vec!["R8", "U5", "L5", "D3"], vec!["U7", "R6", "D4", "L4"]];
        assert_eq!(walk_panel(wires).1, 30);
    }

    #[test]
    fn it_should_find_lowest_steps_2() {
        let wires: Vec<Vec<&str>> = vec![
            vec!["R75", "D30", "R83", "U83", "L12", "D49", "R71", "U7", "L72"],
            vec!["U62", "R66", "U55", "R34", "D71", "R55", "D58", "R83"],
        ];
        assert_eq!(walk_panel(wires).1, 610);
    }

    #[test]
    fn it_should_find_lowest_steps_3() {
        let wires: Vec<Vec<&str>> = vec![
            vec![
                "R98", "U47", "R26", "D63", "R33", "U87", "L62", "D20", "R33", "U53", "R51",
            ],
            vec![
                "U98", "R91", "D20", "R16", "D67", "R40", "U7", "R15", "U6", "R7",
            ],
        ];
        assert_eq!(walk_panel(wires).1, 410);
    }
}
