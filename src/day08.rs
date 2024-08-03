use std::collections::HashSet;

pub fn day08(input_lines: &str) -> (String, String) {
    let bottom_right = input_lines.clone().chars().rev().collect::<String>();
    let tl_visible = top_left_visible(input_lines, true);
    println!("{:?}", tl_visible);
    let br_visible = top_left_visible(&bottom_right, false);
    let visible = tl_visible.union(&br_visible).collect::<Vec<&Tree>>();
    let answer1 = visible.len();
    let answer2 = 0;
    (format!("{}", answer1), format!("{}", answer2))
}

#[derive(Eq, Hash, PartialEq, Clone, Debug)]
struct Tree(usize, usize);

fn top_left_visible(woodland: &str, top_left_is_nw: bool) -> HashSet<Tree> {
    let mut woodland = woodland.lines().peekable();
    let column_count = woodland.peek().unwrap().len();
    let row_count: usize = woodland.clone().count();
    let mut top_maxes: Vec<i32> = vec![-1; column_count];
    let mut top_visible: Vec<HashSet<Tree>> = vec![HashSet::new(); column_count];
    let mut visible = HashSet::from_iter(
        woodland
            .enumerate()
            .map(|(row_idx, row)| {
                let mut left_max: i32 = -1;
                let row_left_visible = row
                    .chars()
                    .enumerate()
                    .filter_map(|(col_idx, cell)| {
                        let (x, y) = match top_left_is_nw {
                            true => (col_idx, row_idx),
                            false => (column_count - 1 - col_idx, row_count - 1 - row_idx),
                        };
                        let tree = Tree(x, y);
                        let mut visible = None;
                        let height = cell.to_string().parse::<i32>().unwrap();
                        if height > top_maxes[col_idx] {
                            top_maxes[col_idx] = height;
                            top_visible[col_idx].insert(tree.clone());
                            println!("Added ({}, -{}), height: {} to column", x, y, height);
                        }
                        if height > left_max {
                            left_max = height;
                            println!("Added ({}, -{}), height: {} to row", x, y, height);
                            visible = Some(tree)
                        }
                        visible
                    })
                    .collect::<Vec<Tree>>();
                row_left_visible.into_iter()
            })
            .flatten(),
    );
    for column in top_visible {
        visible.extend(column);
    }
    visible
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_day08_part1_case1() {
        assert_eq!(
            day08(
                "30373
25512
65332
33549
35390"
            )
            .0,
            "21".to_string()
        )
    }

    #[test]
    fn check_day08_part2_case1() {
        assert_eq!(
            day08(
                "30373
25512
65332
33549
35390"
            )
            .1,
            "0".to_string()
        )
    }

    #[test]
    fn check_day08_both_case1() {
        assert_eq!(
            day08(
                "30373
25512
65332
33549
35390"
            ),
            ("21".to_string(), "0".to_string())
        )
    }
}
