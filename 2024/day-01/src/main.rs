use std::collections::HashMap;

fn main() {
    let input = include_str!("../input/input.txt");
    println!("{}", part_one(input));
    println!("{}", part_two(input));
}

fn part_one(input: &str) -> i64 {
    let mut vec1 = Vec::new();
    let mut vec2 = Vec::new();

    for line in input.lines() {
        let mut nums = line
            .split_whitespace()
            .map(|num| num.parse::<i64>().unwrap());
        vec1.push(nums.next().unwrap());
        vec2.push(nums.next().unwrap());
    }

    vec1.sort();
    vec2.sort();

    let mut sum: i64 = 0;
    for i in 0..vec1.len() {
        sum += (vec1[i] - vec2[i]).abs();
    }

    sum
}

fn part_two(input: &str) -> i64 {
    let pairs: Vec<(i64, i64)> = input
        .lines()
        .map(|line| {
            let mut nums = line
                .split_whitespace()
                .map(|num| num.parse::<i64>().unwrap());
            (nums.next().unwrap(), nums.next().unwrap())
        })
        .collect();

    let mut map: HashMap<i64, i64> = HashMap::new();
    for (_, val) in &pairs {
        *map.entry(*val).or_insert(0) += 1;
    }

    pairs
        .iter()
        .map(|(key, _)| key * map.get(key).unwrap_or(&0))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = r#"3 4
        4 3
        2 5
        1 3
        3 9
        3 3"#;
        assert_eq!(part_one(input), 11)
    }

    #[test]
    fn test_part_one_res() {
        let input = include_str!("../input/input.txt");
        assert_eq!(part_one(input), 1223326)
    }

    #[test]
    fn test_part_two() {
        let input = r#"3 4
        4 3
        2 5
        1 3
        3 9
        3 3"#;
        assert_eq!(part_two(input), 31)
    }

    #[test]
    fn test_part_two_res() {
        let input = include_str!("../input/input.txt");
        assert_eq!(part_two(input), 21070419)
    }
}
