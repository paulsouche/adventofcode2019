use std::collections::HashMap;
use std::{collections::VecDeque, fs::read_to_string, io};

fn main() -> io::Result<()> {
    let program: Vec<isize> = read_to_string("src/bin/day13-care-package/input.txt")?
        .trim()
        .split(',')
        .map(|line| line.parse::<isize>().unwrap())
        .collect();

    let mut blocks: Vec<Vec<isize>> = vec![];
    let mut block: Vec<isize> = vec![];
    let mut computer = IntcodeComputer::new();
    computer.reset();
    computer.push_instructions(program.clone());

    loop {
        if block.len() == 3 {
            blocks.push(block);
            block = vec![]
        }

        match computer.run() {
            ProgramState::Halted => break,
            ProgramState::NeedInput => panic!("No need for input ?"),
            ProgramState::Output(out) => block.push(out),
        }
    }

    println!(
        "{}",
        blocks
            .iter()
            .filter(|b| b[2] == 2)
            .collect::<Vec<&Vec<isize>>>()
            .len()
    );

    computer.reset();
    let mut game = program.clone();
    game[0] = 2;
    computer.push_instructions(game);

    let mut ball: Option<isize> = None;
    let mut joystick: Option<isize> = None;
    let mut score = 0;

    loop {
        match computer.run() {
            ProgramState::Halted => break,
            ProgramState::NeedInput => {
                let ball_x = ball.unwrap();
                let joystick_x = joystick.unwrap();
                if ball_x < joystick_x {
                    computer.push_input(-1);
                } else if ball_x > joystick_x {
                    computer.push_input(1);
                } else {
                    computer.push_input(0);
                }
            }
            ProgramState::Output(out) => {
                block.push(out);

                if block.len() == 3 {
                    let id = block.pop().unwrap();
                    let y = block.pop().unwrap();
                    let x = block.pop().unwrap();

                    if x == -1 && y == 0 {
                        score = id;
                    } else if id == 3 {
                        joystick = Some(x);
                    } else if id == 4 {
                        ball = Some(x);
                    }
                }
            }
        }
    }

    println!("{}", score);

    Ok(())
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
