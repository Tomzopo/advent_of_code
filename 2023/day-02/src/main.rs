mod part1;
mod part2;

use crate::{part1::part_one, part2::part_two};

fn main() {
    let input_one = include_str!("../input/input1.txt");
    println!("{}", part_one(input_one));
    println!("{}", part_two(input_one));
}
