use std::collections::HashMap;
use std::{collections::VecDeque, fs::read_to_string, io};

fn main() -> io::Result<()> {
    let program: Vec<isize> = read_to_string("src/bin/day15-oxygen-system/input.txt")?
        .trim()
        .split(',')
        .map(|line| line.parse::<isize>().unwrap())
        .collect();

    let mut maze: HashMap<String, Coordinates> = HashMap::new();
    let oxygen_center = walk_program(&program, &mut maze).unwrap();
    println!("{}", oxygen_center.distance);
    println!("{}", spread_oxygen(&mut maze, &oxygen_center));

    Ok(())
}

fn spread_oxygen(maze: &mut HashMap<String, Coordinates>, oxygen_center: &Coordinates) -> usize {
    let oxygenated = Coordinates::new(
        oxygen_center.x,
        oxygen_center.y,
        None,
        Some(CoordinatesKind::Oxygen),
    );
    let mut oxygen_coordinates: Vec<Coordinates> = vec![oxygenated];
    maze.insert(
        oxygen_center.to_maze_key(),
        Coordinates::new(
            oxygen_center.x,
            oxygen_center.y,
            None,
            Some(CoordinatesKind::Oxygen),
        ),
    );

    loop {
        match oxygen_coordinates.pop() {
            Some(coordinates) => {
                match maze.get(&get_north_key(&coordinates)).unwrap().kind {
                    CoordinatesKind::DeadEnd => {
                        let oxygenated = get_north_coordinates(&coordinates);
                        maze.insert(oxygenated.to_maze_key(), oxygenated);
                        oxygen_coordinates.push(get_north_coordinates(&coordinates));
                    }
                    _ => (),
                }

                match maze.get(&get_east_key(&coordinates)).unwrap().kind {
                    CoordinatesKind::DeadEnd => {
                        let oxygenated = get_east_coordinates(&coordinates);
                        maze.insert(oxygenated.to_maze_key(), oxygenated);
                        oxygen_coordinates.push(get_east_coordinates(&coordinates));
                    }
                    _ => (),
                }

                match maze.get(&get_south_key(&coordinates)).unwrap().kind {
                    CoordinatesKind::DeadEnd => {
                        let oxygenated = get_south_coordinates(&coordinates);
                        maze.insert(oxygenated.to_maze_key(), oxygenated);
                        oxygen_coordinates.push(get_south_coordinates(&coordinates));
                    }
                    _ => (),
                }

                match maze.get(&get_west_key(&coordinates)).unwrap().kind {
                    CoordinatesKind::DeadEnd => {
                        let oxygenated = get_west_coordinates(&coordinates);
                        maze.insert(oxygenated.to_maze_key(), oxygenated);
                        oxygen_coordinates.push(get_west_coordinates(&coordinates));
                    }
                    _ => (),
                }

                // print_maze(&maze, &coordinates, &Direction::North);
            }
            _ => break,
        }
    }

    let mut oxygen_minutes = usize::min_value();
    for coordinate in maze.values().filter(|c| c.kind == CoordinatesKind::Oxygen) {
        if coordinate.distance > oxygen_minutes {
            oxygen_minutes = coordinate.distance;
        }
    }
    oxygen_minutes
}

