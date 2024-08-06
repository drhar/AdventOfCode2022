use std::collections::HashSet;

use regex::Regex;

pub fn day09(input_lines: &str) -> (String, String) {
    let instruction_regex = Regex::new(r"([RLUD]) (\d+)").unwrap();
    let mut pt1_rope = vec![(0, 0); 2];
    let mut pt2_rope = vec![(0, 0); 10];
    let mut pt1_tail_positions: HashSet<(i32, i32)> = HashSet::new();
    let mut pt2_tail_positions: HashSet<(i32, i32)> = HashSet::new();

    for instruction in instruction_regex.captures_iter(input_lines) {
        let (direction, step_num) = (
            &instruction[1],
            instruction[2]
                .parse::<i32>()
                .unwrap_or_else(|_| panic!("Expected number, got {}", &instruction[2])),
        );
        for _step in 0..step_num {
            pt1_rope = rope_move(direction, pt1_rope);
            pt2_rope = rope_move(direction, pt2_rope);
            pt1_tail_positions.insert(pt1_rope[pt1_rope.len() - 1]);
            pt2_tail_positions.insert(pt2_rope[pt2_rope.len() - 1]);
        }
    }
    let answer1 = pt1_tail_positions.len();
    let answer2 = pt2_tail_positions.len();
    (format!("{}", answer1), format!("{}", answer2))
}

pub fn rope_move(direction: &str, mut rope: Vec<(i32, i32)>) -> Vec<(i32, i32)> {
    rope[0] = head_move(direction, rope[0]);
    for pair in 0..rope.len() - 1 {
        let head = rope[pair];
        let tail = rope[pair + 1];
        let (x, y) = knot_pair_tail_move((head.0 - tail.0, head.1 - tail.1));
        rope[pair + 1] = (tail.0 + x, tail.1 + y);
    }
    rope
}

pub fn head_move(direction: &str, head: (i32, i32)) -> (i32, i32) {
    match direction {
        "R" => (head.0 + 1, head.1),
        "L" => (head.0 - 1, head.1),
        "U" => (head.0, head.1 + 1),
        "D" => (head.0, head.1 - 1),
        _ => panic!("Unknown direction {}", direction),
    }
}

pub fn knot_pair_tail_move(head_tail_vector: (i32, i32)) -> (i32, i32) {
    let (x, y) = head_tail_vector;
    match (x.abs(), y.abs()) {
        (0, 0) | (0, 1) | (1, 0) | (1, 1) => (0, 0),
        (2, _) | (_, 2) => {
            let mut x_mv = 0;
            let mut y_mv = 0;
            if x.abs() != 0 {
                x_mv = x.signum();
            }
            if y.abs() != 0 {
                y_mv = y.signum();
            }
            (x_mv, y_mv)
        }
        _ => panic!("Vector {:?} too long, snap!!", head_tail_vector),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_day09_part1_case1() {
        assert_eq!(
            day09(
                "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2"
            )
            .0,
            "13".to_string()
        )
    }

    #[test]
    fn check_day09_part2_case1() {
        assert_eq!(
            day09(
                "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2"
            )
            .1,
            "1".to_string()
        )
    }

    #[test]
    fn check_day09_part2_case2() {
        assert_eq!(
            day09(
                "R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20"
            )
            .1,
            "36".to_string()
        )
    }

    #[test]
    fn check_day09_both_case1() {
        assert_eq!(
            day09(
                "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2"
            ),
            ("13".to_string(), "1".to_string())
        )
    }
}
