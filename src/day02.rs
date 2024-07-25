pub fn day02(input_lines: &str) -> (String, String) {
    let mut answer1 = 0;
    let mut answer2 = 0;
    input_lines
        .lines()
        .map(|line| {
            let mut iter = line.split_whitespace();
            let opponent = iter.next().unwrap();
            let player = iter.next().unwrap();
            match player {
                "X" => {
                    answer1 += 1 + rock_fight(opponent);
                    answer2 += lose_fight(opponent);
                }
                "Y" => {
                    answer1 += 2 + paper_fight(opponent);
                    answer2 += 3 + draw_fight(opponent);
                }
                "Z" => {
                    answer1 += 3 + scissor_fight(opponent);
                    answer2 += 6 + win_fight(opponent);
                }
                _ => panic!("Invalid move by player! {player}"),
            }
        })
        .for_each(drop);
    (format!("{}", answer1), format!("{}", answer2))
}

pub fn rock_fight(opp: &str) -> i32 {
    match opp {
        "A" => 3,
        "B" => 0,
        "C" => 6,
        _ => panic!("Invalid move by opponent! {opp}"),
    }
}

pub fn paper_fight(opp: &str) -> i32 {
    match opp {
        "A" => 6,
        "B" => 3,
        "C" => 0,
        _ => panic!("Invalid move by opponent! {opp}"),
    }
}

pub fn scissor_fight(opp: &str) -> i32 {
    match opp {
        "A" => 0,
        "B" => 6,
        "C" => 3,
        _ => panic!("Invalid move by opponent! {opp}"),
    }
}

pub fn lose_fight(opp: &str) -> i32 {
    match opp {
        "A" => 3,
        "B" => 1,
        "C" => 2,
        _ => panic!("Invalid move by opponent! {opp}"),
    }
}

pub fn draw_fight(opp: &str) -> i32 {
    match opp {
        "A" => 1,
        "B" => 2,
        "C" => 3,
        _ => panic!("Invalid move by opponent! {opp}"),
    }
}

pub fn win_fight(opp: &str) -> i32 {
    match opp {
        "A" => 2,
        "B" => 3,
        "C" => 1,
        _ => panic!("Invalid move by opponent! {opp}"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_day02_part1_case1() {
        assert_eq!(day02("A Y").0, "8".to_string())
    }

    #[test]
    fn check_day02_part2_case1() {
        assert_eq!(day02("B X").1, "1".to_string())
    }

    #[test]
    fn check_day02_both_case1() {
        assert_eq!(day02("C Z"), ("6".to_string(), "7".to_string()))
    }
}
