// This could do with a step back and re-design both from OOP perspective & "Surely this sort of thing is a
// solved problem and the algorithm I just came up with won't be the best one" perspective :).
use std::collections::HashSet;

pub fn day12(input_lines: &str) -> (String, String) {
    let mut start = (0, 0);
    let mut end = (0, 0);
    let max_x = input_lines.lines().next().unwrap().len() - 1;
    let max_y = input_lines.lines().count() - 1;
    let mut map = input_lines
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(|(x, height)| {
                    let height = match height {
                        'a'..='z' => height as i32 - 'a' as i32,
                        'S' => {
                            start = (x, y);
                            'a' as i32 - 'a' as i32
                        }
                        'E' => {
                            end = (x, y);
                            'z' as i32 - 'a' as i32
                        }
                        h => panic!("Unexpected height: {}", h),
                    };
                    Square::new(height, (x, y), max_x, max_y)
                })
                .collect::<Vec<Square>>()
        })
        .collect::<Vec<Vec<Square>>>();
    for x in 0..=max_x {
        for y in 0..=max_y {
            let neighbours = calculate_access((x, y), &map);
            map[y][x].accessible_from = Some(neighbours);
        }
    }
    walk_route_backwards(HashSet::from([end]), &mut map, 0);

    let answer1 = map[start.1][start.0].steps_from_end.unwrap();
    let answer2 = map
        .iter()
        .map(|row| row.iter())
        .flatten()
        .filter(|square| square.height == 0 && square.steps_from_end.is_some())
        .map(|square| square.steps_from_end.unwrap())
        .min()
        .unwrap();
    (format!("{}", answer1), format!("{}", answer2))
}

pub fn walk_route_backwards(
    neighbours: HashSet<(usize, usize)>,
    map: &mut Vec<Vec<Square>>,
    starting_steps: i32,
) {
    let visited = neighbours.clone();
    for neighbour in neighbours {
        let new_neighbours = map[neighbour.1][neighbour.0].extend_path(starting_steps);

        // New neighbours will always be at least one extra step away, so don't bother visiting nodes we're going to visit at this
        // level from within them.
        let new_neighbours = match new_neighbours {
            Ok(neighbours) => neighbours
                .difference(&visited)
                .cloned()
                .collect::<HashSet<(usize, usize)>>(),
            Err(_) => continue,
        };
        walk_route_backwards(new_neighbours, map, starting_steps + 1);
    }
}

pub fn calculate_access(square: (usize, usize), map: &Vec<Vec<Square>>) -> HashSet<(usize, usize)> {
    let square = &map[square.1][square.0];
    let mut accessible_neighbours = HashSet::new();
    for &(x, y) in &square.neighbours {
        let neighbour_square = &map[y][x];
        if square.height <= neighbour_square.height + 1 {
            accessible_neighbours.insert((x, y));
        }
    }
    accessible_neighbours
}

#[derive(Debug)]
pub struct Square {
    height: i32,
    neighbours: Vec<(usize, usize)>,
    accessible_from: Option<HashSet<(usize, usize)>>,
    steps_from_end: Option<i32>,
}

impl Square {
    pub fn new(height: i32, position: (usize, usize), max_x: usize, max_y: usize) -> Self {
        Self {
            height,
            neighbours: Self::calculate_neighbours(position, max_x, max_y),
            accessible_from: None,
            steps_from_end: None,
        }
    }

    fn calculate_neighbours(
        position: (usize, usize),
        max_x: usize,
        max_y: usize,
    ) -> Vec<(usize, usize)> {
        let (x, y) = position;
        let mut neighbours = Vec::new();
        if x > 0 {
            neighbours.push((x - 1, y));
        }
        if x < max_x {
            neighbours.push((x + 1, y));
        }
        if y > 0 {
            neighbours.push((x, y - 1));
        }
        if y < max_y {
            neighbours.push((x, y + 1));
        }
        neighbours
    }

    pub fn extend_path(&mut self, steps: i32) -> Result<HashSet<(usize, usize)>, &str> {
        if self.steps_from_end.is_none() || self.steps_from_end.unwrap() > steps {
            self.steps_from_end = Some(steps);
            return Ok(self.accessible_from.clone().unwrap());
        }

        Err("Already visited in a better way")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_day12_part1_case1() {
        assert_eq!(
            day12(
                "Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi"
            )
            .0,
            "31".to_string()
        )
    }

    #[test]
    fn check_day12_part2_case1() {
        assert_eq!(
            day12(
                "Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi"
            )
            .1,
            "29".to_string()
        )
    }

    #[test]
    fn check_day12_both_case1() {
        assert_eq!(
            day12(
                "Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi"
            ),
            ("31".to_string(), "29".to_string())
        )
    }
}
