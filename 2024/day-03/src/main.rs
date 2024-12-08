use regex::Regex;

fn main() {
    let input = include_str!("../input/input.txt");
    println!("{}", part_one(input));
    println!("{}", part_two(input));
}

fn part_one(input: &str) -> i64 {
    let regexp = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    regexp
        .captures_iter(input)
        .map(|cap| cap[1].parse::<i64>().unwrap() * cap[2].parse::<i64>().unwrap())
        .sum()
}

fn part_two(input: &str) -> i64 {
    let regexp = Regex::new(r"mul\((\d+),(\d+)\)|do\(\)|don't\(\)").unwrap();
    let mut enabled = true;
    regexp
        .captures_iter(input)
        .map(|cap| {
            if cap[0] == *"don't()" {
                enabled = false;
            } else if cap[0] == *"do()" {
                enabled = true;
            } else if enabled {
                return cap[1].parse::<i64>().unwrap() * cap[2].parse::<i64>().unwrap();
            }
            0
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one_example() {
        let input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        assert_eq!(part_one(input), 161)
    }

    #[test]
    fn test_part_one() {
        let input = include_str!("../input/input.txt");
        assert_eq!(part_one(input), 175015740)
    }

    #[test]
    fn test_part_two_example() {
        let input = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
        assert_eq!(part_two(input), 48)
    }

    #[test]
    fn test_part_two() {
        let input = include_str!("../input/input.txt");
        assert_eq!(part_two(input), 112272912);
    }
}
