use std::{fs::read_to_string, io};

fn main() -> io::Result<()> {
    let input: Vec<i32> = read_to_string("input.txt")?
        .trim()
        .split('\n')
        .map(|line| line.parse::<i32>().unwrap())
        .collect();

    println!("{}", input.iter().map(|&x| required_fuel(x)).sum::<i32>());
    println!("{}", input.iter().map(|&x| module_fuel(x)).sum::<i32>());

    Ok(())
}

fn required_fuel(mass: i32) -> i32 {
    (mass / 3) - 2
}

fn module_fuel(mass: i32) -> i32 {
    let mut sum: i32 = 0;
    let mut calc_mass: i32 = mass;

    loop {
        calc_mass = required_fuel(calc_mass);
        if calc_mass <= 0 {
            break;
        }
        sum += calc_mass;
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::module_fuel;
    use super::required_fuel;

    #[test]
    fn it_should_know_how_much_fuel_for_mass() {
        assert_eq!(required_fuel(12), 2);
        assert_eq!(required_fuel(14), 2);
        assert_eq!(required_fuel(1969), 654);
        assert_eq!(required_fuel(100756), 33583);
    }

    #[test]
    fn it_should_know_how_much_fuel_for_module() {
        assert_eq!(module_fuel(14), 2);
        assert_eq!(module_fuel(1969), 966);
        assert_eq!(module_fuel(100756), 50346);
    }
}
