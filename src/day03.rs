use itertools::Itertools;
use std::collections::HashMap;

const ITEM_TYPES: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

pub fn day03(input_lines: &str) -> (String, String) {
    let mut answer1 = 0;

    let mut priorities: HashMap<char, i32> = HashMap::new();
    for (priority, item_type) in ITEM_TYPES.chars().enumerate() {
        priorities.insert(item_type, priority as i32 + 1);
    }
    let priorities: HashMap<char, i32> = priorities;

    let answer2: i32 = input_lines
        .lines()
        .chunks(3)
        .into_iter()
        .map(|group| {
            let Some(elf_group): Option<(String, String, String)> = group
                .map(|line| {
                    let capacity = line.len();
                    let rucksack = Rucksack {
                        first_comp: &line[..capacity / 2],
                        second_comp: &line[capacity / 2..],
                    };
                    answer1 += prioritise_rucksack(rucksack, &priorities);
                    line.chars().unique().collect::<String>()
                })
                .collect_tuple()
            else {
                todo!()
            };
            elf_group
                .0
                .chars()
                .filter(|&c| elf_group.1.contains(c) && elf_group.2.contains(c))
                .collect::<Vec<char>>()
                .into_iter()
                .unique()
                .map(|item| priorities.get(&item).unwrap())
                .sum::<i32>()
        })
        .sum::<i32>();

    (format!("{}", answer1), format!("{}", answer2))
}

pub struct Rucksack<'a> {
    first_comp: &'a str,
    second_comp: &'a str,
}

pub fn prioritise_rucksack(rucksack: Rucksack, priorities: &HashMap<char, i32>) -> i32 {
    rucksack
        .first_comp
        .chars()
        .unique()
        .filter(|item| rucksack.second_comp.contains(*item))
        .map(|item| priorities.get(&item).unwrap())
        .sum::<i32>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_day03_part1_case1() {
        assert_eq!(
            day03(
                "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw"
            )
            .0,
            "157".to_string()
        )
    }

    #[test]
    fn check_day03_part2_case1() {
        assert_eq!(
            day03(
                "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw"
            )
            .1,
            "70".to_string()
        )
    }

    #[test]
    fn check_day03_both_case1() {
        assert_eq!(
            day03(
                "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw"
            ),
            ("157".to_string(), "70".to_string())
        )
    }
}
