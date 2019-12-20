use regex::Regex;
use std::{fs::read_to_string, io};

fn main() -> io::Result<()> {
    let file = read_to_string("src/bin/day12-the-n-body-problem/input.txt")?;
    let input: Vec<Planet> = file
        .trim()
        .split('\n')
        .map(|l| Planet::new(parse_input_line(l), None))
        .collect();

    println!("{}", part1(&mut input.clone(), &1000));
    println!("{}", part2(&mut input.clone()));

    Ok(())
}

fn part1(planets: &mut Vec<Planet>, steps: &usize) -> isize {
    run_steps(planets, steps);
    let mut energy = 0;
    for planet in planets.iter() {
        energy += (planet.position.0.abs() + planet.position.1.abs() + planet.position.2.abs())
            * (planet.velocity.0.abs() + planet.velocity.1.abs() + planet.velocity.2.abs())
    }

    energy
}

fn part2(planets: &mut Vec<Planet>) -> isize {
    let origin: Vec<Planet> = planets.clone();
    let mut steps = 0;
    let mut axis_steps = vec![0, 0, 0];

    loop {
        run_steps(planets, &1);
        steps += 1;

        for i in 0..axis_steps.len() {
            if axis_steps[i] != 0 {
                continue;
            }

            if same_on_axis(&i, &origin, planets) {
                axis_steps[i] = steps;
            }
        }

        if axis_steps.iter().all(|s| s != &0) {
            break;
        }
    }

    least_common_multiple(axis_steps)
}

fn same_on_axis(axis: &usize, origin: &Vec<Planet>, planets: &Vec<Planet>) -> bool {
    match axis {
        0 => {
            for i in 0..origin.len() {
                if (origin[i].position.0 != planets[i].position.0)
                    || (origin[i].velocity.0 != planets[i].velocity.0)
                {
                    return false;
                }
            }
            true
        }
        1 => {
            for i in 0..origin.len() {
                if (origin[i].position.1 != planets[i].position.1)
                    || (origin[i].velocity.1 != planets[i].velocity.1)
                {
                    return false;
                }
            }
            true
        }
        2 => {
            for i in 0..origin.len() {
                if (origin[i].position.2 != planets[i].position.2)
                    || (origin[i].velocity.2 != planets[i].velocity.2)
                {
                    return false;
                }
            }
            true
        }
        n => panic!("Unknown axis {}", n),
    }
}

fn least_common_multiple(numbers: Vec<isize>) -> isize {
    if numbers.len() == 0 {
        return 0;
    }
    let mut r_val = numbers[0];
    for i in 0..numbers.len() {
        r_val = (r_val * numbers[i]) / greatest_common_divisor(r_val, numbers[i]);
    }
    r_val
}

fn greatest_common_divisor(a: isize, b: isize) -> isize {
    if b == 0 {
        return a;
    }
    greatest_common_divisor(b, a % b)
}

fn parse_input_line(line: &str) -> (isize, isize, isize) {
    let re = Regex::new(r"<x=(-?\d+),\sy=(-?\d+),\sz=(-?\d+)>").unwrap();
    let captures = re.captures_iter(line).next().unwrap();
    (
        captures[1].parse::<isize>().unwrap(),
        captures[2].parse::<isize>().unwrap(),
        captures[3].parse::<isize>().unwrap(),
    )
}

