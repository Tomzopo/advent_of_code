pub fn part_one(input: &str) -> i64 {
    input
        .lines()
        .map(|line| {
            let nums: Vec<_> = line.chars().filter(|c| c.is_ascii_digit()).collect();
            (nums.first().unwrap().to_string() + &nums.last().unwrap().to_string())
                .parse::<i64>()
                .unwrap()
        })
        .collect::<Vec<_>>()
        .iter()
        .sum()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = r#"1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet"#;
        println!("{}", input);
        assert_eq!(part_one(input), 142)
    }
}
