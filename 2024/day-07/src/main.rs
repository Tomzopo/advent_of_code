use itertools::Itertools;

fn main() {
    let input = include_str!("../input/input.txt");
    println!("{}", part_one(input));
    println!("{}", part_two(input));
}

#[derive(Debug)]
struct Equation {
    res: i64,
    nums: Vec<i64>,
}

fn part_one(input: &str) -> i64 {
    solve_equations(input, is_equation_solvable)
}

fn part_two(input: &str) -> i64 {
    solve_equations(input, is_equation_solvable_part2)
}

fn solve_equations(input: &str, solving_func: fn(&Equation) -> bool) -> i64 {
    input
        .lines()
        .filter_map(|line| {
            let mut split = line.split(':');
            if let (Some(res), Some(nums)) = (split.next(), split.next()) {
                let res = res.parse().unwrap();
                let nums: Vec<i64> = nums
                    .split_whitespace()
                    .map(|s| s.parse().unwrap())
                    .collect();

                if solving_func(&Equation { res, nums }) {
                    return Some(res);
                }
            }
            None
        })
        .sum()
}

fn is_equation_solvable(eq: &Equation) -> bool {
    let nums = &eq.nums;
    let n = nums.len();

    (0..(1 << (n - 1))).any(|operation| {
        let mut result: i64 = nums[0];

        for (i, _) in nums.iter().enumerate().take(n).skip(1) {
            if ((operation >> (i - 1)) & 1) == 1 {
                result *= nums[i];
            } else {
                result += nums[i];
            }
        }
        result == eq.res
    })
}

const OPERATIONS: [&str; 3] = ["+", "*", "||"];
fn is_equation_solvable_part2(eq: &Equation) -> bool {
    let nums = &eq.nums;
    let operation_count = nums.len() - 1;

    (0..operation_count)
        .map(|_| OPERATIONS)
        .multi_cartesian_product()
        .any(|operation_seq| {
            let mut ops = operation_seq.iter();
            let result = nums
                .iter()
                .copied()
                .reduce(|a, b| match *ops.next().unwrap() {
                    "+" => a + b,
                    "*" => a * b,
                    "||" => format!("{}{}", a, b).parse::<i64>().unwrap(),
                    _ => unreachable!("borked"),
                });
            result == Some(eq.res)
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one_example() {
        let input = r#"190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20"#;
        assert_eq!(part_one(input), 3749);
    }

    #[test]
    fn test_part_one() {
        let input = include_str!("../input/input.txt");
        assert_eq!(part_one(input), 20665830408335);
    }

    #[test]
    fn test_part_two_example() {
        let input = r#"190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20"#;
        assert_eq!(part_two(input), 11387);
    }

    #[test]
    fn test_part_two() {
        let input = include_str!("../input/input.txt");
        assert_eq!(part_two(input), 354060705047464);
    }
}
