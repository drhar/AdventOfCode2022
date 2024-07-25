pub fn day01(input_lines: &str) -> (String, String) {
    let elf_lines = input_lines.split("\n\n");
    let mut elves: Vec<i32> = vec![];
    for elf in elf_lines {
        elves.push(elf.lines().map(parse_elf).sum());
    }
    elves.sort();

    let answer1 = elves.last().unwrap();
    let answer2 = elves[elves.len() - 3] + elves[elves.len() - 2] + elves[elves.len() - 1];
    (format!("{}", answer1), format!("{}", answer2))
}

pub fn parse_elf(elf: &str) -> i32 {
    elf.parse::<i32>().expect(&format!("Expected number for {}", elf))
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_day01_part1_case1() {
        assert_eq!(day01("1000
2000
3000

4000

5000
6000

7000
8000
9000

10000").0, "24000".to_string())
    }

    #[test]
    fn check_day01_part2_case1() {
        assert_eq!(day01("1000
2000
3000

4000

5000
6000

7000
8000
9000

10000").1, "45000".to_string())
    }

    #[test]
    fn check_day01_both_case1() {
        assert_eq!(day01("1000
2000
3000

4000

5000
6000

7000
8000
9000

10000"), ("24000".to_string(), "45000".to_string()))
    }
}
