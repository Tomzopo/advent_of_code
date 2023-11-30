fn main() {
    let file = include_str!("../input/calories.txt");
    let mut elves_summed_calories: Vec<i64> = file
        .split("\r\n\r\n")
        .map(|elf_calories| {
            elf_calories
                .lines()
                .map(|calorie| calorie.parse::<i64>().unwrap())
                .sum()
        })
        .collect();

    println!(
        "Max Calories {}",
        elves_summed_calories.iter().max().unwrap()
    );

    elves_summed_calories.sort();
    println!(
        "Sum of top 3 Calories: {}",
        elves_summed_calories.iter().rev().take(3).sum::<i64>()
    );
}
