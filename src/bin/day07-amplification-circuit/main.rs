use permutohedron;
use std::collections::HashMap;
use std::{collections::VecDeque, fs::read_to_string, io};

fn main() -> io::Result<()> {
    let program: Vec<isize> = read_to_string("src/bin/day07-amplification-circuit/input.txt")?
        .trim()
        .split(',')
        .map(|line| line.parse::<isize>().unwrap())
        .collect();

    println!(
        "{}",
        find_higher_output(&mut program.clone(), &mut [0, 1, 2, 3, 4])
    );
    println!(
        "{}",
        find_higher_output(&mut program.clone(), &mut [5, 6, 7, 8, 9])
    );
    Ok(())
}

fn find_higher_output(program: &mut Vec<isize>, phases: &mut [isize; 5]) -> isize {
    let mut permutations = Vec::new();

    permutohedron::heap_recursive(phases, |x| permutations.push(x.to_vec()));

    let mut output_max = isize::min_value();

    let mut amps = [
        IntcodeComputer::new(),
        IntcodeComputer::new(),
        IntcodeComputer::new(),
        IntcodeComputer::new(),
        IntcodeComputer::new(),
    ];

    for permutation in permutations {
        for x in 0..5 {
            amps[x].reset();
            amps[x].push_instructions(program.clone());
            amps[x].push_input(permutation[x]);
        }

        let mut index = 0;
        let mut prev_output = 0;

        'outer: loop {
            loop {
                match amps[index].run() {
                    ProgramState::Halted => {
                        if prev_output > output_max {
                            output_max = prev_output;
                        }
                        break 'outer;
                    }
                    ProgramState::NeedInput => amps[index].push_input(prev_output),
                    ProgramState::Output(out) => {
                        prev_output = out;
                        break;
                    }
                }
            }

            index = if index == 4 { 0 } else { index + 1 };
        }
    }

    output_max
}

enum ProgramState {
    Halted,
    NeedInput,
    Output(isize),
}

