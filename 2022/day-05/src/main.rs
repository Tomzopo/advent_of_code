use std::collections::VecDeque;

#[derive(Debug)]
struct CargoMove {
    count: usize,
    from: usize,
    to: usize,
}

fn main() {
    let file = include_str!("../input/input.txt");
    let part1 = crane_9000(file);
    println!("Part 1: {}", part1);
    let part2 = crane_9001(file);
    println!("Part 2: {}", part2);
}

fn parse_input(input: &str) -> (Vec<VecDeque<char>>, Vec<CargoMove>) {
    let (stack, moves) = input.split_at(input.find("\r\n\r\n").unwrap());

    let stacks: Vec<VecDeque<_>> = stack
        .lines()
        .next_back()
        .unwrap()
        .match_indices(|c: char| c.is_ascii_digit())
        .map(|(m, _)| {
            stack
                .lines()
                .rev()
                .skip(1)
                .filter_map(|line| {
                    line.chars().nth(m).and_then(|c| match c {
                        'A'..='Z' => Some(c),
                        ' ' => None,
                        _ => panic!("Unexpected character: {}", c),
                    })
                })
                .collect()
        })
        .collect();

    let moves: Vec<CargoMove> = moves
        .lines()
        .skip(2)
        .map(|line| {
            let mut mod_line = line.to_string();
            mod_line = mod_line.replace("move ", "");
            mod_line = mod_line.replace(" from", "");
            mod_line = mod_line.replace(" to", "");
            mod_line
        })
        .map(|line| {
            let mut split = line.split_whitespace();
            let count = split.next().unwrap().parse::<u64>().unwrap() as usize;
            let from = split.next().unwrap().parse::<u64>().unwrap() as usize;
            let to = split.next().unwrap().parse::<u64>().unwrap() as usize;
            CargoMove { count, from, to }
        })
        .collect();

    (stacks, moves)
}

fn crane_9000(input: &str) -> String {
    let (mut stacks, moves) = parse_input(input);

    for mv in moves {
        for _ in 0..mv.count {
            let temp = stacks[mv.from - 1].pop_back().unwrap();
            stacks[mv.to - 1].push_back(temp);
        }
    }
    stacks.iter().map(|s| s.back().unwrap()).collect()
}

fn crane_9001(input: &str) -> String {
    let (mut stacks, moves) = parse_input(input);

    for mv in moves {
        for _ in 0..mv.count {
            let temp = stacks[mv.from - 1].pop_back().unwrap();
            stacks[mv.to - 1].push_front(temp);
        }
        stacks[mv.to - 1].rotate_left(mv.count);
    }
    stacks.iter().map(|s| s.back().unwrap()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn crane_9000_test() {
        let input = include_str!("../input/test.txt");
        assert_eq!(crane_9000(input), "CMZ");
    }

    #[test]
    fn crane_9001_test() {
        let input = include_str!("../input/test.txt");
        assert_eq!(crane_9001(input), "MCD");
    }

    #[test]
    fn check_solutions() {
        let input = include_str!("../input/input.txt");
        assert_eq!(crane_9000(input), "SVFDLGLWV");
        assert_eq!(crane_9001(input), "DCVTCVPCL");
    }
}
