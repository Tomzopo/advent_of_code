use std::collections::HashSet;

fn main() {
    let input = include_str!("../input/input.txt");
    println!("{}", part_one(input));
    println!("{}", part_two(input));
}

#[derive(Debug, Copy, Clone)]
struct Antenna {
    value: char,
    x: i64,
    y: i64,
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct AntiNode {
    x: i64,
    y: i64,
}

struct AntennaGrid {
    rows_len: usize,
    cols_len: usize,
    antennas: Vec<Antenna>,
}

impl AntiNode {
    fn in_map(&self, max_x: i64, max_y: i64) -> bool {
        self.x >= 0 && self.x < max_x && self.y >= 0 && self.y < max_y
    }
}

impl Antenna {
    fn get_anti_nodes(
        &self,
        other: &Self,
        max_x: i64,
        max_y: i64,
    ) -> (Option<AntiNode>, Option<AntiNode>) {
        let (dx, dy) = (self.x - other.x, self.y - other.y);

        let anti_node_one = AntiNode {
            x: self.x + dx,
            y: self.y + dy,
        };

        let anti_node_two = AntiNode {
            x: other.x - dx,
            y: other.y - dy,
        };

        let anti_node_one = if anti_node_one.in_map(max_x, max_y) {
            Some(anti_node_one)
        } else {
            None
        };

        let anti_node_two = if anti_node_two.in_map(max_x, max_y) {
            Some(anti_node_two)
        } else {
            None
        };

        (anti_node_one, anti_node_two)
    }

    fn get_anti_nodes_continuous(&self, other: &Self, max_x: i64, max_y: i64) -> Vec<AntiNode> {
        let (dx, dy) = (self.x - other.x, self.y - other.y);
        let mut res: Vec<AntiNode> = Vec::new();
        let (mut new_dx, mut new_dy) = (0, 0);

        loop {
            let anti_node_one = AntiNode {
                x: self.x + new_dx,
                y: self.y + new_dy,
            };

            let anti_node_two = AntiNode {
                x: other.x - new_dx,
                y: other.y - new_dy,
            };

            if !&anti_node_one.in_map(max_x, max_y) && !&anti_node_two.in_map(max_x, max_y) {
                break;
            }

            if anti_node_one.in_map(max_x, max_y) {
                res.push(anti_node_one);
            }

            if anti_node_two.in_map(max_x, max_y) {
                res.push(anti_node_two);
            }

            new_dx += dx;
            new_dy += dy;
        }
        res
    }
}

fn part_one(input: &str) -> i64 {
    let grid = parse_input(input);

    grid.antennas
        .iter()
        .enumerate()
        .flat_map(|(i, antenna1)| {
            grid.antennas
                .iter()
                .skip(i + 1)
                .filter_map(move |antenna2| {
                    if antenna1.value == antenna2.value {
                        Some(antenna1.get_anti_nodes(
                            antenna2,
                            grid.cols_len as i64,
                            grid.rows_len as i64,
                        ))
                    } else {
                        None
                    }
                })
        })
        .flat_map(|(anti_node_one, anti_node_two)| [anti_node_one, anti_node_two])
        .flatten()
        .collect::<HashSet<_>>()
        .len() as i64
}

fn part_two(input: &str) -> i64 {
    let grid = parse_input(input);

    grid.antennas
        .iter()
        .enumerate()
        .flat_map(|(i, antenna1)| {
            grid.antennas
                .iter()
                .skip(i + 1)
                .filter_map(move |antenna2| {
                    if antenna1.value == antenna2.value {
                        Some(antenna1.get_anti_nodes_continuous(
                            antenna2,
                            grid.cols_len as i64,
                            grid.rows_len as i64,
                        ))
                    } else {
                        None
                    }
                })
        })
        .flat_map(|anti_nodes| [anti_nodes])
        .flatten()
        .collect::<HashSet<_>>()
        .len() as i64
}

fn parse_input(input: &str) -> AntennaGrid {
    let rows_len = input.lines().count();
    let cols_len = input.lines().next().unwrap().chars().count();

    let antennas = input
        .lines()
        .enumerate()
        .flat_map(|(row, line)| {
            line.chars()
                .enumerate()
                .filter_map(move |(col, c)| match c {
                    '.' => None,
                    c => Some(Antenna {
                        value: c,
                        x: row as i64,
                        y: col as i64,
                    }),
                })
        })
        .collect();

    AntennaGrid {
        rows_len,
        cols_len,
        antennas,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_anti_node_locator() {
        let antenna_one = Antenna {
            value: 'A',
            x: 4,
            y: 3,
        };
        let antenna_two = Antenna {
            value: 'A',
            x: 5,
            y: 5,
        };
        let grid = AntennaGrid {
            rows_len: 10,
            cols_len: 10,
            antennas: vec![antenna_one, antenna_two],
        };

        assert_eq!(
            antenna_one.get_anti_nodes(&antenna_two, grid.cols_len as i64, grid.rows_len as i64),
            (Some(AntiNode { x: 3, y: 1 }), Some(AntiNode { x: 6, y: 7 }))
        );
    }

    #[test]
    fn test_part_one_example() {
        let input = r#"............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............"#;
        assert_eq!(part_one(input), 14);
    }

    #[test]
    fn test_part_one_example_simple() {
        let input = r#"..........
..........
..........
....a.....
........a.
.....a....
..........
......A...
..........
.........."#;
        assert_eq!(part_one(input), 4);
    }

    #[test]
    fn test_part_one() {
        let input = include_str!("../input/input.txt");
        assert_eq!(part_one(input), 295);
    }

    #[test]
    fn test_part_two_example() {
        let input = r#"............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............"#;
        assert_eq!(part_two(input), 34);
    }

    #[test]
    fn test_part_two_example_simple() {
        let input = r#"T.........
...T......
.T........
..........
..........
..........
..........
..........
..........
.........."#;
        assert_eq!(part_two(input), 9);
    }

    #[test]
    fn test_part_two() {
        let input = include_str!("../input/input.txt");
        assert_eq!(part_two(input), 1034);
    }
}