struct IntcodeComputer {
    instructions: HashMap<usize, isize>,
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
        self.inputs.clear();
    }

    fn push_instructions(&mut self, program: Vec<isize>) {
        for ind in 0..program.len() {
            self.instructions.insert(ind, program[ind]);
        }
    }

    fn push_input(&mut self, input: isize) {
        self.inputs.push_back(input);
    }

    fn run(&mut self) -> ProgramState {
        if self.instructions[&self.pointer] == 99 {
            return ProgramState::Halted;
        }

        let mut output: Option<isize> = None;

        loop {
            let value = self.instructions[&self.pointer];
            let operator = value - (value / 100) * 100;
            let mode1 = (value - (value / 1000) * 1000) / 100;
            let mode2 = (value - (value / 10000) * 10000) / 1000;

            if let Some(out) = output {
                return ProgramState::Output(out);
            }

            match operator {
                99 => break,
                1 => {
                    let value_param1 = self.get_instruction_value(
                        &mode1,
                        self.instructions[&(self.pointer + 1)] as usize,
                    );
                    let value_param2 = self.get_instruction_value(
                        &mode2,
                        self.instructions[&(self.pointer + 2)] as usize,
                    );
                    let ind = self.instructions[&(self.pointer + 3)] as usize;

                    self.instructions.insert(ind, value_param1 + value_param2);
                    self.pointer += 4;
                }
                2 => {
                    let value_param1 = self.get_instruction_value(
                        &mode1,
                        self.instructions[&(self.pointer + 1)] as usize,
                    );
                    let value_param2 = self.get_instruction_value(
                        &mode2,
                        self.instructions[&(self.pointer + 2)] as usize,
                    );
                    let ind = self.instructions[&(self.pointer + 3)] as usize;

                    self.instructions.insert(ind, value_param1 * value_param2);
                    self.pointer += 4;
                }
                3 => {
                    let ind = self.instructions[&(self.pointer + 1)] as usize;
                    self.instructions.insert(ind, match self.inputs.pop_front() {
                        Some(x) => x,
                        None => return ProgramState::NeedInput,
                    });
                    self.pointer += 2;
                }
                4 => {
                    let ind = self.instructions[&(self.pointer + 1)] as usize;

                    output = Some(self.instructions[&ind]);
                    self.pointer += 2;
                }
                5 => {
                    let value_param1 = self.get_instruction_value(
                        &mode1,
                        self.instructions[&(self.pointer + 1)] as usize,
                    );
                    if value_param1 != 0 {
                        self.pointer = self.get_instruction_value(
                            &mode2,
                            self.instructions[&(self.pointer + 2)] as usize,
                        ) as usize;
                    } else {
                        self.pointer += 3;
                    }
                }
                6 => {
                    let value_param1 = self.get_instruction_value(
                        &mode1,
                        self.instructions[&(self.pointer + 1)] as usize,
                    );
                    if value_param1 == 0 {
                        self.pointer = self.get_instruction_value(
                            &mode2,
                            self.instructions[&(self.pointer + 2)] as usize,
                        ) as usize;
                    } else {
                        self.pointer += 3;
                    }
                }
                7 => {
                    let value_param1 = self.get_instruction_value(
                        &mode1,
                        self.instructions[&(self.pointer + 1)] as usize,
                    );
                    let value_param2 = self.get_instruction_value(
                        &mode2,
                        self.instructions[&(self.pointer + 2)] as usize,
                    );
                    let ind = self.instructions[&(self.pointer + 3)] as usize;

                    if value_param1 < value_param2 {
                        self.instructions.insert(ind, 1);
                    } else {
                        self.instructions.insert(ind, 0);
                    }
                    self.pointer += 4;
                }
                8 => {
                    let value_param1 = self.get_instruction_value(
                        &mode1,
                        self.instructions[&(self.pointer + 1)] as usize,
                    );
                    let value_param2 = self.get_instruction_value(
                        &mode2,
                        self.instructions[&(self.pointer + 2)] as usize,
                    );
                    let ind = self.instructions[&(self.pointer + 3)] as usize;

                    if value_param1 == value_param2 {
                        self.instructions.insert(ind, 1);
                    } else {
                        self.instructions.insert(ind, 0);
                    }
                    self.pointer += 4;
                }
                9 => {
                    let value_param1 = self.get_instruction_value(
                        &mode1,
                        self.instructions[&(self.pointer + 1)] as usize,
                    );
                    self.relative_base += value_param1;
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

    fn get_instruction_value(&self, mode: &isize, ind: usize) -> isize {
        match mode {
            0 => self.instructions[&ind],
            1 => ind as isize,
            2 => self.instructions[&(ind + self.relative_base as usize)],
            n => panic!("Unknown mode {}", n),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::find_higher_output;

    #[test]
    fn it_should_find_higher_output_1() {
        let program: Vec<isize> = vec![
            3, 15, 3, 16, 1002, 16, 10, 16, 1, 16, 15, 15, 4, 15, 99, 0, 0,
        ];
        assert_eq!(
            find_higher_output(&mut program.clone(), &mut [0, 1, 2, 3, 4]),
            43210
        );
    }

    #[test]
    fn it_should_find_higher_output_2() {
        let program: Vec<isize> = vec![
            3, 23, 3, 24, 1002, 24, 10, 24, 1002, 23, -1, 23, 101, 5, 23, 23, 1, 24, 23, 23, 4, 23,
            99, 0, 0,
        ];
        assert_eq!(
            find_higher_output(&mut program.clone(), &mut [0, 1, 2, 3, 4]),
            54321
        );
    }

    #[test]
    fn it_should_find_higher_output_3() {
        let program: Vec<isize> = vec![
            3, 31, 3, 32, 1002, 32, 10, 32, 1001, 31, -2, 31, 1007, 31, 0, 33, 1002, 33, 7, 33, 1,
            33, 31, 31, 1, 32, 31, 31, 4, 31, 99, 0, 0, 0,
        ];
        assert_eq!(
            find_higher_output(&mut program.clone(), &mut [0, 1, 2, 3, 4]),
            65210
        );
    }

    #[test]
    fn it_should_find_higher_output_in_feedback_loop_1() {
        let program: Vec<isize> = vec![
            3, 26, 1001, 26, -4, 26, 3, 27, 1002, 27, 2, 27, 1, 27, 26, 27, 4, 27, 1001, 28, -1,
            28, 1005, 28, 6, 99, 0, 0, 5,
        ];
        assert_eq!(
            find_higher_output(&mut program.clone(), &mut [5, 6, 7, 8, 9]),
            139629729
        );
    }

    #[test]
    fn it_should_find_higher_output_in_feedback_loop_2() {
        let program: Vec<isize> = vec![
            3, 52, 1001, 52, -5, 52, 3, 53, 1, 52, 56, 54, 1007, 54, 5, 55, 1005, 55, 26, 1001, 54,
            -5, 54, 1105, 1, 12, 1, 53, 54, 53, 1008, 54, 0, 55, 1001, 55, 1, 55, 2, 53, 55, 53, 4,
            53, 1001, 56, -1, 56, 1005, 56, 6, 99, 0, 0, 0, 0, 10,
        ];
        assert_eq!(
            find_higher_output(&mut program.clone(), &mut [5, 6, 7, 8, 9]),
            18216
        );
    }
}
