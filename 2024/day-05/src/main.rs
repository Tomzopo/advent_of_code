use std::collections::{HashMap, HashSet};

fn main() {
    let input = include_str!("../input/input.txt");
    println!("{}", part_one(input));
    println!("{}", part_two(input));
}

fn parse_input(input: &str) -> (HashMap<i64, HashSet<i64>>, Vec<Vec<i64>>) {
    let (ordering_input, queue_input) = input.split_once("\n\n").unwrap();

    let ordering_map = ordering_input
        .lines()
        .map(|line| line.split_once('|').unwrap())
        .map(|(a, b)| (a.parse().unwrap(), b.parse().unwrap()))
        .fold(HashMap::new(), |mut map, (a, b)| {
            map.entry(b).or_insert_with(HashSet::new).insert(a);
            map
        });

    let queue = queue_input
        .lines()
        .map(|line| {
            line.split(',')
                .map(|s| s.parse().unwrap())
                .collect::<Vec<_>>()
        })
        .collect();

    (ordering_map, queue)
}

fn is_sorted(ordering_map: &HashMap<i64, HashSet<i64>>, pq: &[i64]) -> bool {
    pq.windows(2).all(|w| {
        ordering_map
            .get(&w[1])
            .map_or(false, |set| set.contains(&w[0]))
    })
}

fn sort_queue(ordering_map: &HashMap<i64, HashSet<i64>>, pq: &mut Vec<i64>) {
    pq.sort_by(|a, b| match ordering_map.get(b) {
        Some(set) => set.contains(a).cmp(&true),
        None => std::cmp::Ordering::Less,
    });
}

fn part_one(input: &str) -> i64 {
    let (ordering_map, queue) = parse_input(input);

    queue
        .iter()
        .filter_map(|pq| {
            if is_sorted(&ordering_map, pq) {
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
            if !is_sorted(&ordering_map, pq) {
                sort_queue(&ordering_map, pq);
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
