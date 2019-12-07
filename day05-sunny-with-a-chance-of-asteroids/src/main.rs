use std::{fs::read_to_string, io};

fn main() -> io::Result<()> {
    let program: Vec<i32> = read_to_string("input.txt")?
        .trim()
        .split(',')
        .map(|line| line.parse::<i32>().unwrap())
        .collect();

    println!("{}", run_program(&mut program.clone(), &mut vec![1]));
    println!("{}", run_program(&mut program.clone(), &mut vec![5]));
    Ok(())
}

fn get_program_value(program: &mut Vec<i32>, mode: &i32, ind: usize) -> i32 {
    match mode {
        0 => program[ind],
        1 => ind as i32,
        n => panic!("Unknown mode {}", n),
    }
}

fn run_program(program: &mut Vec<i32>, inputs: &mut Vec<i32>) -> i32 {
    let mut i: usize = 0;
    let mut output: i32 = 0;
    loop {
        let n = program[i];
        let operator = n - (n / 100) * 100;
        let mode1 = (n - (n / 1000) * 1000) / 100;
        let mode2 = (n - (n / 10000) * 10000) / 1000;

        match operator {
            99 => break,
            1 => {
                let value_param1 = get_program_value(program, &mode1, program[i + 1] as usize);
                let value_param2 = get_program_value(program, &mode2, program[i + 2] as usize);
                let ind = program[i + 3] as usize;

                program[ind] = value_param1 + value_param2;
                i += 4;
            }
            2 => {
                let value_param1 = get_program_value(program, &mode1, program[i + 1] as usize);
                let value_param2 = get_program_value(program, &mode2, program[i + 2] as usize);
                let ind = program[i + 3] as usize;

                program[ind] = value_param1 * value_param2;
                i += 4;
            }
            3 => {
                let ind = program[i + 1] as usize;

                if inputs.len() == 0 {
                    panic!("No more inputs !");
                }

                program[ind] = inputs.remove(0);
                i += 2;
            }
            4 => {
                let ind = program[i + 1] as usize;

                output = program[ind];
                i += 2;
            }
            5 => {
                let value_param1 = get_program_value(program, &mode1, program[i + 1] as usize);
                if value_param1 != 0 {
                    i = get_program_value(program, &mode2, program[i + 2] as usize) as usize;
                } else {
                    i += 3;
                }
            }
            6 => {
                let value_param1 = get_program_value(program, &mode1, program[i + 1] as usize);
                if value_param1 == 0 {
                    i = get_program_value(program, &mode2, program[i + 2] as usize) as usize;
                } else {
                    i += 3;
                }
            }
            7 => {
                let value_param1 = get_program_value(program, &mode1, program[i + 1] as usize);
                let value_param2 = get_program_value(program, &mode2, program[i + 2] as usize);
                let ind = program[i + 3] as usize;

                if value_param1 < value_param2 {
                    program[ind] = 1;
                } else {
                    program[ind] = 0;
                }
                i += 4;
            }
            8 => {
                let value_param1 = get_program_value(program, &mode1, program[i + 1] as usize);
                let value_param2 = get_program_value(program, &mode2, program[i + 2] as usize);
                let ind = program[i + 3] as usize;

                if value_param1 == value_param2 {
                    program[ind] = 1;
                } else {
                    program[ind] = 0;
                }
                i += 4;
            }
            n => panic!("Unknown operator parameter {}", n),
        }
    }
    output
}

#[cfg(test)]
mod tests {
    use super::run_program;

    #[test]
    fn it_should_output_something() {
        let program: Vec<i32> = vec![3, 0, 4, 0, 99];
        assert_eq!(run_program(&mut program.clone(), &mut vec![1]), 1);
    }

    #[test]
    fn it_should_handle_parameters() {
        let program: Vec<i32> = vec![1002, 4, 3, 4, 33];
        assert_eq!(run_program(&mut program.clone(), &mut vec![1]), 0);
    }

    #[test]
    fn it_should_handle_negative_integers() {
        let program: Vec<i32> = vec![1101, 100, -1, 4, 0];
        assert_eq!(run_program(&mut program.clone(), &mut vec![1]), 0);
    }

    #[test]
    fn it_should_know_if_input_equals_8_in_position_mode() {
        let program: Vec<i32> = vec![3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8];
        assert_eq!(run_program(&mut program.clone(), &mut vec![8]), 1);
        assert_eq!(run_program(&mut program.clone(), &mut vec![9]), 0);
    }

    #[test]
    fn it_should_know_if_input_is_less_than_8_in_position_mode() {
        let program: Vec<i32> = vec![3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8];
        assert_eq!(run_program(&mut program.clone(), &mut vec![7]), 1);
        assert_eq!(run_program(&mut program.clone(), &mut vec![8]), 0);
    }

    #[test]
    fn it_should_know_if_input_equals_8_in_immediate_mode() {
        let program: Vec<i32> = vec![3, 3, 1108, -1, 8, 3, 4, 3, 99];
        assert_eq!(run_program(&mut program.clone(), &mut vec![8]), 1);
        assert_eq!(run_program(&mut program.clone(), &mut vec![9]), 0);
    }

    #[test]
    fn it_should_know_if_input_is_less_than_8_in_immediate_mode() {
        let program: Vec<i32> = vec![3, 3, 1107, -1, 8, 3, 4, 3, 99];
        assert_eq!(run_program(&mut program.clone(), &mut vec![7]), 1);
        assert_eq!(run_program(&mut program.clone(), &mut vec![8]), 0);
    }

    #[test]
    fn it_should_know_if_input_is_zero_in_position_mode() {
        let program: Vec<i32> = vec![3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9];
        assert_eq!(run_program(&mut program.clone(), &mut vec![0]), 0);
        assert_eq!(run_program(&mut program.clone(), &mut vec![1]), 1);
    }

    #[test]
    fn it_should_know_if_input_is_zero_in_immediate_mode() {
        let program: Vec<i32> = vec![3, 3, 1105, -1, 9, 1101, 0, 0, 12, 4, 12, 99, 1];
        assert_eq!(run_program(&mut program.clone(), &mut vec![0]), 0);
        assert_eq!(run_program(&mut program.clone(), &mut vec![1]), 1);
    }

    #[test]
    fn it_should_know_if_input_is_less_than_equals_or_greater_than_8() {
        let program: Vec<i32> = vec![
            3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31, 1106, 0, 36, 98, 0,
            0, 1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104, 999, 1105, 1, 46, 1101, 1000, 1, 20, 4,
            20, 1105, 1, 46, 98, 99,
        ];
        // assert_eq!(run_program(&mut program.clone(), 7), 999);
        assert_eq!(run_program(&mut program.clone(), &mut vec![8]), 1000);
        assert_eq!(run_program(&mut program.clone(), &mut vec![9]), 1001);
    }
}
