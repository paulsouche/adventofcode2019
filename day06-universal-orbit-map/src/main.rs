use std::collections::HashMap;
use std::{fs::read_to_string, io};

fn main() -> io::Result<()> {
    let input = read_to_string("input.txt")?;
    let planets: Vec<&str> = input.trim().split('\n').collect();

    println!("{}", get_total_of_orbits(&planets));
    println!("{}", get_shorter_path(&planets));
    Ok(())
}

fn walk_orbits(universe: &HashMap<&str, &str>, satellite: &str) -> i32 {
    match universe.get(satellite) {
        Some(center) => 1 + walk_orbits(&universe, &center),
        _ => 0,
    }
}

fn get_total_of_orbits(planets: &Vec<&str>) -> i32 {
    let mut universe = HashMap::new();

    for str_orbit in planets {
        let orbit: Vec<&str> = str_orbit.split(')').collect();
        let center = orbit[0];
        let satellite = orbit[1];
        universe.insert(satellite, center);
    }

    let mut n = 0;
    for key in universe.keys() {
        n += walk_orbits(&universe, &key);
    }
    n
}

fn get_shorter_path(planets: &Vec<&str>) -> usize {
    let mut universe = HashMap::new();

    for str_orbit in planets {
        let orbit: Vec<&str> = str_orbit.split(')').collect();
        let center = orbit[0];
        let satellite = orbit[1];
        universe.insert(satellite, center);
    }

    let mut planet_paths = Vec::new();
    for planet in vec!["YOU", "SAN"] {
        let mut path = Vec::new();
        let mut satellite = planet;
        loop {
            match universe.get(satellite) {
                Some(center) => {
                    path.push(center);
                    satellite = center;
                }
                _ => break,
            }
        }
        planet_paths.push(path);
    }

    let you_path = &planet_paths[0];
    let san_path = &planet_paths[1];

    let mut n = 0;
    for i in 0..you_path.len() {
        let path = &you_path[i];
        match san_path.iter().position(|x| x == path) {
            Some(x) => {
                n = x + i;
                break;
            }
            None => continue,
        }
    }
    n
}

#[cfg(test)]
mod tests {
    use super::get_shorter_path;
    use super::get_total_of_orbits;

    #[test]
    fn it_should_compute_the_number_of_orbits() {
        let planets: Vec<&str> = vec![
            "COM)B", "B)C", "C)D", "D)E", "E)F", "B)G", "G)H", "D)I", "E)J", "J)K", "K)L",
        ];
        assert_eq!(get_total_of_orbits(&planets), 42);
    }

    #[test]
    fn it_should_compute_the_shorter_path_between_planets() {
        let planets: Vec<&str> = vec![
            "COM)B", "B)C", "C)D", "D)E", "E)F", "B)G", "G)H", "D)I", "E)J", "J)K", "K)L", "K)YOU",
            "I)SAN",
        ];
        assert_eq!(get_shorter_path(&planets), 4);
    }
}
