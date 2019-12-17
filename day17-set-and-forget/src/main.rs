use std::collections::HashMap;
use std::{collections::VecDeque, fs::read_to_string, io};

fn main() -> io::Result<()> {
    let program: Vec<isize> = read_to_string("input.txt")?
        .trim()
        .split(',')
        .map(|line| line.parse::<isize>().unwrap())
        .collect();

    let mut maze: HashMap<String, Coordinate> = HashMap::new();
    println!("{}", walk_program(&program, &mut maze));
    println!("{}", find_sequence(&maze));
    // Output
    // A,B,A,C,B,C,B,C,A,C
    // A => L,10,R,12,R,12
    // B => R,6,R,10,L,10
    // C => R,10,L,10,L,12,R,6
    let inputs = String::from("A,B,A,C,B,C,B,C,A,C\nL,10,R,12,R,12\nR,6,R,10,L,10\nR,10,L,10,L,12,R,6\nn\n");
    println!("{}", save_robots(&program, &inputs));

    Ok(())
}

fn save_robots(program: &Vec<isize>, inputs_str: &str) -> isize {
    let mut computer = IntcodeComputer::new();
    let mut instructions = program.clone();
    let ascii_map: HashMap<char, isize> = get_ascii_map();
    let mut inputs: Vec<isize> = inputs_str.chars().rev().map(|c| ascii_map.get(&c).unwrap().clone()).collect();
    // override movement logic
    instructions[0] = 2;
    computer.reset();
    computer.push_instructions(instructions);

    let mut star_dust = 0;

    loop {
        match computer.run() {
            ProgramState::Halted => break,
            ProgramState::NeedInput => computer.push_input(inputs.pop().unwrap()),
            ProgramState::Output(out) => {
                star_dust = out;
            }
        }
    }
    star_dust
}

fn get_ascii_map () -> HashMap<char, isize> {
    let mut ascii_map: HashMap<char, isize> = HashMap::new();
    ascii_map.insert('\n', 10);
    ascii_map.insert(',', 44);
    ascii_map.insert('0', 48);
    ascii_map.insert('1', 49);
    ascii_map.insert('2', 50);
    ascii_map.insert('3', 51);
    ascii_map.insert('4', 52);
    ascii_map.insert('5', 53);
    ascii_map.insert('6', 54);
    ascii_map.insert('7', 55);
    ascii_map.insert('8', 56);
    ascii_map.insert('9', 57);
    ascii_map.insert('A', 65);
    ascii_map.insert('B', 66);
    ascii_map.insert('C', 67);
    ascii_map.insert('L', 76);
    ascii_map.insert('R', 82);
    ascii_map.insert('n', 110);
    ascii_map
}

fn find_sequence(maze: &HashMap<String, Coordinate>) -> String {
    let mut robot = maze
        .values()
        .find(|c| c.camera_output.unwrap() == '^')
        .unwrap()
        .clone();
    robot.camera_output = Some('X');
    let mut walked_maze = maze.clone();
    let mut current_direction = Direction::North;
    let mut sequence: Vec<String> = vec![];

    loop {
        let new_direction = find_direction(&walked_maze, &robot, &current_direction);
        match new_direction {
            Some(direction) => {
                if direction != current_direction {
                    sequence.push(get_rotate_instruction(&current_direction, &direction))
                }
                current_direction = direction;

                let mut step = 0;
                loop {
                    let key = Coordinate::get_direction_maze_key(&robot, &Some(&current_direction));


                    if !walked_maze.contains_key(&key) {
                        break;
                    }

                    let tile_camera_output = walked_maze.get(&key).unwrap().camera_output.unwrap();

                    if tile_camera_output == '.' || tile_camera_output == '\n' {
                        break;
                    }

                    step += 1;
                    match current_direction {
                        Direction::North => robot.y -= 1,
                        Direction::East => robot.x += 1,
                        Direction::South => robot.y += 1,
                        Direction::West => robot.x -= 1,
                    }
                    walked_maze.insert(robot.to_maze_key(), robot.clone());
                }
                sequence.push(step.to_string());
            }
            _ => break,
        }
    }

    sequence.join(",")
}

fn find_direction(
    maze: &HashMap<String, Coordinate>,
    robot: &Coordinate,
    current_direction: &Direction,
) -> Option<Direction> {
    let mut new_direction: Direction = current_direction.clone();
    loop {
        let key = Coordinate::get_direction_maze_key(&robot, &Some(&new_direction));

        if maze.contains_key(&key) {
            let camera_output = maze.get(&key).unwrap().camera_output.unwrap();
            if camera_output != '.' && camera_output != 'X' && camera_output != '\n' {
                return Some(new_direction);
            }
        }

        new_direction = turn_right(new_direction);

        if &new_direction == current_direction {
            return None;
        }
    }
}

fn turn_right(direction: Direction) -> Direction {
    match direction {
        Direction::North => Direction::East,
        Direction::East => Direction::South,
        Direction::South => Direction::West,
        Direction::West => Direction::North,
    }
}

