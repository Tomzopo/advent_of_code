fn main() {
    let file = include_str!("../input/input.txt");
    let part1 = full_range_overlap(file);
    println!("Part 1: {}", part1);
    let part2 = any_range_overlap(file);
    println!("Part 2: {}", part2);
}

fn get_ranges(input: &str) -> Vec<Vec<Vec<u64>>> {
    input
        .lines()
        .map(|input| {
            input
                .split(',')
                .map(|input| {
                    input
                        .split('-')
                        .map(|input| input.parse::<u64>().unwrap())
                        .collect::<Vec<u64>>()
                })
                .collect::<Vec<Vec<u64>>>()
        })
        .collect()
}

fn full_range_overlap(input: &str) -> u64 {
    let res = get_ranges(input);

    res.iter()
        .map(|input| {
            if does_range_fully_contain(input[0].to_owned(), input[1].to_owned()) {
                return 1;
            }
            0
        })
        .sum()
}

fn any_range_overlap(input: &str) -> u64 {
    let res = get_ranges(input);

    res.iter()
        .map(|input| {
            if does_range_overlap(input[0].to_owned(), input[1].to_owned()) {
                return 1;
            }
            0
        })
        .sum()
}

fn does_range_fully_contain(range1: Vec<u64>, range2: Vec<u64>) -> bool {
    (range1.first() >= range2.first() && range1.last() <= range2.last())
        || (range1.first() <= range2.first() && range1.last() >= range2.last())
}

fn does_range_overlap(range1: Vec<u64>, range2: Vec<u64>) -> bool {
    (range1.first() >= range2.first() && range1.first() <= range2.last())
        || (range1.last() >= range2.first() && range1.last() <= range2.last())
        || (range2.first() >= range1.first() && range2.first() <= range1.last())
        || (range2.last() >= range1.first() && range2.last() <= range1.last())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn full_range_overlap_test() {
        let input = concat!(
            "2-4,6-8\n",
            "2-3,4-5\n",
            "5-7,7-9\n",
            "2-8,3-7\n",
            "6-6,4-6\n",
            "2-6,4-8\n"
        );

        assert_eq!(full_range_overlap(input), 2);
    }

    #[test]
    fn any_range_overlap_test() {
        let input = concat!(
            "2-4,6-8\n",
            "2-3,4-5\n",
            "5-7,7-9\n",
            "2-8,3-7\n",
            "6-6,4-6\n",
            "2-6,4-8\n"
        );

        assert_eq!(any_range_overlap(input), 4);
    }

    #[test]
    fn check_solutions() {
        let input = include_str!("../input/input.txt");
        assert_eq!(full_range_overlap(input), 582);
        assert_eq!(any_range_overlap(input), 893);
    }
}
