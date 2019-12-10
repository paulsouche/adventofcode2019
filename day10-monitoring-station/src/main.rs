use std::cmp::Ordering;
use std::{fs::read_to_string, io};

fn main() -> io::Result<()> {
    let file = read_to_string("input.txt")?;
    let input: Vec<&str> = file.trim().split('\n').collect();

    let tuple = find_monitoring_station_asteroid(&input, Some(200));

    println!("{}", tuple.0);
    println!("{}", tuple.1);

    Ok(())
}

fn find_monitoring_station_asteroid(
    universe: &Vec<&str>,
    nth_asteroid_index: Option<usize>,
) -> (usize, f64) {
    let mut asteroids: Vec<Asteroid> = vec![];
    for y in 0..universe.len() {
        for x in 0..universe[y].len() {
            if universe[y].chars().nth(x).unwrap() == '#' {
                asteroids.push(Asteroid::new(x as f64, y as f64))
            }
        }
    }

    let nth_asteroid_index = match nth_asteroid_index {
        Some(number) => number,
        _ => usize::max_value(),
    };
    let mut max_visible_asteroids = usize::min_value();
    let mut nth_asteroid = Asteroid::new(0.0, 0.0);
    for asteroid1 in asteroids.iter() {
        let mut vertices: Vec<Vertice> = vec![];
        for asteroid2 in asteroids.iter() {
            if asteroid1 == asteroid2 {
                continue;
            }

            let mut found = false;
            let mut vertice = Vertice::new(asteroid1, asteroid2);
            for previous_vertice in vertices.iter_mut() {
                if previous_vertice == &mut vertice {
                    found = true;
                    previous_vertice
                        .add_asteroid(Asteroid::new(asteroid2.x, asteroid2.y), asteroid1);
                }
            }

            if !found {
                let mut vertice = Vertice::new(asteroid1, asteroid2);
                vertice.add_asteroid(Asteroid::new(asteroid2.x, asteroid2.y), asteroid1);
                vertices.push(vertice);
            }
        }

        if vertices.len() > max_visible_asteroids {
            max_visible_asteroids = vertices.len();

            if max_visible_asteroids >= nth_asteroid_index {
                vertices.sort_by(|a, b| a.partial_cmp(b).unwrap());

                let mut destroyed_asteroids: Vec<Asteroid> = vec![];
                while destroyed_asteroids.len() < nth_asteroid_index {
                    for vertice in vertices.iter_mut() {
                        let vaporised_asteroid = vertice.destroy_asteroid().unwrap();
                        destroyed_asteroids.push(vaporised_asteroid);
                        if destroyed_asteroids.len() >= nth_asteroid_index {
                            break;
                        }
                    }
                }
                nth_asteroid = match destroyed_asteroids.pop() {
                    Some(a) => a,
                    _ => panic!("Asteroid not found !"),
                };
            }
        }
    }

    (
        max_visible_asteroids,
        nth_asteroid.x * 100.0 + nth_asteroid.y,
    )
}

#[derive(Debug)]
struct Asteroid {
    x: f64,
    y: f64,
}

impl Asteroid {
    fn new(x: f64, y: f64) -> Self {
        Asteroid { x: x, y: y }
    }

    fn partial_cmp(&self, other: &Self, origin: &Asteroid) -> Option<Ordering> {
        if self.manhattan_distance(origin) > other.manhattan_distance(origin) {
            return Some(Ordering::Less);
        }
        Some(Ordering::Greater)
    }

    fn manhattan_distance(&self, origin: &Asteroid) -> f64 {
        (origin.x - self.x).abs() + (origin.y - self.y).abs()
    }
}

impl PartialEq for Asteroid {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

#[derive(Debug)]
struct Vertice {
    a: f64,
    dx: bool,
    dy: bool,
    asteroids: Vec<Asteroid>,
}

impl Vertice {
    fn new(asteroid1: &Asteroid, asteroid2: &Asteroid) -> Self {
        Vertice {
            a: (asteroid1.y - asteroid2.y) / (asteroid2.x - asteroid1.x),
            dx: if asteroid1.x < asteroid2.x {
                true
            } else {
                false
            },
            dy: if asteroid1.y < asteroid2.y {
                true
            } else {
                false
            },
            asteroids: vec![],
        }
    }

    fn add_asteroid(&mut self, asteroid: Asteroid, origin: &Asteroid) -> () {
        self.asteroids.push(asteroid);
        self.asteroids
            .sort_by(|a, b| a.partial_cmp(b, origin).unwrap());
    }

    fn destroy_asteroid(&mut self) -> Option<Asteroid> {
        self.asteroids.pop()
    }
}

impl Eq for Vertice {}

impl PartialEq for Vertice {
    fn eq(&self, other: &Self) -> bool {
        self.a == other.a && self.dx == other.dx && self.dy == other.dy
    }
}

impl PartialOrd for Vertice {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if (self.dx != other.dx) || (self.dy != other.dy) {
            /*
                   d1x   | d1y   | d2x   | d2y
                /  true  | false |
                \                | true  | true
                 / true  | false |
                /                | false | true
                \/ true  | false | false | false


                /\ true  | true  | false | trues
                \                | false | false
                 \ true  | true  |
                \                | false | false
                /  false | true  |
            */
            if (self.dx == true && (self.dy == false || (other.dx != true || other.dy != false)))
                || (self.dy == true && other.dx == false && other.dy == false)
            {
                return Some(Ordering::Less);
            }
            /*
                   d1x   | d1y   | d2x   | d2y
                /                | true  | false
                \  true  | true  |
                 /               | true  | false
                /  false | true  |
                \/ false | false | true  | false


                /\ false | true  | true  | true
                \  false | false |
                 \               | true  | true
                \  false | false |
                /                | false | true
            */
            return Some(Ordering::Greater);
        }

