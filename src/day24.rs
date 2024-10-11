use std::collections::{HashMap, VecDeque};

pub fn day24(input: &str) -> (String, String) {
    day24_main(input, false)
}

pub fn day24_main(input_lines: &str, debug: bool) -> (String, String) {
    let mut valley = Valley::from_str(input_lines);
    let entrance = valley.get_entrance();
    let exit = valley.get_exit();
    let answer1 = fastest_traverse(&mut valley, entrance, exit, debug);

    valley = valley.predict_state(answer1);
    let time_back_to_start = fastest_traverse(&mut valley, exit, entrance, debug);

    valley = valley.predict_state(time_back_to_start);
    let answer2 =
        answer1 + time_back_to_start + fastest_traverse(&mut valley, entrance, exit, debug);
    (format!("{}", answer1), format!("{}", answer2))
}

#[derive(Clone, Debug, Hash, Eq, PartialEq)]
pub enum Blizzard {
    North,
    South,
    East,
    West,
}

#[derive(Clone, Debug, Hash, Eq, PartialEq)]
pub enum GridPosition {
    Empty,
    Wall,
    Blizzards(VecDeque<Blizzard>),
}

#[derive(Clone, PartialEq)]
pub struct Valley {
    map: Vec<Vec<GridPosition>>,
    blizzard_cycle_length: Option<usize>,
    blizzard_cycle_start: Option<usize>,
    entrance: (usize, usize),
    exit: (usize, usize),
}

impl Valley {
    pub fn from_str(input_str: &str) -> Self {
        let num_rows = input_str.lines().count();
        let mut expedition = (0, 0);
        let mut exit = (0, 0);
        let starting_map = input_str
            .lines()
            .enumerate()
            .map(|(y, line)| {
                line.chars()
                    .enumerate()
                    .map(|(x, c)| match c {
                        '#' => GridPosition::Wall,
                        '.' => {
                            if y == 0 {
                                expedition = (x, y)
                            } else if y == num_rows - 1 {
                                exit = (x, y)
                            }
                            GridPosition::Empty
                        }
                        '>' => GridPosition::Blizzards(VecDeque::from([Blizzard::East])),
                        '<' => GridPosition::Blizzards(VecDeque::from([Blizzard::West])),
                        '^' => GridPosition::Blizzards(VecDeque::from([Blizzard::North])),
                        'v' => GridPosition::Blizzards(VecDeque::from([Blizzard::South])),
                        _ => panic!("Invalid character in input"),
                    })
                    .collect()
            })
            .collect();
        Self {
            map: starting_map,
            blizzard_cycle_length: None,
            blizzard_cycle_start: None,
            entrance: expedition,
            exit,
        }
    }

    pub fn get_entrance(&self) -> (usize, usize) {
        self.entrance
    }

    pub fn get_exit(&self) -> (usize, usize) {
        self.exit
    }

    pub fn find_blizzard_cycles(&mut self) -> (usize, usize) {
        if self.blizzard_cycle_length.is_some() {
            return (
                self.blizzard_cycle_start.unwrap(),
                self.blizzard_cycle_length.unwrap(),
            );
        }
        let mut tortoise = self.predict_state(0);
        let mut hare = self.predict_state(1);
        let mut two_power = 0;
        let mut cycle_length = 1;
        while tortoise.map != hare.map {
            if two_power == cycle_length {
                tortoise = hare.clone();
                two_power *= 2;
                cycle_length = 0;
            }
            hare = hare.predict_state(1);
            cycle_length += 1;
        }
        self.blizzard_cycle_length = Some(cycle_length);
        let mut hare = self.predict_state(0);
        tortoise = self.predict_state(0);
        hare = hare.predict_state(cycle_length);
        let mut cycle_start = 0;
        while tortoise.predict_state(cycle_start) != hare.predict_state(cycle_start) {
            cycle_start += 1;
        }
        self.blizzard_cycle_start = Some(cycle_start);
        (
            self.blizzard_cycle_start.unwrap(),
            self.blizzard_cycle_length.unwrap(),
        )
    }