fn get_rotate_instruction(direction1: &Direction, direction2: &Direction) -> String {
    let l = String::from("L");
    let r = String::from("R");

    match direction1 {
        Direction::North => match direction2 {
            Direction::North => panic!("No need to turn"),
            Direction::East => r,
            Direction::South => panic!("Cannot turn backward"),
            Direction::West => l,
        },
        Direction::East => match direction2 {
            Direction::North => l,
            Direction::East => panic!("No need to turn"),
            Direction::South => r,
            Direction::West => panic!("Cannot turn backward"),
        },
        Direction::South => match direction2 {
            Direction::North => panic!("Cannot turn backward"),
            Direction::East => l,
            Direction::South => panic!("No need to turn"),
            Direction::West => r,
        },
        Direction::West => match direction2 {
            Direction::North => r,
            Direction::East => panic!("Cannot turn backward"),
            Direction::South => l,
            Direction::West => panic!("No need to turn"),
        },
    }
}

fn walk_program(program: &Vec<isize>, maze: &mut HashMap<String, Coordinate>) -> isize {
    let mut computer = IntcodeComputer::new();
    let mut coords = Coordinate::new(0, 0, None);
    let mut max_x = 0;
    let mut max_y = 0;
    computer.reset();
    computer.push_instructions(program.clone());

    loop {
        match computer.run() {
            ProgramState::Halted => break,
            ProgramState::NeedInput => panic!("No need for input"),
            ProgramState::Output(out) => {
                coords.camera_output = Some((out as u8) as char);
                maze.insert(coords.to_maze_key(), coords.clone());

                if coords.x > max_x {
                    max_x = coords.x
                }

                if coords.y > max_y {
                    max_y = coords.y
                }

                match out {
                    10 => {
                        coords.x = 0;
                        coords.y += 1;
                    }
                    _ => coords.x += 1,
                }

                if coords.y < 3 || coords.x < 1 || coords.x > max_x - 1 {
                    continue;
                }

                let mut above_coordinate = coords.clone();
                above_coordinate.y -= 2;

                if is_alignement(&above_coordinate, &maze) {
                    above_coordinate.camera_output = Some('O');
                    maze.insert(above_coordinate.to_maze_key(), above_coordinate);
                }
            }
        }
    }

    print_maze(&maze, &max_x, &max_y);

    maze.values()
        .filter(|c| c.camera_output.unwrap() == 'O')
        .map(|c| c.x * c.y)
        .sum()
}

fn is_alignement(coordinate: &Coordinate, maze: &HashMap<String, Coordinate>) -> bool {
    vec![
        Coordinate::get_direction_maze_key(coordinate, &None),
        Coordinate::get_direction_maze_key(coordinate, &Some(&Direction::North)),
        Coordinate::get_direction_maze_key(coordinate, &Some(&Direction::East)),
        Coordinate::get_direction_maze_key(coordinate, &Some(&Direction::South)),
        Coordinate::get_direction_maze_key(coordinate, &Some(&Direction::West)),
    ]
    .iter()
    .map(|k| maze.get(k).unwrap().camera_output.unwrap())
    .all(|c| c == '#')
}

fn print_maze(maze: &HashMap<String, Coordinate>, max_x: &isize, max_y: &isize) -> () {
    let mut graphic_maze: Vec<Vec<String>> = vec![];
    for y in 0..max_y.clone() {
        let mut line: Vec<String> = vec![];
        for x in 0..max_x.clone() {
            line.push(
                maze.get(&Coordinate::static_to_maze_key(x, y))
                    .unwrap()
                    .camera_output
                    .unwrap()
                    .to_string(),
            );
        }
        graphic_maze.push(line)
    }

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

#[derive(Clone, Debug, PartialEq)]
enum Direction {
    North,
    East,
    South,
    West,
}

#[derive(Clone, Debug, PartialEq)]
struct Coordinate {
    x: isize,
    y: isize,
    camera_output: Option<char>,
}

impl Coordinate {
    fn new(x: isize, y: isize, camera_output: Option<char>) -> Coordinate {
        Coordinate {
            x: x,
            y: y,
            camera_output: camera_output,
        }
    }

    fn static_to_maze_key(x: isize, y: isize) -> String {
        x.to_string() + "," + &y.to_string()
    }

    fn get_direction_maze_key(coordinate: &Coordinate, direction: &Option<&Direction>) -> String {
        match direction {
            Some(Direction::North) => {
                Coordinate::static_to_maze_key(coordinate.x, coordinate.y - 1)
            }
            Some(Direction::East) => Coordinate::static_to_maze_key(coordinate.x + 1, coordinate.y),
            Some(Direction::South) => {
                Coordinate::static_to_maze_key(coordinate.x, coordinate.y + 1)
            }
            Some(Direction::West) => Coordinate::static_to_maze_key(coordinate.x - 1, coordinate.y),
            _ => Coordinate::static_to_maze_key(coordinate.x, coordinate.y),
        }
    }

    fn to_maze_key(&self) -> String {
        Coordinate::static_to_maze_key(self.x, self.y)
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
