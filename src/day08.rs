use std::collections::HashSet;

pub fn day08(input_lines: &str) -> (String, String) {
    let (visible, trees) = woodland_walk(input_lines);
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

fn woodland_walk(woodland: &str) -> (HashSet<(usize, usize)>, Vec<Vec<Tree>>) {
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
    let mut column_maxes: Vec<i32> = vec![-1; column_count];
    let mut column_visible: Vec<HashSet<(usize, usize)>> = vec![HashSet::new(); column_count];

    let mut outside_visible =
        HashSet::from_iter(woodland.enumerate().flat_map(|(row_idx, row)| {
            let mut left_max: i32 = -1;
            let row_visible = row
                .chars()
                .enumerate()
                .map(|(col_idx, cell)| {
                    let (x, y) = (col_idx, row_idx);
                    let mut visible: HashSet<(usize, usize)> = HashSet::new();
                    let height = cell.to_string().parse::<i32>().unwrap();
                    let mut left_view = 0;
                    let mut up_view = 0;

                    // If our woodland is more than a single tree and we're not on the left/top, there must be at least one tree.
                    if x > 0 {
                        left_view += 1;
                    }
                    if y > 0 {
                        up_view += 1;
                    }
                    // Work out view of this tree to the left.
                    while left_view < x {
                        let left_tree = &trees[y][x - left_view];
                        if left_tree.height >= height {
                            break;
                        }
                        // We can definitely see over any tree this tree can see over.
                        left_view += left_tree.left_view;
                    }
                    // Update view right from previous trees. No trees left of our left_view could see us.
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

                    // Work out view ups from this tree. Same process as left view but change y coordinate.
                    while up_view < y {
                        let top_tree = &trees[y - up_view][x];
                        if top_tree.height >= height {
                            break;
                        }
                        // We can definitely see over any tree this tree can see over
                        up_view += top_tree.up_view;
                    }

                    // Update view down from previous trees. Same process as right view but change y coordinate.
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

                    let curr_tree = &mut trees[y][x];
                    curr_tree.height = height;
                    curr_tree.left_view = left_view;
                    curr_tree.right_view = 0;
                    curr_tree.up_view = up_view;
                    curr_tree.down_view = 0;

                    // Decide if this tree is visible from the top
                    if height > column_maxes[x] {
                        column_maxes[x] = height;
                        column_visible[x].insert((x, y));
                    }
                    // Decide if this tree is visible from the left
                    if height > left_max {
                        left_max = height;
                        visible.insert((x, y));
                    }

                    // Right edge. Clean-up right views of trees that can see out of the woodland. These are also the trees that
                    // are visible from the outside on the right. No need to go further than heighest tree.
                    if x == column_count - 1 {
                        // Outer tree always visible
                        visible.insert((x, y));
                        let mut left_tree_idx = x;
                        let mut left_highest = height;
                        while left_highest < left_max && left_tree_idx > 0 {
                            let left_tree = &mut trees[y][left_tree_idx];
                            if left_tree.height > left_highest {
                                left_highest = left_tree.height;
                                left_tree.right_view = x - left_tree_idx;
                                visible.insert((left_tree_idx, y));
                            }
                            left_tree_idx -= left_tree.left_view;
                        }
                    }

                    // Bottom edge, set outside visibility and downwards visibility as with right edge.
                    if y == row_count - 1 {
                        column_visible[col_idx].insert((x, y));
                        let mut up_tree_idx = y;
                        let mut up_highest = height;
                        while up_highest < column_maxes[x] && up_tree_idx > 0 {
                            let up_tree = &mut trees[up_tree_idx][x];
                            if up_tree.height > up_highest {
                                up_highest = up_tree.height;
                                up_tree.down_view = y - up_tree_idx;
                                column_visible[col_idx].insert((x, y));
                            }
                            up_tree_idx -= up_tree.up_view;
                        }
                    }

                    visible
                })
                .filter(|visible| !visible.is_empty())
                .flatten()
                // We force this so that we can use the nested enumeration
                .collect::<HashSet<(usize, usize)>>();
            row_visible.into_iter()
        }));
    for column in column_visible {
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