    pub fn predict_state(&self, mut minute: usize) -> Self {
        if let Some(cycle_start) = self.blizzard_cycle_start {
            if minute > cycle_start {
                let sig_minutes = (minute - cycle_start) % self.blizzard_cycle_length.unwrap();
                minute = cycle_start + sig_minutes;
            }
        }
        let mut new_map = self.map.clone();
        let map_len = self.map.len();
        let map_width = self.map[0].len();

        // We iterate over the old, immutable map to ensure we don't move a blizzard twice
        for (y, row) in self.map.iter().enumerate() {
            for (x, cell) in row.iter().enumerate() {
                if let GridPosition::Blizzards(ref blizzards) = cell {
                    for blizzard in blizzards {
                        //Find where this blizzard will be
                        let empty_space_x = map_width - 2;
                        let first_x = 1;
                        let last_x = map_width - 2;
                        // There are up to two special case columns with more empty space
                        let mut empty_space_y = map_len - 2;
                        let mut first_y = 1;
                        let mut last_y = map_len - 2;
                        if !matches!(self.map[0][x], GridPosition::Wall) {
                            empty_space_y += 1;
                            first_y = 0;
                        }
                        if !matches!(self.map[map_len - 1][x], GridPosition::Wall) {
                            empty_space_y += 1;
                            last_y = map_len - 1;
                        }
                        let (new_x, new_y) = match blizzard {
                            Blizzard::North => {
                                let new_y = y as isize - (minute % empty_space_y) as isize;
                                if new_y < first_y as isize {
                                    (x, (new_y + empty_space_y as isize) as usize)
                                } else {
                                    (x, new_y as usize)
                                }
                            }
                            Blizzard::South => {
                                let new_y = y + (minute % empty_space_y);
                                if new_y > last_y {
                                    (x, new_y % empty_space_y)
                                } else {
                                    (x, new_y)
                                }
                            }
                            Blizzard::East => {
                                let new_x = x + (minute % empty_space_x);
                                if new_x > last_x {
                                    (new_x % empty_space_x, y)
                                } else {
                                    (new_x, y)
                                }
                            }
                            Blizzard::West => {
                                let new_x = x as isize - (minute % empty_space_x) as isize;
                                if new_x < first_x as isize {
                                    ((new_x + empty_space_x as isize) as usize, y)
                                } else {
                                    (new_x as usize, y)
                                }
                            }
                        };

                        // Move the blizzard
                        let in_flight =
                            if let GridPosition::Blizzards(ref mut blizzards) = new_map[y][x] {
                                let blizzard = blizzards.pop_front().unwrap();
                                if blizzards.is_empty() {
                                    new_map[y][x] = GridPosition::Empty;
                                }
                                blizzard
                            } else {
                                panic!("No blizzard in starting position ({}, {})", x, y);
                            };
                        if let GridPosition::Blizzards(ref mut blizzards) = new_map[new_y][new_x] {
                            blizzards.push_back(in_flight);
                        } else if matches!(new_map[new_y][new_x], GridPosition::Empty) {
                            new_map[new_y][new_x] =
                                GridPosition::Blizzards(VecDeque::from([in_flight]));
                        } else {
                            panic!(
                                "Blizzard moved to a wall from ({}, {}) to ({}, {}) in minute {}",
                                x, y, new_x, new_y, minute
                            );
                        }
                    }
                }
            }
        }
        let cycle_start = if let Some(start) = self.blizzard_cycle_start {
            if start <= minute {
                Some(0)
            } else {
                Some(start - minute)
            }
        } else {
            None
        };
        Self {
            map: new_map,
            blizzard_cycle_length: self.blizzard_cycle_length,
            blizzard_cycle_start: cycle_start,
            entrance: self.entrance,
            exit: self.exit,
        }
    }

    pub fn get_possible_actions(&self, expedition_start: (usize, usize)) -> Vec<(usize, usize)> {
        let mut actions = Vec::new();
        let (x, y) = expedition_start;
        if matches!(self.map[y][x], GridPosition::Empty) {
            actions.push((x, y));
        }
        if y > 0 && matches!(self.map[y - 1][x], GridPosition::Empty) {
            actions.push((x, y - 1));
        }
        if y < self.map.len() - 1 && matches!(self.map[y + 1][x], GridPosition::Empty) {
            actions.push((x, y + 1));
        }
        if x > 0 && matches!(self.map[y][x - 1], GridPosition::Empty) {
            actions.push((x - 1, y));
        }
        if x < self.map[0].len() - 1 && matches!(self.map[y][x + 1], GridPosition::Empty) {
            actions.push((x + 1, y));
        }
        actions
    }

    pub fn print(&self, expedition: (usize, usize)) {
        let mut y = 0;
        let mut x = 0;
        for row in &self.map {
            for cell in row {
                if (x, y) == expedition {
                    print!("E");
                } else {
                    match cell {
                        GridPosition::Empty => print!("."),
                        GridPosition::Wall => print!("#"),
                        GridPosition::Blizzards(bs) => {
                            if bs.len() == 1 {
                                match bs[0] {
                                    Blizzard::North => print!("^"),
                                    Blizzard::South => print!("v"),
                                    Blizzard::East => print!(">"),
                                    Blizzard::West => print!("<"),
                                }
                            } else {
                                print!("{}", bs.len());
                            }
                        }
                    }
                }
                x += 1;
            }
            println!();
            y += 1;
            x = 0;
        }
        println!();
    }
}

