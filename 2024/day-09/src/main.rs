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
    check_sum(input)
}

fn parse_input(input: &str) -> Vec<Option<i64>> {
    let mut id = 0;
    input
        .chars()
        .map(|c| c.to_digit(10).unwrap() as usize)
        .enumerate()
        .flat_map(|(i, desc)| {
            if i % 2 == 0 {
                let elems = vec![Some(id); desc];
                id += 1;
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

fn get_max_id(input: &[Option<i64>]) -> &i64 {
    input.iter().filter_map(|x| x.as_ref()).max().unwrap()
}
fn compact_data_part_two(mut input: Vec<Option<i64>>) -> Vec<Option<i64>> {
    let max_id = get_max_id(&input);

    for id in (0..=*max_id).rev() {
        let some_start_index = input.iter().position(|&x| x == Some(id));
        let some_end_index = input.iter().rposition(|&x| x == Some(id));
        let some_length = some_end_index.unwrap() - some_start_index.unwrap() + 1;
        // println!(
        //     "ID: {}, Some start: {}, Some end: {}, Length: {}",
        //     id,
        //     some_start_index.unwrap(),
        //     some_end_index.unwrap(),
        //     some_length
        // );

        let mut offset = 0;
        loop {
            let (none_start_index, none_end_index) = find_none_chunk(&input, offset).unwrap();
            let none_length = none_end_index - none_start_index + 1;
            // println!(
            //     "ID: {}, None start: {}, None end: {}, Length: {}",
            //     id, none_start_index, none_end_index, none_length
            // );

            if none_start_index >= some_end_index.unwrap() {
                break;
            }

            if some_length <= none_length {
                for inp in input.iter_mut().skip(none_start_index).take(some_length) {
                    *inp = Some(id);
                }

                for inp in input
                    .iter_mut()
                    .take(some_end_index.unwrap() + 1)
                    .skip(some_start_index.unwrap())
                {
                    *inp = None;
                }
                break;
            }

            offset = none_end_index + 1;
        }

        // println!("{:?}", input);
    }

    input
}

fn find_none_chunk(input: &[Option<i64>], offset: usize) -> Option<(usize, usize)> {
    let mut in_none_chunk = false;
    let mut start_index = None;

    for (i, element) in input.iter().enumerate().skip(offset) {
        match (element, in_none_chunk) {
            (None, false) => {
                start_index = Some(i);
                in_none_chunk = true;
            }
            (Some(_), true) => {
                return start_index.map(|start| (start, i - 1));
            }
            _ => {}
        }
    }

    if in_none_chunk {
        return start_index.map(|start| (start, input.len() - 1));
    }

    None
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
        assert_eq!(part_two(input), 6412390114238);
    }
}