fn walk_program(
    program: &Vec<isize>,
    maze: &mut HashMap<String, Coordinates>,
) -> Option<Coordinates> {
    let mut drone_coordinates = Coordinates::new(0, 0, None, None);
    let mut current_direction = Direction::North;
    let mut needs_return = false;
    let mut oxygen_center: Option<Coordinates> = None;
    maze.insert(drone_coordinates.to_maze_key(), drone_coordinates.clone());
    let mut computer = IntcodeComputer::new();
    computer.reset();
    computer.push_instructions(program.clone());

    loop {
        match computer.run() {
            ProgramState::Halted => break,
            ProgramState::NeedInput => {
                // print_maze(&maze, &drone_coordinates, &current_direction);

                match current_direction {
                    Direction::North => computer.push_input(1),
                    Direction::East => computer.push_input(4),
                    Direction::South => computer.push_input(2),
                    Direction::West => computer.push_input(3),
                };
            }
            ProgramState::Output(out) => match out {
                0 => {
                    update_maze(
                        maze,
                        &drone_coordinates,
                        &current_direction,
                        &CoordinatesKind::Wall,
                    );

                    if !was_on_each_cell(&maze, &drone_coordinates) {
                        match current_direction {
                            Direction::North => {
                                if !has_walked_east(&maze, &drone_coordinates) {
                                    current_direction = Direction::East;
                                } else if !has_walked_south(&maze, &drone_coordinates) {
                                    current_direction = Direction::South;
                                } else {
                                    current_direction = Direction::West;
                                }
                            }
                            Direction::East => {
                                if !has_walked_south(&maze, &drone_coordinates) {
                                    current_direction = Direction::South;
                                } else {
                                    current_direction = Direction::West;
                                }
                            }
                            Direction::South => current_direction = Direction::West,
                            Direction::West => current_direction = Direction::North,
                        }
                        continue;
                    }

                    if is_north_free(&maze, &drone_coordinates) {
                        current_direction = Direction::North;
                        continue;
                    }

                    if is_east_free(&maze, &drone_coordinates) {
                        current_direction = Direction::East;
                        continue;
                    }

                    if is_south_free(&maze, &drone_coordinates) {
                        current_direction = Direction::South;
                        continue;
                    }

                    if is_west_free(&maze, &drone_coordinates) {
                        current_direction = Direction::West;
                        continue;
                    }

                    break;
                }
                1 | 2 => {
                    if needs_return {
                        needs_return = false;
                    } else {
                        update_maze(
                            maze,
                            &drone_coordinates,
                            &current_direction,
                            &CoordinatesKind::Free,
                        );
                        needs_return = !was_on_each_cell(&maze, &drone_coordinates);
                        if needs_return {
                            current_direction = match current_direction {
                                Direction::North => Direction::South,
                                Direction::East => Direction::West,
                                Direction::South => Direction::North,
                                Direction::West => Direction::East,
                            };
                            continue;
                        }
                    }

                    // Explore each tile
                    if !map_has_north(&maze, &drone_coordinates) {
                        current_direction = Direction::North;
                        continue;
                    }

                    if !map_has_east(&maze, &drone_coordinates) {
                        current_direction = Direction::East;
                        continue;
                    }

                    if !map_has_south(&maze, &drone_coordinates) {
                        current_direction = Direction::South;
                        continue;
                    }

                    if !map_has_west(&maze, &drone_coordinates) {
                        current_direction = Direction::West;
                        continue;
                    }

                    update_drone(&mut drone_coordinates, &current_direction, &maze);

                    if out == 2 {
                        oxygen_center = Some(drone_coordinates.clone());
                    }

                    // Walk each tile
                    if !has_walked_north(&maze, &drone_coordinates) {
                        current_direction = Direction::North;
                        continue;
                    }

                    if !has_walked_east(&maze, &drone_coordinates) {
                        current_direction = Direction::East;
                        continue;
                    }

                    if !has_walked_south(&maze, &drone_coordinates) {
                        current_direction = Direction::South;
                        continue;
                    }

                    if !has_walked_west(&maze, &drone_coordinates) {
                        current_direction = Direction::West;
                        continue;
                    }

                    // Dead end
                    let key = &drone_coordinates.to_maze_key();
                    let mut cell = maze.get(key).unwrap().clone();
                    cell.kind = CoordinatesKind::DeadEnd;
                    maze.insert(key.clone(), cell);

                    if is_north_free(&maze, &drone_coordinates) {
                        current_direction = Direction::North;
                        continue;
                    }

                    if is_east_free(&maze, &drone_coordinates) {
                        current_direction = Direction::East;
                        continue;
                    }

                    if is_south_free(&maze, &drone_coordinates) {
                        current_direction = Direction::South;
                        continue;
                    }

                    if is_west_free(&maze, &drone_coordinates) {
                        current_direction = Direction::West;
                        continue;
                    }

                    break;
                }
                n => panic!("Unkonwn output {}", n),
            },
        }
    }

    oxygen_center
}

fn update_maze(
    maze: &mut HashMap<String, Coordinates>,
    drone_coordinates: &Coordinates,
    current_direction: &Direction,
    kind: &CoordinatesKind,
) -> () {
    let coordinates = match current_direction {
        Direction::North => Coordinates::new(
            drone_coordinates.x,
            drone_coordinates.y + 1,
            Some(drone_coordinates.distance + 1),
            Some(kind.clone()),
        ),
        Direction::East => Coordinates::new(
            drone_coordinates.x + 1,
            drone_coordinates.y,
            Some(drone_coordinates.distance + 1),
            Some(kind.clone()),
        ),
        Direction::South => Coordinates::new(
            drone_coordinates.x,
            drone_coordinates.y - 1,
            Some(drone_coordinates.distance + 1),
            Some(kind.clone()),
        ),
        Direction::West => Coordinates::new(
            drone_coordinates.x - 1,
            drone_coordinates.y,
            Some(drone_coordinates.distance + 1),
            Some(kind.clone()),
        ),
    };

    let key = coordinates.to_maze_key();
    if !maze.contains_key(&key) {
        maze.insert(coordinates.to_maze_key(), coordinates.clone());
    } else {
        if maze.get(&key).unwrap().distance > coordinates.distance {
            maze.insert(coordinates.to_maze_key(), coordinates.clone());
        }
    }
}

