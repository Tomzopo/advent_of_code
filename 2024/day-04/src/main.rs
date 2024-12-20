fn main() {
    let input = include_str!("../input/input.txt");
    println!("{}", part_one(input));
    println!("{}", part_two(input));
}

fn parse_input(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

const DIRECTIONS: [(i32, i32); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

const WORD: &[char] = &['X', 'M', 'A', 'S'];
const WORD_TWO: &[char] = &['M', 'A', 'S'];

fn part_one(input: &str) -> i64 {
    let input = parse_input(input);

    let mut count = 0;
    for y in 0..input.len() {
        for x in 0..input[0].len() {
            count += search_around_cell(&input, x, y);
        }
    }

    count
}

fn in_bounds(x: i32, y: i32, m: i32, n: i32) -> bool {
    x >= 0 && x < m && y >= 0 && y < n
}

fn search_around_cell(input: &[Vec<char>], row: usize, col: usize) -> i64 {
    let m = input.len() as i32;
    let n = input[0].len() as i32;

    if input[row][col] != WORD[0] {
        return 0;
    }

    let mut count = 0;

    for (dx, dy) in DIRECTIONS {
        let mut dis = 1;
        let mut curr_x = row as i32 + dx;
        let mut curr_y = col as i32 + dy;

        while dis < WORD.len() {
            if !in_bounds(curr_x, curr_y, m, n) {
                break;
            }

            if input[curr_x as usize][curr_y as usize] != WORD[dis] {
                break;
            }

            curr_x += dx;
            curr_y += dy;
            dis += 1;
        }

        if dis == WORD.len() {
            count += 1;
        }
    }

    count
}

fn part_two(input: &str) -> i64 {
    let input = parse_input(input);

    let mut count = 0;
    for y in 1..input.len() - 1 {
        for x in 1..input[0].len() - 1 {
            count += search_xmas_around_cell(&input, x, y);
        }
    }

    count
}

fn search_xmas_around_cell(input: &[Vec<char>], row: usize, col: usize) -> i64 {
    let mut count = 0;
    let num_diff = (WORD_TWO[0] as i32 - WORD_TWO[2] as i32).abs();

    if input[row][col] != WORD_TWO[1] {
        return 0;
    } else {
        let tl = input[row - 1][col - 1];
        let tr = input[row - 1][col + 1];
        let bl = input[row + 1][col - 1];
        let br = input[row + 1][col + 1];

        if (tl as i32 - br as i32).abs() == num_diff && (tr as i32 - bl as i32).abs() == num_diff {
            count += 1;
        }
    }

    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one_example() {
        let input = r#"MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX"#;
        assert_eq!(part_one(input), 18);
    }

    #[test]
    fn test_part_one() {
        let input = include_str!("../input/input.txt");
        assert_eq!(part_one(input), 2534);
    }

    #[test]
    fn test_part_two_example() {
        let input = r#"MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX"#;
        assert_eq!(part_two(input), 9);
    }

    #[test]
    fn test_part_two() {
        let input = include_str!("../input/input.txt");
        assert_eq!(part_two(input), 1866);
    }
}
