use std::collections::HashSet;

const MAX_TREE_HEIGHT: i32 = 9;

pub fn day08(input_lines: &str) -> (String, String) {
    let bottom_right = input_lines.chars().rev().collect::<String>();
    let (tl_visible, trees) = top_left_woodland_walk(input_lines, true);
    println!("{:?}", tl_visible);
    let (br_visible, _) = top_left_woodland_walk(&bottom_right, false);
    let visible = tl_visible
        .union(&br_visible)
        .collect::<Vec<&(usize, usize)>>();
    let answer1 = visible.len();

    // Debug prints
    // for (i, row) in trees.iter().enumerate() {
    //     for (j, tree) in row.iter().enumerate() {
    //         println!(
    //             "({}, {}) is {:?}, score {}",
    //             j,
    //             i,
    //             tree,
    //             (tree.left_view * tree.right_view * tree.up_view * tree.down_view)
    //         );
    //     }
    // }

    let answer2 = trees
        .iter()
        .flatten()
        .map(|tree| tree.left_view * tree.right_view * tree.up_view * tree.down_view)
        .max()
        .unwrap();
    (format!("{}", answer1), format!("{}", answer2))
}

#[derive(Clone, Debug)]
struct Tree {
    height: i32,
    left_view: usize,
    right_view: usize,
    up_view: usize,
    down_view: usize,
}

fn top_left_woodland_walk(
    woodland: &str,
    top_left_is_nw: bool,
) -> (HashSet<(usize, usize)>, Vec<Vec<Tree>>) {
    let mut woodland = woodland.lines().peekable();
    let column_count = woodland.peek().unwrap().len();
    let row_count: usize = woodland.clone().count();
    let mut trees: Vec<Vec<Tree>> = vec![
        vec![
            Tree {
                height: 0,
                left_view: 0,
                right_view: 0,
                up_view: 0,
                down_view: 0
            };
            column_count
        ];
        row_count
    ];
    let mut top_maxes: Vec<i32> = vec![-1; column_count];
    let mut bottom_maxes: Vec<i32> = vec![-1; column_count];
    let mut top_visible: Vec<HashSet<(usize, usize)>> = vec![HashSet::new(); column_count];

    let mut outside_visible =
        HashSet::from_iter(woodland.enumerate().flat_map(|(row_idx, row)| {
            let mut left_max: i32 = -1;
            let mut right_max: i32 = -1;
            let row_left_visible = row
                .chars()
                .enumerate()
                .filter_map(|(col_idx, cell)| {
                    let (x_cd, y_cd) = match top_left_is_nw {
                        true => (col_idx, row_idx),
                        false => (column_count - 1 - col_idx, row_count - 1 - row_idx),
                    };
                    let tree_coords = (x_cd, y_cd);
                    let mut visible = None;
                    let height = cell.to_string().parse::<i32>().unwrap();
                    let mut left_view = 0;
                    let mut up_view = 0;

                    // If our woodland is more than a single tree and we're not on the left/top, there must be at least one tree.
                    let x = col_idx;
                    let y = row_idx;
                    if x > 0 {
                        left_view += 1;
                    }
                    if y > 0 {
                        up_view += 1;
                    }
                    // Work out left view of tree
                    while left_view < x {
                        let left_tree = &trees[y][x - left_view];
                        if left_tree.height >= height {
                            break;
                        }
                        // We can definitely see over any tree this tree can see over
                        left_view += left_tree.left_view;
                    }
                    // Update right view of previous trees. No trees left of our left_view could see us.
                    let mut left_tree_idx = x - left_view;
                    while left_tree_idx < x {
                        let left_tree = &mut trees[y][left_tree_idx];
                        // We're to the right of this tree, so if it has no right view it's because it can see at least to us. If
                        // we're at the end of the row then this tree can see the whole way to the end.
                        if left_tree.right_view == 0 {
                            if left_tree.height <= height {
                                left_tree.right_view = x - left_tree_idx;
                            }
                            left_tree_idx += 1;
                        } else {
                            // We know how far to the right this tree can see, which means everything in it's right_view also already
                            // has a set right_view so we can skip them.
                            left_tree_idx += left_tree.right_view;
                        }
                    }
                    // Right edge clean-up, any trees taller than us and to our left can see the whole way.
                    // No need to go further than heighest tree.
                    if x == column_count - 1 {
                        let mut left_tree_idx = x - left_view;
                        let mut left_tree = &mut trees[y][left_tree_idx];
                        let mut left_highest = left_tree.height;
                        left_tree.right_view = left_view;
                        while left_highest < MAX_TREE_HEIGHT && left_tree_idx > 0 {
                            if left_tree.height > left_highest {
                                left_highest = left_tree.height;
                                left_tree.right_view = x - left_tree_idx;
                            }
                            left_tree_idx -= left_tree.left_view;
                            left_tree = &mut trees[y][left_tree_idx];
                        }
                    }

                    // Work out view down from this tree. Same process as left view but change y coordinate.
                    while up_view < y {
                        let top_tree = &trees[y - up_view][x];
                        if top_tree.height >= height {
                            break;
                        }
                        // We can definitely see over any tree this tree can see over
                        up_view += top_tree.up_view;
                    }

                    // Update down view of previous trees. Same process as right view but change y coordinate.
                    let mut top_tree_idx = y - up_view;
                    while top_tree_idx < y {
                        let top_tree = &mut trees[top_tree_idx][x];
                        // We're below this tree, so if it has no down view it's because it can see at least to us
                        if top_tree.down_view == 0 {
                            if top_tree.height <= height {
                                top_tree.down_view = y - top_tree_idx;
                            }
                            top_tree_idx += 1;
                        } else {
                            // We know how far to the bottom this tree can see, which means everything in it's down_view
                            // also alreadys as a set down_view so we can skip them.
                            top_tree_idx += top_tree.down_view;
                        }
                    }

                    // Bottom edge clean-up, any trees taller than us and above us can see the whole way.
                    // No need to go further than heighest tree.
                    if y == row_count - 1 {
                        let mut up_tree_idx = y - up_view;
                        let mut up_tree = &mut trees[up_tree_idx][x];
                        let mut up_highest = up_tree.height;
                        up_tree.down_view = up_view;
                        while up_highest < MAX_TREE_HEIGHT && up_tree_idx > 0 {
                            if up_tree.height > up_highest {
                                up_highest = up_tree.height;
                                up_tree.down_view = y - up_tree_idx;
                            }
                            up_tree_idx -= up_tree.up_view;
                            up_tree = &mut trees[up_tree_idx][x];
                        }
                    }

                    let curr_tree = &mut trees[y][x];
                    curr_tree.height = height;
                    curr_tree.left_view = left_view;
                    curr_tree.right_view = 0;
                    curr_tree.up_view = up_view;
                    curr_tree.down_view = 0;

                    // Decide if this tree is visible from the top
                    if height > top_maxes[col_idx] {
                        top_maxes[col_idx] = height;
                        top_visible[col_idx].insert(tree_coords);
                    }
                    // Decide if this tree is visible from the left
                    if height > left_max {
                        left_max = height;
                        visible = Some(tree_coords)
                    }
                    visible
                })
                .collect::<Vec<(usize, usize)>>();
            row_left_visible.into_iter()
        }));
    for column in top_visible {
        outside_visible.extend(column);
    }
    (outside_visible, trees)
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
            "8".to_string()
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
            ("21".to_string(), "8".to_string())
        )
    }
}