fn update_drone(
    coordinate: &mut Coordinates,
    current_direction: &Direction,
    maze: &HashMap<String, Coordinates>,
) {
    match current_direction {
        Direction::North => coordinate.y += 1,
        Direction::East => coordinate.x += 1,
        Direction::South => coordinate.y -= 1,
        Direction::West => coordinate.x -= 1,
    };
    let distance = match maze.get(&coordinate.to_maze_key()) {
        Some(c) => c.distance,
        _ => coordinate.distance + 1,
    };
    coordinate.distance = distance;
}

fn was_on_each_cell(maze: &HashMap<String, Coordinates>, drone_coordinates: &Coordinates) -> bool {
    map_has_north(maze, drone_coordinates)
        && map_has_east(maze, drone_coordinates)
        && map_has_south(maze, drone_coordinates)
        && map_has_west(maze, drone_coordinates)
}

fn map_has_north(maze: &HashMap<String, Coordinates>, drone_coordinates: &Coordinates) -> bool {
    map_has_coordinates(maze, &get_north_key(drone_coordinates))
}

fn map_has_east(maze: &HashMap<String, Coordinates>, drone_coordinates: &Coordinates) -> bool {
    map_has_coordinates(maze, &get_east_key(drone_coordinates))
}

fn map_has_south(maze: &HashMap<String, Coordinates>, drone_coordinates: &Coordinates) -> bool {
    map_has_coordinates(maze, &get_south_key(drone_coordinates))
}

fn map_has_west(maze: &HashMap<String, Coordinates>, drone_coordinates: &Coordinates) -> bool {
    map_has_coordinates(maze, &get_west_key(drone_coordinates))
}

fn map_has_coordinates(maze: &HashMap<String, Coordinates>, key: &String) -> bool {
    maze.contains_key(key)
}

fn has_walked_north(maze: &HashMap<String, Coordinates>, drone_coordinates: &Coordinates) -> bool {
    has_walked_coordinate(
        maze,
        &get_north_key(drone_coordinates),
        &drone_coordinates.distance,
    )
}

fn has_walked_east(maze: &HashMap<String, Coordinates>, drone_coordinates: &Coordinates) -> bool {
    has_walked_coordinate(
        maze,
        &get_east_key(drone_coordinates),
        &drone_coordinates.distance,
    )
}

fn has_walked_south(maze: &HashMap<String, Coordinates>, drone_coordinates: &Coordinates) -> bool {
    has_walked_coordinate(
        maze,
        &get_south_key(drone_coordinates),
        &drone_coordinates.distance,
    )
}

fn has_walked_west(maze: &HashMap<String, Coordinates>, drone_coordinates: &Coordinates) -> bool {
    has_walked_coordinate(
        maze,
        &get_west_key(drone_coordinates),
        &drone_coordinates.distance,
    )
}

fn has_walked_coordinate(
    maze: &HashMap<String, Coordinates>,
    key: &String,
    distance: &usize,
) -> bool {
    if !maze.contains_key(key) {
        return false;
    }

    let coordinate = maze.get(key).unwrap();
    match coordinate.kind {
        CoordinatesKind::Wall | CoordinatesKind::DeadEnd => true,
        CoordinatesKind::Free | CoordinatesKind::Oxygen => &coordinate.distance <= distance,
    }
}

fn is_north_free(maze: &HashMap<String, Coordinates>, drone_coordinates: &Coordinates) -> bool {
    is_coordinate_free(maze, &get_north_key(drone_coordinates))
}

fn is_east_free(maze: &HashMap<String, Coordinates>, drone_coordinates: &Coordinates) -> bool {
    is_coordinate_free(maze, &get_east_key(drone_coordinates))
}

fn is_south_free(maze: &HashMap<String, Coordinates>, drone_coordinates: &Coordinates) -> bool {
    is_coordinate_free(maze, &get_south_key(drone_coordinates))
}

