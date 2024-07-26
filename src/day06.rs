use itertools::Itertools;

pub fn day06(input_lines: &str) -> (String, String) {
    let answer1 = find_marker(input_lines, 4);
    let answer2 = find_marker(input_lines, 14);
    (format!("{}", answer1), format!("{}", answer2))
}

pub fn find_marker(message: &str, marker_len: usize) -> usize {
    let (index, _marker) = message
        .chars()
        .collect::<Vec<char>>()
        .windows(marker_len)
        .enumerate()
        .take_while(|w| w.1.into_iter().unique().count() != marker_len)
        .last()
        .unwrap();
    // Return postiion after _last_ char. Iterator is 0 indexed.
    index + 1 + marker_len
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_day06_part1_case1() {
        assert_eq!(day06("bvwbjplbgvbhsrlpgdmjqwftvncz").0, "5".to_string())
    }
    #[test]
    fn check_day06_part1_case2() {
        assert_eq!(day06("nppdvjthqldpwncqszvftbrmjlhg").0, "6".to_string())
    }
    #[test]
    fn check_day06_part1_case3() {
        assert_eq!(
            day06("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg").0,
            "10".to_string()
        )
    }
    #[test]
    fn check_day06_part1_case4() {
        assert_eq!(
            day06("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw").0,
            "11".to_string()
        )
    }
    #[test]
    fn check_day06_part2_case1() {
        assert_eq!(day06("mjqjpqmgbljsphdztnvjfqwrcgsmlb").1, "19".to_string())
    }
    #[test]
    fn check_day06_part2_case2() {
        assert_eq!(day06("bvwbjplbgvbhsrlpgdmjqwftvncz").1, "23".to_string())
    }
    #[test]
    fn check_day06_part2_case3() {
        assert_eq!(day06("nppdvjthqldpwncqszvftbrmjlhg").1, "23".to_string())
    }
    #[test]
    fn check_day06_part2_case4() {
        assert_eq!(
            day06("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg").1,
            "29".to_string()
        )
    }
    #[test]
    fn check_day06_part2_case5() {
        assert_eq!(
            day06("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw").1,
            "26".to_string()
        )
    }

    #[test]
    fn check_day06_both_case1() {
        assert_eq!(
            day06("bvwbjplbgvbhsrlpgdmjqwftvncz"),
            ("5".to_string(), "23".to_string())
        )
    }
}
