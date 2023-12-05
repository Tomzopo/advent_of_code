use itertools::Itertools;

trait AlmanacMap {
    fn get_mapping(&self, input: i64) -> Option<i64>;
}

#[derive(Debug)]
struct Map {
    destination_start: i64,
    source_start: i64,
    range: i64,
}

impl AlmanacMap for Map {
    fn get_mapping(&self, input: i64) -> Option<i64> {
        if self.source_start <= input && input < self.source_start + self.range {
            Some(input - self.source_start + self.destination_start)
        } else {
            None
        }
    }
}

#[derive(Debug)]
struct CategoryMap {
    maps: Vec<Map>,
}

impl AlmanacMap for CategoryMap {
    fn get_mapping(&self, input: i64) -> Option<i64> {
        self.maps.iter().find_map(|map| map.get_mapping(input))
    }
}

#[derive(Debug)]
struct GardenMap {
    block_maps: Vec<CategoryMap>,
}

impl AlmanacMap for GardenMap {
    fn get_mapping(&self, input: i64) -> Option<i64> {
        let mut path: Option<i64> = Some(input);
        self.block_maps.iter().for_each(|block_map| {
            if let Some(num) = block_map.get_mapping(path.unwrap()) {
                path = Some(num)
            }
        });
        path
    }
}

pub fn part_one(input: &str) -> i64 {
    let mut inputs = input.split("\n\n");
    let seeds = parse_seeds(inputs.next());
    let garden = parse_garden(inputs);

    *seeds
        .iter()
        .map(|seed| garden.get_mapping(*seed).expect("Broken Pathing"))
        .collect::<Vec<i64>>()
        .iter()
        .min()
        .unwrap()
}

fn parse_seeds(input: Option<&str>) -> Vec<i64> {
    input
        .unwrap()
        .split_whitespace()
        .skip(1)
        .map(|inp| inp.parse::<i64>().unwrap())
        .collect()
}

fn parse_garden(input: core::str::Split<&str>) -> GardenMap {
    let block_maps: Vec<CategoryMap> = input
        .map(|block| {
            let maps = block
                .lines()
                .skip(1)
                .map(|map| {
                    let (destination_start, source_start, range) = map
                        .split_whitespace()
                        .map(|num| num.parse::<i64>().expect("Failed to convert value to i64"))
                        .collect_tuple()
                        .expect("Invalid amount of numbers for map");

                    Map {
                        destination_start,
                        source_start,
                        range,
                    }
                })
                .collect::<Vec<Map>>();

            CategoryMap { maps }
        })
        .collect();

    GardenMap { block_maps }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one_test() {
        struct TestCase {
            input: &'static str,
            expected: i64,
        }

        let test_cases = vec![
            TestCase {
                input: r#"seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4"#,
                expected: 35,
            },
            TestCase {
                input: include_str!("../input/input1.txt"),
                expected: 278755257,
            },
        ];

        for test_case in test_cases {
            assert_eq!(part_one(test_case.input), test_case.expected);
        }
    }
}
