fn main() {
    let mut input = vec![
        1, 0, 0, 3, 1, 1, 2, 3, 1, 3, 4, 3, 1, 5, 0, 3, 2, 6, 1, 19, 1, 5, 19, 23, 1, 13, 23, 27,
        1, 6, 27, 31, 2, 31, 13, 35, 1, 9, 35, 39, 2, 39, 13, 43, 1, 43, 10, 47, 1, 47, 13, 51, 2,
        13, 51, 55, 1, 55, 9, 59, 1, 59, 5, 63, 1, 6, 63, 67, 1, 13, 67, 71, 2, 71, 10, 75, 1, 6,
        75, 79, 1, 79, 10, 83, 1, 5, 83, 87, 2, 10, 87, 91, 1, 6, 91, 95, 1, 9, 95, 99, 1, 99, 9,
        103, 2, 103, 10, 107, 1, 5, 107, 111, 1, 9, 111, 115, 2, 13, 115, 119, 1, 119, 10, 123, 1,
        123, 10, 127, 2, 127, 10, 131, 1, 5, 131, 135, 1, 10, 135, 139, 1, 139, 2, 143, 1, 6, 143,
        0, 99, 2, 14, 0, 0,
    ];
    input[1] = 12;
    input[2] = 2;
    let output = run_program(input);
    println!("{}", output[0]);

    let mut found = false;
    for noun in 0..99 {
        for verb in 0..99 {
            let mut input2 = vec![
                1, 0, 0, 3, 1, 1, 2, 3, 1, 3, 4, 3, 1, 5, 0, 3, 2, 6, 1, 19, 1, 5, 19, 23, 1, 13,
                23, 27, 1, 6, 27, 31, 2, 31, 13, 35, 1, 9, 35, 39, 2, 39, 13, 43, 1, 43, 10, 47, 1,
                47, 13, 51, 2, 13, 51, 55, 1, 55, 9, 59, 1, 59, 5, 63, 1, 6, 63, 67, 1, 13, 67, 71,
                2, 71, 10, 75, 1, 6, 75, 79, 1, 79, 10, 83, 1, 5, 83, 87, 2, 10, 87, 91, 1, 6, 91,
                95, 1, 9, 95, 99, 1, 99, 9, 103, 2, 103, 10, 107, 1, 5, 107, 111, 1, 9, 111, 115,
                2, 13, 115, 119, 1, 119, 10, 123, 1, 123, 10, 127, 2, 127, 10, 131, 1, 5, 131, 135,
                1, 10, 135, 139, 1, 139, 2, 143, 1, 6, 143, 0, 99, 2, 14, 0, 0,
            ];
            input2[1] = noun;
            input2[2] = verb;
            let output = run_program(input2);
            if output[0] == 19690720 {
                println!("{}", noun * 100 + verb);
                found = true;
            }

            if found {
                break;
            }
        }
        if found {
            break;
        }
    }
}

fn run_program(mut input: Vec<i32>) -> Vec<i32> {
    for i in (0..input.len()).step_by(4) {
        let operator = input[i];

        while input.len() <= i + 3 {
            input.push(0);
        }
        let pos1 = input[i + 1] as usize;
        let pos2 = input[i + 2] as usize;
        let pos3 = input[i + 3] as usize;

        match operator {
            99 => break,
            1 => input[pos3] = input[pos1] + input[pos2],
            2 => input[pos3] = input[pos1] * input[pos2],
            _ => {
                println!("WARNING unknown operator {}", operator);
                break;
            }
        }
    }
    input
}

#[cfg(test)]
mod tests {
    use super::run_program;

    #[test]
    fn it_should_know_how_run_program_1() {
        let input = vec![1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50];
        let output: Vec<String> = run_program(input).iter().map(|x| x.to_string()).collect();
        assert_eq!(output.join(","), "3500,9,10,70,2,3,11,0,99,30,40,50");
    }

    #[test]
    fn it_should_know_how_run_program_2() {
        let input = vec![1, 0, 0, 0, 99];
        let output: Vec<String> = run_program(input).iter().map(|x| x.to_string()).collect();
        assert_eq!(output.join(","), "2,0,0,0,99,0,0,0");
    }

    #[test]
    fn it_should_know_how_run_program_3() {
        let input = vec![2, 3, 0, 3, 99];
        let output: Vec<String> = run_program(input).iter().map(|x| x.to_string()).collect();
        assert_eq!(output.join(","), "2,3,0,6,99,0,0,0");
    }

    #[test]
    fn it_should_know_how_run_program_4() {
        let input = vec![2, 4, 4, 5, 99, 0];
        let output: Vec<String> = run_program(input).iter().map(|x| x.to_string()).collect();
        assert_eq!(output.join(","), "2,4,4,5,99,9801,0,0");
    }

    #[test]
    fn it_should_know_how_run_program_5() {
        let input = vec![1, 1, 1, 4, 99, 5, 6, 0, 99];
        let output: Vec<String> = run_program(input).iter().map(|x| x.to_string()).collect();
        assert_eq!(output.join(","), "30,1,1,4,2,5,6,0,99,0,0,0");
    }
}
