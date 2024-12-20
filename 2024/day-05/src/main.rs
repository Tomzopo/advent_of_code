use std::collections::{HashMap, HashSet};

fn main() {
    let input = include_str!("../input/input.txt");
    println!("{}", part_one(input));
    println!("{}", part_two(input));
}

fn parse_input(input: &str) -> (HashMap<i64, HashSet<i64>>, Vec<Vec<i64>>) {
    let (ordering_input, queue_input) = input.split_once("\n\n").unwrap();

    let ordering = ordering_input
        .lines()
        .map(|line| line.split_once('|').unwrap())
        .map(|(a, b)| (a.parse::<i64>().unwrap(), b.parse::<i64>().unwrap()))
        .collect::<Vec<(i64, i64)>>();

    let mut ordering_map = HashMap::new();
    for (a, b) in ordering {
        ordering_map.entry(b).or_insert_with(HashSet::new).insert(a);
    }

    let queue = queue_input
        .lines()
        .map(|line| {
            line.split(',')
                .map(|s| s.parse::<i64>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    (ordering_map, queue)
}

fn part_one(input: &str) -> i64 {
    let (ordering_map, queue) = parse_input(input);

    queue
        .iter()
        .filter_map(|pq| {
            if pq.windows(2).all(
                |w| match (ordering_map.get(&w[1]), ordering_map.get(&w[0])) {
                    (Some(set), _) => set.contains(&w[0]),
                    _ => false,
                },
            ) {
                Some(pq[pq.len() / 2])
            } else {
                None
            }
        })
        .sum()
}

fn part_two(input: &str) -> i64 {
    let (ordering_map, mut queue) = parse_input(input);

    queue
        .iter_mut()
        .filter_map(|pq| {
            if pq.windows(2).any(
                |w| match (ordering_map.get(&w[1]), ordering_map.get(&w[0])) {
                    (Some(set), _) => !set.contains(&w[0]),
                    _ => true,
                },
            ) {
                pq.sort_by(|a, b| match ordering_map.get(b) {
                    Some(set) => set.contains(a).cmp(&true),
                    None => std::cmp::Ordering::Less,
                });
                Some(pq[pq.len() / 2])
            } else {
                None
            }
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one_example() {
        let input = r#"47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47"#;
        assert_eq!(part_one(input), 143);
    }

    #[test]
    fn test_part_one() {
        let input = include_str!("../input/input.txt");
        assert_eq!(part_one(input), 4959);
    }

    #[test]
    fn test_part_two_example() {
        let input = r#"47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47"#;
        assert_eq!(part_two(input), 123);
    }

    #[test]
    fn test_part_two() {
        let input = include_str!("../input/input.txt");
        assert_eq!(part_two(input), 4655);
    }
}
