use std::{cmp::PartialEq, collections::HashSet};

use crate::Tile::Obstacle;

fn main() {
    let input = include_str!("../input/input.txt");
    println!("{}", part_one(input));
    println!("{}", part_two(input));
}

#[derive(Debug, PartialEq)]
enum Tile {
    Obstacle,
    Start,
}

#[derive(Debug, Eq, PartialEq, Copy, Clone, Hash)]
struct Position {
    x: i64,
    y: i64,
    direction: Direction,
}

impl Position {
    fn patrol(&self) -> Self {
        let (new_x, new_y) = match self.direction {
            Direction::Up => (self.x, self.y - 1),
            Direction::Right => (self.x + 1, self.y),
            Direction::Down => (self.x, self.y + 1),
            Direction::Left => (self.x - 1, self.y),
        };

        Self {
            x: new_x,
            y: new_y,
            direction: self.direction,
        }
    }

    fn rotate(&mut self) {
        use Direction::*;
        self.direction = match self.direction {
            Up => Right,
            Right => Down,
            Down => Left,
            Left => Up,
        };
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Default for Direction {
    fn default() -> Self {
        Self::Up
    }
}

fn parse_grid(input: &str) -> Vec<Vec<Option<Tile>>> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    '^' => Some(Tile::Start),
                    '#' => Some(Tile::Obstacle),
                    _ => None,
                })
                .collect()
        })
        .collect()
}

fn get_starting_position(grid: &[Vec<Option<Tile>>]) -> Option<(i64, i64)> {
    grid.iter().enumerate().find_map(|(y, row)| {
        row.iter().enumerate().find_map(|(x, tile)| match tile {
            Some(Tile::Start) => Some((x as i64, y as i64)),
            _ => None,
        })
    })
}

fn part_one(input: &str) -> i64 {
    let grid = parse_grid(input);
    let (start_x, start_y) = get_starting_position(&grid).unwrap();

    let mut position = Position {
        x: start_x,
        y: start_y,
        direction: Direction::Up,
    };

    let (row_size, col_size) = (grid.len(), grid[0].len());
    let mut visited_set = HashSet::new();
    visited_set.insert((position.x, position.y));

    loop {
        let new_position = position.patrol();
        if !in_bounds(new_position.x, new_position.y, row_size, col_size) {
            break;
        } else if grid[new_position.y as usize][new_position.x as usize] == Some(Obstacle) {
            position.rotate();
        } else {
            position = new_position;
            visited_set.insert((position.x, position.y));
        }
    }

    visited_set.len() as i64
}

fn part_two(input: &str) -> i64 {
    let grid = parse_grid(input);
    let (start_x, start_y) = get_starting_position(&grid).unwrap();
    let starting_position = Position {
        x: start_x,
        y: start_y,
        direction: Direction::Up,
    };
    let mut position = starting_position;

    let (row_size, col_size) = (grid.len(), grid[0].len());
    let mut visited_set = HashSet::new();

    loop {
        let new_position = position.patrol();
        if !in_bounds(new_position.x, new_position.y, row_size, col_size) {
            break;
        } else if grid[new_position.y as usize][new_position.x as usize] == Some(Obstacle) {
            position.rotate();
        } else {
            position = new_position;
            visited_set.insert((position.x, position.y));
        }
    }

    // I am not proud of this :(
    let res: Vec<_> = visited_set
        .iter()
        .filter(|new_wall| {
            let mut position = starting_position;
            let mut visited_set = HashSet::<Position>::new();
            visited_set.insert(position);

            loop {
                let new_position = position.patrol();
                if !in_bounds(new_position.x, new_position.y, row_size, col_size) {
                    break false;
                } else if grid[new_position.y as usize][new_position.x as usize] == Some(Obstacle)
                    || (new_position.x == new_wall.0 && new_position.y == new_wall.1)
                {
                    position.rotate();
                    continue;
                }
                if visited_set.contains(&new_position) {
                    break true;
                }
                position = new_position;
                visited_set.insert(position);
            }
        })
        .collect();

    res.len() as i64
}

fn in_bounds(x: i64, y: i64, rows: usize, cols: usize) -> bool {
    x >= 0 && x < cols as i64 && y >= 0 && y < rows as i64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one_example() {
        let input = r#"....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#..."#;
        assert_eq!(part_one(input), 41);
    }

    #[test]
    fn test_part_one() {
        let input = include_str!("../input/input.txt");
        assert_eq!(part_one(input), 5551);
    }

    #[test]
    fn test_part_two_example() {
        let input = r#"....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#..."#;
        assert_eq!(part_two(input), 6);
    }

    #[test]
    fn test_part_two() {
        let input = include_str!("../input/input.txt");
        assert_eq!(part_two(input), 1939);
    }
}
