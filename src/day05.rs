use std::collections::HashMap;

pub fn day05(input_lines: &str) -> (String, String) {
    let (crate_diag, instructions) = input_lines.split_once("\n\n").unwrap();
    let crate_column_labels = crate_diag.lines().last().unwrap();

    let mut crates: HashMap<usize, Vec<char>> = HashMap::new();
    for line in crate_diag.lines().rev() {
        if line == crate_column_labels {
            continue;
        }
        for (curr_crate, column) in line.chars().zip(crate_column_labels.chars()) {
            if curr_crate.is_alphabetic() {
                let stack = crates
                    .entry(column as usize - '0' as usize)
                    .or_insert_with(Vec::new);
                stack.push(curr_crate);
            }
        }
    }
    let mut crates2 = crates.clone();
    let re = regex::Regex::new(r"move (\d*) from (\d*) to (\d*)").unwrap();
    for instruction in re.captures_iter(instructions) {
        let (quantity, from, to) = (
            instruction[1].parse::<usize>().unwrap(),
            instruction[2].parse::<usize>().unwrap(),
            instruction[3].parse::<usize>().unwrap(),
        );
        crates = crate_mover_9000(crates, quantity, from, to);
        crates2 = crate_mover_9001(crates2, quantity, from, to)
    }
    // crate_diag
    //     .lines()
    //     .map(|line| line.chars().filter(|c| c.is_alphabetic()).enumerate());
    let answer1 = read_diag(crates);
    let answer2 = read_diag(crates2);
    (format!("{}", answer1), format!("{}", answer2))
}

pub fn crate_mover(
    mut crates: HashMap<usize, Vec<char>>,
    quantity: usize,
    from: usize,
    to: usize,
    multi_crate_moves: bool,
) -> HashMap<usize, Vec<char>> {
    let new_stack_height = crates.get(&from).unwrap().len() - quantity;
    let mut transit = crates.get_mut(&from).unwrap().split_off(new_stack_height);
    if !multi_crate_moves {
        transit = transit.into_iter().rev().collect::<Vec<char>>();
    }

    crates.get_mut(&to).unwrap().extend(transit);

    crates
}

pub fn crate_mover_9000(
    mut crates: HashMap<usize, Vec<char>>,
    quantity: usize,
    from: usize,
    to: usize,
) -> HashMap<usize, Vec<char>> {
    crate_mover(crates, quantity, from, to, false)
}

pub fn crate_mover_9001(
    mut crates: HashMap<usize, Vec<char>>,
    quantity: usize,
    from: usize,
    to: usize,
) -> HashMap<usize, Vec<char>> {
    crate_mover(crates, quantity, from, to, true)
}

pub fn read_diag(diag: HashMap<usize, Vec<char>>) -> String {
    let mut diag_summary = String::new();
    for column in 1 as usize..*diag.keys().max().unwrap() + 1 {
        diag_summary.push(*diag.get(&column).unwrap().last().unwrap());
    }
    diag_summary
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_day05_part1_case1() {
        assert_eq!(
            day05(
                "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2"
            )
            .0,
            "CMZ".to_string()
        )
    }

    #[test]
    fn check_day05_part2_case1() {
        assert_eq!(day05("").1, "0".to_string())
    }

    #[test]
    fn check_day05_both_case1() {
        assert_eq!(day05(""), ("0".to_string(), "0".to_string()))
    }
}
