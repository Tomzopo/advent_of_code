fn main() {
    let input = include_str!("../input/input.txt");
    println!("{}", part_one(input));
    println!("{}", part_two(input));
}

fn part_one(input: &str) -> i64 {
    let input = parse_input(input);
    let input = compact_data_part_one(input);
    check_sum(input)
}

fn part_two(input: &str) -> i64 {
    let input = parse_input(input);
    let input = compact_data_part_two(input);
    println!("{:?}", input);
    check_sum(input)
}

fn parse_input(input: &str) -> Vec<Option<i64>> {
    let mut num = 0;
    input
        .chars()
        .map(|c| c.to_digit(10).unwrap() as usize)
        .enumerate()
        .flat_map(|(i, desc)| {
            if i % 2 == 0 {
                let elems = vec![Some(num); desc];
                num += 1;
                elems
            } else {
                vec![None; desc]
            }
        })
        .collect()
}

fn compact_data_part_one(mut input: Vec<Option<i64>>) -> Vec<Option<i64>> {
    let mut last_some_index = input.iter().rposition(|x| x.is_some());
    let mut first_none_index = input.iter().position(|x| x.is_none());

    while let (Some(last_some), Some(first_none)) = (last_some_index, first_none_index) {
        if first_none > last_some {
            break;
        }
        input.swap(last_some, first_none);

        last_some_index = input.iter().rposition(|x| x.is_some());
        first_none_index = input.iter().position(|x| x.is_none());
    }
    input
}
fn compact_data_part_two(mut input: Vec<Option<i64>>) -> Vec<Option<i64>> {
    todo!()
}

fn check_sum(input: Vec<Option<i64>>) -> i64 {
    input
        .iter()
        .enumerate()
        .filter_map(|(i, x)| x.as_ref().map(|x| x * i as i64))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one_example() {
        let input = r#"2333133121414131402"#;
        assert_eq!(part_one(input), 1928);
    }

    #[test]
    fn test_part_one() {
        let input = include_str!("../input/input.txt");
        assert_eq!(part_one(input), 6390180901651);
    }

    #[test]
    fn test_part_two_example() {
        let input = r#"2333133121414131402"#;
        assert_eq!(part_two(input), 2858);
    }

    #[test]
    fn test_part_two() {
        let input = include_str!("../input/input.txt");
        assert_eq!(part_two(input), 0);
    }
}
