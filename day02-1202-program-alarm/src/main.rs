use std::{fs::read_to_string, io};

fn main() -> io::Result<()> {
    let input: Vec<usize> = read_to_string("input.txt")?
        .trim()
        .split(',')
        .map(|line| line.parse::<usize>().unwrap())
        .collect();

    println!("{}", get_program_output(&mut input.clone(), 12, 2));
    println!("{}", find_noun_and_verb(&input));
    Ok(())
}

fn find_noun_and_verb(input: &Vec<usize>) -> usize {
    for noun in 0..100 {
        for verb in 0..100 {
            if get_program_output(&mut input.clone(), noun, verb) == 19690720 {
                return noun * 100 + verb;
            }
        }
    }
    unreachable!("Did not find any code that ended with: 19690720")
}

fn get_program_output(input: &mut Vec<usize>, noun: usize, word: usize) -> usize {
    input[1] = noun;
    input[2] = word;
    run_program(input).remove(0)
}

fn run_program(input: &mut Vec<usize>) -> &mut Vec<usize> {
    for i in (0..input.len()).step_by(4) {
        let operator = input[i];

        // This could be better...
        while input.len() <= i + 3 {
            input.push(0);
        }
        let ind = input[i + 3];

        match operator {
            99 => break,
            1 => input[ind] = input[input[i + 1]] + input[input[i + 2]],
            2 => input[ind] = input[input[i + 1]] * input[input[i + 2]],
            n => panic!("reached unknown code: {}", n),
        }
    }
    input
}

#[cfg(test)]
mod tests {
    use super::run_program;

    #[test]
    fn it_should_know_how_run_program_1() {
        let input: Vec<usize> = vec![1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50];
        let output: Vec<String> = run_program(&mut input.clone())
            .iter()
            .map(|x| x.to_string())
            .collect();
        assert_eq!(output.join(","), "3500,9,10,70,2,3,11,0,99,30,40,50");
    }

    #[test]
    fn it_should_know_how_run_program_2() {
        let input: Vec<usize> = vec![1, 0, 0, 0, 99];
        let output: Vec<String> = run_program(&mut input.clone())
            .iter()
            .map(|x| x.to_string())
            .collect();
        assert_eq!(output.join(","), "2,0,0,0,99,0,0,0");
    }

    #[test]
    fn it_should_know_how_run_program_3() {
        let input: Vec<usize> = vec![2, 3, 0, 3, 99];
        let output: Vec<String> = run_program(&mut input.clone())
            .iter()
            .map(|x| x.to_string())
            .collect();
        assert_eq!(output.join(","), "2,3,0,6,99,0,0,0");
    }

    #[test]
    fn it_should_know_how_run_program_4() {
        let input: Vec<usize> = vec![2, 4, 4, 5, 99, 0];
        let output: Vec<String> = run_program(&mut input.clone())
            .iter()
            .map(|x| x.to_string())
            .collect();
        assert_eq!(output.join(","), "2,4,4,5,99,9801,0,0");
    }

    #[test]
    fn it_should_know_how_run_program_5() {
        let input: Vec<usize> = vec![1, 1, 1, 4, 99, 5, 6, 0, 99];
        let output: Vec<String> = run_program(&mut input.clone())
            .iter()
            .map(|x| x.to_string())
            .collect();
        assert_eq!(output.join(","), "30,1,1,4,2,5,6,0,99,0,0,0");
    }
}
