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
struct Antinode {
    x: i64,
    y: i64,
}

struct AntennaGrid {
    rows_len: usize,
    cols_len: usize,
    antennas: Vec<Antenna>,
}

impl Antinode {
    fn in_map(self, max_x: i64, max_y: i64) -> Option<Antinode> {
        if self.x >= 0 && self.x < max_x && self.y >= 0 && self.y < max_y {
            Some(self)
        } else {
            None
        }
    }
}

impl Antenna {
    fn get_antinodes(
        &self,
        other: &Self,
        max_x: i64,
        max_y: i64,
    ) -> (Option<Antinode>, Option<Antinode>) {
        let (dx, dy) = (self.x - other.x, self.y - other.y);

        let antinode_one = Antinode {
            x: self.x + dx,
            y: self.y + dy,
        };

        let antinode_two = Antinode {
            x: other.x - dx,
            y: other.y - dy,
        };

        (
            antinode_one.in_map(max_x, max_y),
            antinode_two.in_map(max_x, max_y),
        )
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
                        Some(antenna1.get_antinodes(
                            antenna2,
                            grid.cols_len as i64,
                            grid.rows_len as i64,
                        ))
                    } else {
                        None
                    }
                })
        })
        .flat_map(|(antinode_one, antinode_two)| [antinode_one, antinode_two])
        .flatten()
        .collect::<HashSet<_>>()
        .len() as i64
}

fn part_two(input: &str) -> i64 {
    todo!()
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
    fn test_antinode_locator() {
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
            antenna_one.get_antinodes(&antenna_two, grid.cols_len as i64, grid.rows_len as i64),
            (Some(Antinode { x: 3, y: 1 }), Some(Antinode { x: 6, y: 7 }))
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
    fn test_part_two() {
        let input = include_str!("../input/input.txt");
        assert_eq!(part_two(input), 0);
    }
}