fn is_west_free(maze: &HashMap<String, Coordinates>, drone_coordinates: &Coordinates) -> bool {
    is_coordinate_free(maze, &get_west_key(drone_coordinates))
}

fn is_coordinate_free(maze: &HashMap<String, Coordinates>, key: &String) -> bool {
    if !maze.contains_key(key) {
        return true;
    }

    let coordinate = maze.get(key).unwrap();
    match coordinate.kind {
        CoordinatesKind::Wall | CoordinatesKind::DeadEnd => false,
        CoordinatesKind::Free | CoordinatesKind::Oxygen => true,
    }
}

fn get_north_key(coordinates: &Coordinates) -> String {
    get_north_coordinates(coordinates).to_maze_key()
}

fn get_east_key(coordinates: &Coordinates) -> String {
    get_east_coordinates(coordinates).to_maze_key()
}

fn get_south_key(coordinates: &Coordinates) -> String {
    get_south_coordinates(coordinates).to_maze_key()
}

fn get_west_key(coordinates: &Coordinates) -> String {
    get_west_coordinates(coordinates).to_maze_key()
}

fn get_north_coordinates(coordinates: &Coordinates) -> Coordinates {
    Coordinates::new(
        coordinates.x,
        coordinates.y + 1,
        Some(coordinates.distance + 1),
        Some(coordinates.kind.clone()),
    )
}

fn get_east_coordinates(coordinates: &Coordinates) -> Coordinates {
    Coordinates::new(
        coordinates.x + 1,
        coordinates.y,
        Some(coordinates.distance + 1),
        Some(coordinates.kind.clone()),
    )
}

fn get_south_coordinates(coordinates: &Coordinates) -> Coordinates {
    Coordinates::new(
        coordinates.x,
        coordinates.y - 1,
        Some(coordinates.distance + 1),
        Some(coordinates.kind.clone()),
    )
}

fn get_west_coordinates(coordinates: &Coordinates) -> Coordinates {
    Coordinates::new(
        coordinates.x - 1,
        coordinates.y,
        Some(coordinates.distance + 1),
        Some(coordinates.kind.clone()),
    )
}

fn print_maze(
    maze: &HashMap<String, Coordinates>,
    drone_coordinates: &Coordinates,
    current_direction: &Direction,
) -> () {
    let mut min_x = isize::max_value();
    let mut max_x = isize::min_value();
    let mut min_y = isize::max_value();
    let mut max_y = isize::min_value();

    for coordinate in maze.values() {
        if coordinate.x > max_x {
            max_x = coordinate.x
        }

        if coordinate.x < min_x {
            min_x = coordinate.x
        }

        if coordinate.y > max_y {
            max_y = coordinate.y
        }

        if coordinate.y < min_y {
            min_y = coordinate.y
        }
    }

    let mut graphic_maze: Vec<Vec<String>> = vec![];

    for y in min_y..max_y + 1 {
        let mut line: Vec<String> = vec![];
        for x in min_x..max_x + 1 {
            if x == drone_coordinates.x && y == drone_coordinates.y {
                match current_direction {
                    Direction::North => line.push("^".to_owned()),
                    Direction::East => line.push(">".to_owned()),
                    Direction::South => line.push("v".to_owned()),
                    Direction::West => line.push("<".to_owned()),
                }
                continue;
            }

            let key = Coordinates::new(x, y, None, None).to_maze_key();
            if maze.contains_key(&key) {
                let coords = maze.get(&key).unwrap();
                match coords.kind {
                    CoordinatesKind::Wall => line.push("#".to_owned()),
                    CoordinatesKind::DeadEnd => line.push("X".to_owned()),
                    CoordinatesKind::Oxygen => line.push("O".to_owned()),
                    CoordinatesKind::Free => {
                        line.push((coords.distance % 10).to_string().to_owned())
                    }
                }
            } else {
                line.push(" ".to_owned());
            }
        }
        graphic_maze.push(line)
    }

    graphic_maze.reverse();

    println!("");
    println!(
        "{}",
        graphic_maze
            .iter()
            .map(|l| l.join(""))
            .collect::<Vec<String>>()
            .join("\n")
    );
    println!("");
}

#[derive(Debug)]
enum Direction {
    North,
    East,
    South,
    West,
}

#[derive(Clone, Debug, PartialEq)]
enum CoordinatesKind {
    Wall,
    DeadEnd,
    Free,
    Oxygen,
}

#[derive(Clone, Debug)]
struct Coordinates {
    x: isize,
    y: isize,
    distance: usize,
    kind: CoordinatesKind,
}

