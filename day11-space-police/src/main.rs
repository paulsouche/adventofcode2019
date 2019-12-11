use std::collections::HashMap;
use std::{collections::VecDeque, fs::read_to_string, io};

fn main() -> io::Result<()> {
    let program: Vec<isize> = read_to_string("input.txt")?
        .trim()
        .split(',')
        .map(|line| line.parse::<isize>().unwrap())
        .collect();

    let mut direction = Direction::Top;
    let mut coordinates: (isize, isize) = (0, 0);
    let mut panel: HashMap<(isize, isize), isize> = HashMap::new();
    let mut computer = IntcodeComputer::new();
    let mut has_painted = false;
    computer.reset();
    computer.push_input(get_color(&panel, &coordinates));
    computer.push_instructions(program.clone());

    loop {
        match computer.run() {
            ProgramState::Halted => break,
            ProgramState::NeedInput => computer.push_input(get_color(&panel, &coordinates)),
            ProgramState::Output(out) => {
                match out {
                    0 | 1 => (),
                    _ => panic!("Invalid output"),
                }
                if !has_painted {
                    panel.insert(coordinates, out);
                    has_painted = true;
                } else {
                    direction = get_new_direction(&direction, &out);
                    coordinates = get_new_coordinates(&direction, &coordinates);
                    has_painted = false;
                }
            }
        }
    }

    println!("{}", panel.keys().len());

    direction = Direction::Top;
    coordinates = (0, 0);
    panel = HashMap::new();
    let mut min_coordinates: (isize, isize) = (isize::max_value(), isize::max_value());
    let mut max_coordinates: (isize, isize) = (isize::min_value(), isize::min_value());
    computer.reset();
    computer.push_input(1);
    computer.push_instructions(program.clone());
    loop {
        match computer.run() {
            ProgramState::Halted => break,
            ProgramState::NeedInput => computer.push_input(get_color(&panel, &coordinates)),
            ProgramState::Output(out) => {
                match out {
                    0 | 1 => (),
                    _ => panic!("Invalid output"),
                }
                if !has_painted {
                    panel.insert(coordinates, out);
                    has_painted = true;
                } else {
                    direction = get_new_direction(&direction, &out);
                    coordinates = get_new_coordinates(&direction, &coordinates);

                    if min_coordinates.0 > coordinates.0 {
                        min_coordinates.0 = coordinates.0;
                    }

                    if min_coordinates.1 > coordinates.1 {
                        min_coordinates.1 = coordinates.1;
                    }

                    if max_coordinates.0 < coordinates.0 {
                        max_coordinates.0 = coordinates.0;
                    }

                    if max_coordinates.1 < coordinates.1 {
                        max_coordinates.1 = coordinates.1;
                    }
                    has_painted = false;
                }
            }
        }
    }

    let mut image = vec![];
    for y in min_coordinates.1..(max_coordinates.1 + 1) {
        let mut line = vec![];
        for x in (min_coordinates.0 + 1)..(max_coordinates.0) {
            line.push(match get_color(&panel, &(x, y)) {
                0 => " ",
                1 => "#",
                n => panic!("Unknown color {}", n),
            });
        }
        image.push(line.join(""));
    }

    image.reverse();
    println!("{}", image.join("\n"));

    Ok(())
}

fn get_color(panel: &HashMap<(isize, isize), isize>, coordinates: &(isize, isize)) -> isize {
    match panel.get(coordinates) {
        Some(color) => color.clone(),
        _ => 0,
    }
}

fn get_new_coordinates(direction: &Direction, coordinates: &(isize, isize)) -> (isize, isize) {
    match direction {
        Direction::Top => (coordinates.0, coordinates.1 + 1),
        Direction::Right => (coordinates.0 + 1, coordinates.1),
        Direction::Bottom => (coordinates.0, coordinates.1 - 1),
        Direction::Left => (coordinates.0 - 1, coordinates.1),
    }
}

fn get_new_direction(direction: &Direction, output: &isize) -> Direction {
    match direction {
        Direction::Top => match output {
            0 => Direction::Left,
            1 => Direction::Right,
            n => panic!("Unknown output {}", n),
        },
        Direction::Right => match output {
            0 => Direction::Top,
            1 => Direction::Bottom,
            n => panic!("Unknown output {}", n),
        },
        Direction::Bottom => match output {
            0 => Direction::Right,
            1 => Direction::Left,
            n => panic!("Unknown output {}", n),
        },
        Direction::Left => match output {
            0 => Direction::Bottom,
            1 => Direction::Top,
            n => panic!("Unknown output {}", n),
        },
    }
}

enum Direction {
    Top,
    Right,
    Bottom,
    Left,
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
