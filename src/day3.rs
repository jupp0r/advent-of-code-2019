use std::collections::{HashMap, HashSet};
use std::io::{stdin, Read};

pub fn run() {
    let mut s = Vec::new();
    stdin()
        .read_to_end(&mut s)
        .expect("did not enter a correct string");
    let input_string = std::str::from_utf8(&s).expect("invalid utf8");

    let path_strings = input_string.lines().collect::<Vec<&str>>();

    let (_, dist1, dist2) =
        closest_crosspoint(path_strings[0], path_strings[1]).expect("paths don't cross");
    println!("{}", dist1 + dist2);
}

fn closest_crosspoint(path1: &str, path2: &str) -> Option<(Coordinate, usize, usize)> {
    let path1_points = points_on_path(&parse_path(&path1));
    let path2_points = points_on_path(&parse_path(&path2));
    let cross_points: Vec<(&Coordinate, usize, usize)> = path1_points
        .0
        .intersection(&path2_points.0)
        .map(|coord| {
            (
                coord,
                path1_points.1.get(coord).unwrap().clone(),
                path2_points.1.get(coord).unwrap().clone(),
            )
        })
        .collect();

    let min_crosspoint = cross_points
        .iter()
        .filter(|(c, _, _)| c != &&Coordinate { x: 0, y: 0 })
        .min_by(|&a, &b| (a.1 + a.2).cmp(&(b.1 + b.2)));
    min_crosspoint
        .copied()
        .map(|(&coord, dist1, dist2)| (coord, dist1, dist2))
}

fn parse_path(path: &str) -> Path {
    let instructions = path
        .split(",")
        .map(|instruction| {
            let direction = match instruction.get(0..1) {
                Some("L") => Direction::Left,
                Some("R") => Direction::Right,
                Some("U") => Direction::Up,
                Some("D") => Direction::Down,
                _ => panic!("invalid token found in input stream"),
            };
            let distance = instruction
                .get(1..)
                .expect("missing distance token")
                .parse::<usize>()
                .expect("expected integer token");
            (direction, distance)
        })
        .collect();
    Path { instructions }
}

fn points_on_path(path: &Path) -> (HashSet<Coordinate>, HashMap<Coordinate, usize>) {
    let mut coords = HashSet::new();
    let mut steps = HashMap::new();
    let mut pointer = Coordinate { x: 0, y: 0 };
    let mut step_counter = 0;

    for (direction, distance) in &path.instructions {
        match direction {
            Direction::Up => {
                for i in 0..*distance {
                    let coord = Coordinate {
                        y: pointer.y + (i as i64),
                        ..pointer
                    };
                    coords.insert(coord);
                    steps.entry(coord).or_insert(step_counter + i);
                }
                pointer.y = pointer.y + (*distance as i64);
            }
            Direction::Down => {
                for i in 0..*distance {
                    let coord = Coordinate {
                        y: pointer.y - (i as i64),
                        ..pointer
                    };
                    coords.insert(coord);
                    steps.entry(coord).or_insert(step_counter + i);
                }
                pointer.y = pointer.y - (*distance as i64);
            }
            Direction::Left => {
                for i in 0..*distance {
                    let coord = Coordinate {
                        x: pointer.x - (i as i64),
                        ..pointer
                    };
                    coords.insert(coord);
                    steps.entry(coord).or_insert(step_counter + i);
                }
                pointer.x = pointer.x - (*distance as i64);
            }
            Direction::Right => {
                for i in 0..*distance {
                    let coord = Coordinate {
                        x: pointer.x + (i as i64),
                        ..pointer
                    };
                    coords.insert(coord);
                    steps.entry(coord).or_insert(step_counter + i);
                }
                pointer.x = pointer.x + (*distance as i64);
            }
        }
        step_counter = step_counter + distance;
    }

    (coords, steps)
}

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug)]
struct Path {
    instructions: Vec<(Direction, usize)>,
}

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
struct Coordinate {
    x: i64,
    y: i64,
}

#[cfg(test)]
mod tests {
    use super::*;

    // disabled because selection algorithm changed for part 2 of day 3
    // #[test]
    // fn test_crosspoint_example1() {
    //     let (crosspoint, _, _) = closest_crosspoint(
    //         "R75,D30,R83,U83,L12,D49,R71,U7,L72",
    //         "U62,R66,U55,R34,D71,R55,D58,R83",
    //     )
    //     .unwrap();
    //     assert_eq!(crosspoint.x.abs() + crosspoint.y.abs(), 159);
    // }

    // #[test]
    // fn test_crosspoint_example2() {
    //     let (crosspoint, _, _) = closest_crosspoint(
    //         "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51",
    //         "U98,R91,D20,R16,D67,R40,U7,R15,U6,R7",
    //     )
    //     .unwrap();
    //     assert_eq!(crosspoint.x.abs() + crosspoint.y.abs(), 135);
    // }

    // #[test]
    // fn test_crosspoints_example3() {
    //     let (crosspoint, _, _) = closest_crosspoint("R8,U5,L5,D3", "U7,R6,D4,L4").unwrap();
    //     assert_eq!(crosspoint, Coordinate { x: 3, y: 3 });
    // }

    #[test]
    fn test_crosspoints_steps() {
        let (_, dist1, dist2) = closest_crosspoint(
            "R75,D30,R83,U83,L12,D49,R71,U7,L72",
            "U62,R66,U55,R34,D71,R55,D58,R83",
        )
        .unwrap();
        assert_eq!(dist1 + dist2, 610);
    }

    #[test]
    fn test_crosspoints_steps2() {
        let (_, dist1, dist2) = closest_crosspoint(
            "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51",
            "U98,R91,D20,R16,D67,R40,U7,R15,U6,R7",
        )
        .unwrap();
        assert_eq!(dist1 + dist2, 410);
    }
}