fn run_steps(planets: &mut Vec<Planet>, steps: &usize) -> () {
    for _ in 0..steps.clone() {
        let planets_copy = planets.clone();
        for planet in planets.iter_mut() {
            let other_planets: Vec<&Planet> =
                planets_copy.iter().filter(|plt| !planet.eq(plt)).collect();
            planet.update_velocity(&other_planets).update_position();
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
struct Planet {
    position: (isize, isize, isize),
    velocity: (isize, isize, isize),
    other_planets: Vec<Planet>,
}

impl Planet {
    fn new(pos: (isize, isize, isize), vel: Option<(isize, isize, isize)>) -> Self {
        Planet {
            position: pos,
            velocity: match vel {
                Some(v) => v,
                _ => (0, 0, 0),
            },
            other_planets: vec![],
        }
    }

    fn update_velocity(&mut self, other_planets: &Vec<&Planet>) -> &mut Self {
        let mut gravity = (0, 0, 0);
        for planet in other_planets.iter() {
            if planet.position.0 > self.position.0 {
                gravity.0 += 1;
            } else if planet.position.0 < self.position.0 {
                gravity.0 -= 1;
            }

            if planet.position.1 > self.position.1 {
                gravity.1 += 1;
            } else if planet.position.1 < self.position.1 {
                gravity.1 -= 1;
            }

            if planet.position.2 > self.position.2 {
                gravity.2 += 1;
            } else if planet.position.2 < self.position.2 {
                gravity.2 -= 1;
            }
        }

        self.velocity.0 += gravity.0;
        self.velocity.1 += gravity.1;
        self.velocity.2 += gravity.2;

        self
    }

    fn update_position(&mut self) -> &mut Self {
        self.position.0 += self.velocity.0;
        self.position.1 += self.velocity.1;
        self.position.2 += self.velocity.2;
        self
    }
}

#[cfg(test)]
mod tests {
    use super::parse_input_line;
    use super::part1;
    use super::part2;
    use super::run_steps;
    use super::Planet;

    #[test]
    fn it_should_parse_input_lines() {
        assert_eq!(parse_input_line(&"<x=-1, y=0, z=2>"), (-1, 0, 2));
        assert_eq!(parse_input_line(&"<x=2, y=-10, z=-7>"), (2, -10, -7));
        assert_eq!(parse_input_line(&"<x=4, y=-8, z=8>"), (4, -8, 8));
        assert_eq!(parse_input_line(&"<x=3, y=5, z=-1>"), (3, 5, -1));
    }

    #[test]
    fn it_should_update_velocity_and_position() {
        let mut planet = Planet::new((-1, 0, 2), None);

        let planet2 = Planet::new((2, -10, -7), None);
        let planet3 = Planet::new((4, -8, 8), None);
        let planet4 = Planet::new((3, 5, -1), None);

        let other_planets: Vec<&Planet> = vec![&planet2, &planet3, &planet4];

        planet.update_velocity(&other_planets).update_position();

        assert_eq!(planet, Planet::new((2, -1, 1), Some((3, -1, -1))));
    }

    #[test]
    fn it_should_run_steps_1() {
        let mut planets = vec![
            Planet::new((-1, 0, 2), None),
            Planet::new((2, -10, -7), None),
            Planet::new((4, -8, 8), None),
            Planet::new((3, 5, -1), None),
        ];

        run_steps(&mut planets, &1);

        assert_eq!(
            planets,
            vec![
                Planet::new((2, -1, 1), Some((3, -1, -1))),
                Planet::new((3, -7, -4), Some((1, 3, 3))),
                Planet::new((1, -7, 5), Some((-3, 1, -3))),
                Planet::new((2, 2, 0), Some((-1, -3, 1))),
            ]
        );
    }

    #[test]
    fn it_should_run_steps_2() {
        let mut planets = vec![
            Planet::new((-1, 0, 2), None),
            Planet::new((2, -10, -7), None),
            Planet::new((4, -8, 8), None),
            Planet::new((3, 5, -1), None),
        ];

        run_steps(&mut planets, &2);

        assert_eq!(
            planets,
            vec![
                Planet::new((5, -3, -1), Some((3, -2, -2))),
                Planet::new((1, -2, 2), Some((-2, 5, 6))),
                Planet::new((1, -4, -1), Some((0, 3, -6))),
                Planet::new((1, -4, 2), Some((-1, -6, 2))),
            ]
        );
    }

    #[test]
    fn it_should_run_steps_10() {
        let mut planets = vec![
            Planet::new((-1, 0, 2), None),
            Planet::new((2, -10, -7), None),
            Planet::new((4, -8, 8), None),
            Planet::new((3, 5, -1), None),
        ];

        run_steps(&mut planets, &10);

        assert_eq!(
            planets,
            vec![
                Planet::new((2, 1, -3), Some((-3, -2, 1))),
                Planet::new((1, -8, 0), Some((-1, 1, 3))),
                Planet::new((3, -6, 1), Some((3, 2, -3))),
                Planet::new((2, 0, 4), Some((1, -1, -1))),
            ]
        );
    }

    #[test]
    fn it_should_compute_energy_1() {
        let mut planets = vec![
            Planet::new((-1, 0, 2), None),
            Planet::new((2, -10, -7), None),
            Planet::new((4, -8, 8), None),
            Planet::new((3, 5, -1), None),
        ];

        assert_eq!(part1(&mut planets, &10), 179);
    }

    #[test]
    fn it_should_compute_energy_2() {
        let mut planets = vec![
            Planet::new((-8, -10, 0), None),
            Planet::new((5, 5, 10), None),
            Planet::new((2, -7, 3), None),
            Planet::new((9, -8, -3), None),
        ];

        assert_eq!(part1(&mut planets, &100), 1940);
    }

    #[test]
    fn it_should_compute_the_number_of_steps_to_initial_state_1() {
        let mut planets = vec![
            Planet::new((-1, 0, 2), None),
            Planet::new((2, -10, -7), None),
            Planet::new((4, -8, 8), None),
            Planet::new((3, 5, -1), None),
        ];

        assert_eq!(part2(&mut planets), 2772);
    }

    #[test]
    fn it_should_compute_the_number_of_steps_to_initial_state_2() {
        let mut planets = vec![
            Planet::new((-8, -10, 0), None),
            Planet::new((5, 5, 10), None),
            Planet::new((2, -7, 3), None),
            Planet::new((9, -8, -3), None),
        ];

        assert_eq!(part2(&mut planets), 4686774924);
    }
}
