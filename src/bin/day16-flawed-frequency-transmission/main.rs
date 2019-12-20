use std::{fs::read_to_string, io};

fn main() -> io::Result<()> {
    let file = read_to_string("src/bin/day16-flawed-frequency-transmission/input.txt")?;
    let input = file.trim();

    println!("{}", part1(&input, 100));
    println!("{}", part2(&input, 100, 10000));

    Ok(())
}

fn part1(input: &str, steps: usize) -> String {
    let mut phase: Vec<u8> = input
        .chars()
        .map(|c| c.to_string().parse::<u8>().unwrap())
        .collect();

    for _ in 0..steps {
        do_step(&mut phase);
    }

    phase
        .iter()
        .take(8)
        .map(|&i| (i + b'0') as char)
        .collect::<String>()
}

fn part2(input: &str, steps: usize, repeat: usize) -> String {
    let offset: usize = input[0..7].parse().unwrap();
    let mut phase: Vec<u8> = input
        .chars()
        .cycle()
        .skip(offset % input.len())
        .take(input.len() * repeat - offset)
        .map(|c| c.to_string().parse::<u8>().unwrap())
        .collect();

    phase.reverse();

    for _ in 0..steps {
        let mut acc: u32 = 0;
        for i in 0..phase.len() {
            acc += phase[i] as u32;
            phase[i] = (acc % 10) as u8;
        }
    }

    phase
        .iter()
        .rev()
        .take(8)
        .map(|&i| (i + b'0') as char)
        .collect::<String>()
}

fn do_step(phase: &mut Vec<u8>) -> () {
    for i in 1..phase.len() {
        let mut index = i - 1;
        let mut out: [isize; 2] = [0, 0];
        while index < phase.len() {
            for o in 0..2 {
                for _ in 0..i {
                    if index >= phase.len() {
                        break;
                    }
                    out[o] += phase[index] as isize;
                    index += 1;
                }
                index += i;
            }
        }
        phase[i - 1] = ((out[0] - out[1]).abs() % 10) as u8;
    }
}

#[cfg(test)]
mod tests {
    use super::part1;
    use super::part2;

    #[test]
    fn it_should_output_after_n_steps() {
        assert_eq!(part1(&"12345678", 1), "48226158");
        assert_eq!(part1(&"12345678", 2), "34040438");
        assert_eq!(part1(&"12345678", 3), "03415518");
        assert_eq!(part1(&"12345678", 4), "01029498");
    }

    #[test]
    fn it_should_output_after_100_steps() {
        assert_eq!(part1(&"80871224585914546619083218645595", 100), "24176176");
        assert_eq!(part1(&"19617804207202209144916044189917", 100), "73745418");
        assert_eq!(part1(&"69317163492948606335995924319873", 100), "52432133");
    }

    #[test]
    fn it_should_output_after_100_steps_on_a_repeated_10000_times() {
        assert_eq!(
            part2(&"03036732577212944063491565474664", 100, 10000),
            "84462026"
        );
        assert_eq!(
            part2(&"02935109699940807407585447034323", 100, 10000),
            "78725270"
        );
        assert_eq!(
            part2(&"03081770884921959731165446850517", 100, 10000),
            "53553731"
        );
    }
}