impl Coordinates {
    fn new(x: isize, y: isize, distance: Option<usize>, kind: Option<CoordinatesKind>) -> Self {
        Coordinates {
            x: x,
            y: y,
            distance: match distance {
                Some(d) => d,
                _ => 0,
            },
            kind: match kind {
                Some(k) => k,
                _ => CoordinatesKind::Free,
            },
        }
    }

    fn to_maze_key(&self) -> String {
        self.x.to_string() + "," + &self.y.to_string()
    }
}

enum ProgramState {
    Halted,
    NeedInput,
    Output(isize),
}

struct IntcodeComputer {
    instructions: HashMap<isize, isize>,
    pointer: usize,
    relative_base: isize,
    inputs: VecDeque<isize>,
}

impl IntcodeComputer {
    fn new() -> Self {
        IntcodeComputer {
            instructions: HashMap::new(),
            pointer: 0,
            relative_base: 0,
            inputs: VecDeque::new(),
        }
    }

    fn reset(&mut self) {
        self.instructions.clear();
        self.pointer = 0;
        self.relative_base = 0;
        self.inputs.clear();
    }

    fn push_instructions(&mut self, program: Vec<isize>) {
        for ind in 0..program.len() {
            self.instructions.insert(ind as isize, program[ind]);
        }
    }

    fn push_input(&mut self, input: isize) {
        self.inputs.push_back(input);
    }

    fn run(&mut self) -> ProgramState {
        if self.instructions[&(self.pointer as isize)] == 99 {
            return ProgramState::Halted;
        }

        let mut output: Option<isize> = None;

        loop {
            if let Some(out) = output {
                return ProgramState::Output(out);
            }

            let value = self.instructions[&(self.pointer as isize)];

            let operator = value % 100;
            let mode1 = (value - (value / 1000) * 1000) / 100;
            let mode2 = (value - (value / 10000) * 10000) / 1000;
            let mode3 = (value - (value / 100000) * 100000) / 10000;

            let param1 = self.get_paramter(&mode1, &(self.pointer as isize + 1));
            let param2 = self.get_paramter(&mode2, &(self.pointer as isize + 2));
            let param3 = self.get_paramter(&mode3, &(self.pointer as isize + 3));

            let value1 = self.get_value(&param1);
            let value2 = self.get_value(&param2);

            match operator {
                99 => break,
                1 => {
                    self.instructions.insert(param3, value1 + value2);
                    self.pointer += 4;
                }
                2 => {
                    self.instructions.insert(param3, value1 * value2);
                    self.pointer += 4;
                }
                3 => {
                    self.instructions.insert(
                        param1,
                        match self.inputs.pop_front() {
                            Some(x) => x,
                            None => return ProgramState::NeedInput,
                        },
                    );
                    self.pointer += 2;
                }
                4 => {
                    output = Some(value1);
                    self.pointer += 2;
                }
                5 => {
                    if value1 != 0 {
                        self.pointer = value2 as usize;
                    } else {
                        self.pointer += 3;
                    }
                }
                6 => {
                    if value1 == 0 {
                        self.pointer = value2 as usize;
                    } else {
                        self.pointer += 3;
                    }
                }
                7 => {
                    if value1 < value2 {
                        self.instructions.insert(param3, 1);
                    } else {
                        self.instructions.insert(param3, 0);
                    }
                    self.pointer += 4;
                }
                8 => {
                    if value1 == value2 {
                        self.instructions.insert(param3, 1);
                    } else {
                        self.instructions.insert(param3, 0);
                    }
                    self.pointer += 4;
                }
                9 => {
                    self.relative_base += value1;
                    self.pointer += 2;
                }
                n => panic!("Unknown opcode {}", n),
            }
        }

        if let Some(out) = output {
            return ProgramState::Output(out);
        }

        ProgramState::Halted
    }

    fn get_paramter(&self, mode: &isize, ind: &isize) -> isize {
        match mode {
            0 => match self.instructions.get(&ind) {
                Some(x) => x.clone(),
                _ => 0,
            },
            1 => ind.clone(),
            2 => match self.instructions.get(&ind) {
                Some(x) => (self.relative_base + x.clone()),
                _ => self.relative_base,
            },
            n => panic!("Unknown mode {}", n),
        }
    }

    fn get_value(&self, param: &isize) -> isize {
        match self.instructions.get(param) {
            Some(x) => x.clone(),
            _ => 0,
        }
    }
}