        /*
          d1x   | d1y   | d2x   | d2y
          true  | true  | true  | true
          true  | false | true  | false
          false | true  | false | true
          false | false | false | false
        */

        if self.a > other.a {
            return Some(Ordering::Less);
        }

        if self.a < other.a {
            return Some(Ordering::Greater);
        }

        Some(Ordering::Equal)
    }
}

#[cfg(test)]
mod tests {
    use super::find_monitoring_station_asteroid;
    use super::Asteroid;
    use super::Vertice;

    #[test]
    fn it_should_find_monitoring_station_asteroid_1() {
        let universe: Vec<&str> = vec![".#..#", ".....", "#####", "....#", "...##"];
        assert_eq!(find_monitoring_station_asteroid(&universe, None).0, 8);
    }

    #[test]
    fn it_should_find_monitoring_station_asteroid_2() {
        let universe: Vec<&str> = vec![
            "......#.#.",
            "#..#.#....",
            "..#######.",
            ".#.#.###..",
            ".#..#.....",
            "..#....#.#",
            "#..#....#.",
            ".##.#..###",
            "##...#..#.",
            ".#....####",
        ];
        assert_eq!(find_monitoring_station_asteroid(&universe, None).0, 33);
    }

    #[test]
    fn it_should_find_monitoring_station_asteroid_3() {
        let universe: Vec<&str> = vec![
            "#.#...#.#.",
            ".###....#.",
            ".#....#...",
            "##.#.#.#.#",
            "....#.#.#.",
            ".##..###.#",
            "..#...##..",
            "..##....##",
            "......#...",
            ".####.###.",
        ];
        assert_eq!(find_monitoring_station_asteroid(&universe, None).0, 35);
    }

    #[test]
    fn it_should_find_monitoring_station_asteroid_4() {
        let universe: Vec<&str> = vec![
            ".#..#..###",
            "####.###.#",
            "....###.#.",
            "..###.##.#",
            "##.##.#.#.",
            "....###..#",
            "..#.#..#.#",
            "#..#.#.###",
            ".##...##.#",
            ".....#.#..",
        ];
        assert_eq!(find_monitoring_station_asteroid(&universe, None).0, 41);
    }

    #[test]
    fn it_should_find_monitoring_station_asteroid_5() {
        let universe: Vec<&str> = vec![
            ".#..##.###...#######",
            "##.############..##.",
            ".#.######.########.#",
            ".###.#######.####.#.",
            "#####.##.#.##.###.##",
            "..#####..#.#########",
            "####################",
            "#.####....###.#.#.##",
            "##.#################",
            "#####.##.###..####..",
            "..######..##.#######",
            "####.##.####...##..#",
            ".#####..#.######.###",
            "##...#.##########...",
            "#.##########.#######",
            ".####.#.###.###.#.##",
            "....##.##.###..#####",
            ".#.#.###########.###",
            "#.#.#.#####.####.###",
            "###.##.####.##.#..##",
        ];
        assert_eq!(find_monitoring_station_asteroid(&universe, None).0, 210);
    }

    #[test]
    fn it_should_sort_vertices_1() {
        let mut vec = vec![
            Vertice::new(&Asteroid::new(11.0, 13.0), &Asteroid::new(12.0, 7.0)),
            Vertice::new(&Asteroid::new(11.0, 13.0), &Asteroid::new(10.0, 19.0)),
            Vertice::new(&Asteroid::new(11.0, 13.0), &Asteroid::new(13.0, 2.0)),
        ];

        vec.sort_by(|a, b|  a.partial_cmp(b).unwrap());

        assert_eq!(
            vec,
            vec![
                Vertice::new(&Asteroid::new(11.0, 13.0), &Asteroid::new(12.0, 7.0)),
                Vertice::new(&Asteroid::new(11.0, 13.0), &Asteroid::new(13.0, 2.0)),
                Vertice::new(&Asteroid::new(11.0, 13.0), &Asteroid::new(10.0, 19.0)),
            ]
        );
    }

    #[test]
    fn it_should_find_the_200_th_asteroid_to_be_destroyed() {
        let universe: Vec<&str> = vec![
            ".#..##.###...#######",
            "##.############..##.",
            ".#.######.########.#",
            ".###.#######.####.#.",
            "#####.##.#.##.###.##",
            "..#####..#.#########",
            "####################",
            "#.####....###.#.#.##",
            "##.#################",
            "#####.##.###..####..",
            "..######..##.#######",
            "####.##.####...##..#",
            ".#####..#.######.###",
            "##...#.##########...",
            "#.##########.#######",
            ".####.#.###.###.#.##",
            "....##.##.###..#####",
            ".#.#.###########.###",
            "#.#.#.#####.####.###",
            "###.##.####.##.#..##",
        ];
        assert_eq!(
            find_monitoring_station_asteroid(&universe, Some(200)).1,
            802.0
        );
    }
}