pub fn fastest_traverse(
    starting_valley: &mut Valley,
    start: (usize, usize),
    target: (usize, usize),
    debug: bool,
) -> usize {
    let (cycle_start, cycle_length) = starting_valley.find_blizzard_cycles();
    let mut best_visits: HashMap<(usize, usize, usize), usize> = HashMap::new();
    if !debug {
        let mut queue = VecDeque::from([(0, start)]);
        while let Some((mut minutes, (x, y))) = queue.pop_front() {
            if !worth_visiting(
                (cycle_start, cycle_length),
                (x, y),
                minutes,
                &mut best_visits,
            ) {
                continue;
            }
            if (x, y) == target {
                return minutes;
            }
            minutes += 1;
            let valley = starting_valley.predict_state(minutes);
            let mut possible_actions = valley
                .get_possible_actions((x, y))
                .iter()
                .map(|(x, y)| (minutes, (*x, *y)))
                .collect::<VecDeque<(usize, (usize, usize))>>();
            queue.append(&mut possible_actions);
        }
        panic!("No path found");
    } else {
        let mut count = 0;
        let mut queue = VecDeque::from([(0, vec![start])]);
        while let Some((mut minutes, path_vec)) = queue.pop_front() {
            if count % 10000 == 0 {
                println!(
                    "Count: {}, Minutes: {}, Queue length: {}",
                    count,
                    minutes,
                    queue.len()
                );
            }
            count += 1;
            let (x, y) = path_vec.last().unwrap().clone();
            if !worth_visiting(
                (cycle_start, cycle_length),
                (x, y),
                minutes,
                &mut best_visits,
            ) {
                continue;
            }
            if (x, y) == target {
                println!("\nFound path in {} minutes: {:?}", minutes, path_vec);
                for minute in 0..path_vec.len() {
                    starting_valley
                        .predict_state(minute)
                        .print(path_vec[minute]);
                }
                return minutes;
            }
            minutes += 1;
            let valley = starting_valley.predict_state(minutes);
            let mut possible_actions = valley
                .get_possible_actions((x, y))
                .iter()
                .map(|(x, y)| {
                    let mut path = path_vec.clone();
                    path.push((*x, *y));
                    (minutes, path)
                })
                .collect::<VecDeque<(usize, Vec<(usize, usize)>)>>();
            queue.append(&mut possible_actions);
        }
        panic!("No path found");
    }
}

pub fn worth_visiting(
    cycles: (usize, usize),
    postion: (usize, usize),
    minutes: usize,
    best_visits: &mut HashMap<(usize, usize, usize), usize>,
) -> bool {
    let (cycle_start, cycle_length) = cycles;
    let (x, y) = postion;
    if minutes >= cycle_start {
        let sig_minutes = (minutes - cycle_start) % cycle_length;
        if let Some(best_minutes) = best_visits.get_mut(&(x, y, sig_minutes)) {
            if *best_minutes <= minutes {
                return false;
            } else {
                *best_minutes = minutes;
            }
        } else {
            best_visits.insert((x, y, sig_minutes), minutes);
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_day24_part1_case1() {
        assert_eq!(
            day24_main(
                "#.######
#>>.<^<#
#.<..<<#
#>v.><>#
#<^v^^>#
######.#",
                true
            )
            .0,
            "18".to_string()
        )
    }

    #[test]
    fn check_day24_part2_case1() {
        assert_eq!(
            day24_main(
                "#.######
#>>.<^<#
#.<..<<#
#>v.><>#
#<^v^^>#
######.#",
                true
            )
            .1,
            "54".to_string()
        )
    }

    #[test]
    fn check_day24_both_case1() {
        assert_eq!(
            day24_main(
                "#.######
#>>.<^<#
#.<..<<#
#>v.><>#
#<^v^^>#
######.#",
                true
            ),
            ("18".to_string(), "54".to_string())
        )
    }

    #[test]
    fn check_day24_prod_debug() {
        assert_eq!(
            day24_main(
                "#.######
#>>.<^<#
#.<..<<#
#>v.><>#
#<^v^^>#
######.#",
                true
            ),
            day24_main(
                "#.######
#>>.<^<#
#.<..<<#
#>v.><>#
#<^v^^>#
######.#",
                false
            )
        )
    }
}
