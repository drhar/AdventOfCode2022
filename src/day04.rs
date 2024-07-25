use std::collections::HashSet;

pub fn day04(input_lines: &str) -> (String, String) {
    let answer1 = input_lines
        .lines()
        .map(|line| {
            let (elf1, elf2) = line
                .split_once(',')
                .unwrap_or_else(|| panic!("{} Should be a comma separated pair", line));
            let (elf1, elf2) = (parse_assignment(elf1), parse_assignment(elf2));
            if full_overlap(elf1, elf2) {
                1
            } else {
                0
            }
        })
        .sum::<i32>();
    let answer2 = input_lines
        .lines()
        .map(|line| {
            let (elf1, elf2) = line
                .split_once(',')
                .unwrap_or_else(|| panic!("{} Should be a comma separated pair", line));
            let (elf1, elf2) = (parse_assignment(elf1), parse_assignment(elf2));
            if !elf1.intersection(&elf2).collect::<HashSet<_>>().is_empty() {
                1
            } else {
                0
            }
        })
        .sum::<i32>();
    (format!("{}", answer1), format!("{}", answer2))
}

pub fn parse_assignment(assignment: &str) -> HashSet<i32> {
    let (start, end) = assignment.split_once('-').unwrap_or_else(|| {
        panic!(
            "Expected a '-' separated pair of intergers for {}",
            assignment
        )
    });
    let start = start
        .parse::<i32>()
        .unwrap_or_else(|_| panic!("Expected number not {}", start));
    let end = end
        .parse::<i32>()
        .unwrap_or_else(|_| panic!("Expected number not {}", end));
    (start..=end).collect::<HashSet<i32>>()
}

pub fn full_overlap(assignment1: HashSet<i32>, assignment2: HashSet<i32>) -> bool {
    let overlap: HashSet<_> = assignment1.intersection(&assignment2).collect();
    overlap.len() == assignment1.len() || overlap.len() == assignment2.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_day04_part1_case1() {
        assert_eq!(
            day04(
                "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8"
            )
            .0,
            "2".to_string()
        )
    }

    #[test]
    fn check_day04_part2_case1() {
        assert_eq!(day04("").1, "0".to_string())
    }

    #[test]
    fn check_day04_both_case1() {
        assert_eq!(
            day04(
                "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8"
            ),
            ("2".to_string(), "4".to_string())
        )
    }
}
