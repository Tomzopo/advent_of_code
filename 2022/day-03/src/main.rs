use std::collections::HashSet;

fn main() {
    let file = include_str!("../input/rucksacks.txt");
    let part1 = go_through_sacks_part1(file);
    println!("Part 1: {}", part1);
    let part2 = go_through_sacks_part2(file);
    println!("Part 2: {}", part2);
}

fn go_through_sacks_part1(input: &str) -> u64 {
    let mut sum_priorities: u64 = 0;
    for l in input.lines() {
        let misplaced_item = find_misplaced_item(l);
        sum_priorities += item_priority(misplaced_item);
    }

    sum_priorities
}

fn find_misplaced_item(sack: &str) -> char {
    let (comp1, comp2) = sack.split_at(sack.len() / 2);
    let comp1 = HashSet::<_>::from_iter(comp1.chars());
    let comp2 = HashSet::<_>::from_iter(comp2.chars());
    return *comp1.intersection(&comp2).next().unwrap();
}

fn go_through_sacks_part2(input: &str) -> u64 {
    let mut sum_priorities: u64 = 0;
    let elves: Vec<&str> = input.lines().collect();
    for group in elves.chunks(3) {
        let group_item = find_group_item(group);
        sum_priorities += item_priority(group_item);
    }

    sum_priorities
}

fn find_group_item(elves: &[&str]) -> char {
    let sack1 = HashSet::<_>::from_iter(elves[0].chars());
    let sack2 = HashSet::<_>::from_iter(elves[1].chars());
    let sack3 = HashSet::<_>::from_iter(elves[2].chars());

    let duplicates = sack1
        .intersection(&sack2)
        .copied()
        .collect::<HashSet<char>>();
    return *sack3.intersection(&duplicates).next().unwrap();
}

fn item_priority(item: char) -> u64 {
    if item.is_ascii_uppercase() {
        item as u64 - 65 + 27
    } else if item.is_ascii_lowercase() {
        item as u64 - 97 + 1
    } else {
        panic!("Invalid item");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn item_priority_test() {
        assert_eq!(item_priority('a'), 1);
        assert_eq!(item_priority('A'), 27);
        assert_eq!(item_priority('z'), 26);
        assert_eq!(item_priority('Z'), 52);
    }

    #[test]
    fn part_1() {
        let input = concat!(
            "vJrwpWtwJgWrhcsFMMfFFhFp\n",
            "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL\n",
            "PmmdzqPrVvPwwTWBwg\n",
            "wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn\n",
            "ttgJtRGJQctTZtZT\n",
            "CrZsJsPPZsGzwwsLwLmpwMDw\n",
        );

        assert_eq!(go_through_sacks_part1(input), 157);
    }

    #[test]
    fn part_2() {
        let input = concat!(
            "vJrwpWtwJgWrhcsFMMfFFhFp\n",
            "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL\n",
            "PmmdzqPrVvPwwTWBwg\n",
            "wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn\n",
            "ttgJtRGJQctTZtZT\n",
            "CrZsJsPPZsGzwwsLwLmpwMDw\n",
        );

        assert_eq!(go_through_sacks_part2(input), 70);
    }

    #[test]
    fn results() {
        let input = include_str!("../input/rucksacks.txt");
        assert_eq!(go_through_sacks_part1(input), 7878);
        assert_eq!(go_through_sacks_part2(input), 2760);
    }
}
