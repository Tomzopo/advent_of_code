fn main() {
    let input = include_str!("../input/input.txt");
    println!("{}", part_one(input));
    println!("{}", part_two(input));
}

fn part_one(input: &str) -> i64 {
    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|num| num.parse().unwrap())
                .collect::<Vec<i64>>()
        })
        .filter(|nums| check_report(nums))
        .count() as i64
}

fn part_two(input: &str) -> i64 {
    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|num| num.parse().unwrap())
                .collect::<Vec<i64>>()
        })
        .filter(|rep| {
            check_report(rep) || (0..rep.len()).any(|x| check_report(&remove_one(rep, x)))
        })
        .count() as i64
}

fn remove_one(report: &[i64], index: usize) -> Vec<i64> {
    let mut new_report = report.to_vec();
    new_report.remove(index);
    new_report
}

fn check_report(report: &[i64]) -> bool {
    let is_sorted_asc = is_sorted_asc(report);
    let is_sorted_desc = is_sorted_desc(report);

    (is_sorted_asc || is_sorted_desc)
        && report
            .windows(2)
            .all(|w| (1..=3).contains(&(w[1] - w[0]).abs()))
}

fn is_sorted_asc(nums: &[i64]) -> bool {
    nums.windows(2).all(|w| w[0] <= w[1])
}

fn is_sorted_desc(nums: &[i64]) -> bool {
    nums.windows(2).all(|w| w[0] >= w[1])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = r#"7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9"#;
        assert_eq!(part_one(input), 2)
    }

    #[test]
    fn test_part_one_res() {
        let input = include_str!("../input/input.txt");
        assert_eq!(part_one(input), 472)
    }

    #[test]
    fn test_part_two() {
        let input = r#"7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9"#;
        assert_eq!(part_two(input), 4)
    }

    #[test]
    fn test_part_two_res() {
        let input = include_str!("../input/input.txt");
        assert_eq!(part_two(input), 520)
    }
}
