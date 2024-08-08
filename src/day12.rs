// Surely this sort of thing is a solved problem and the algorithm I just came up with won't be the best one. Consider looking up
use std::collections::HashSet;

pub fn day12(input_lines: &str) -> (String, String) {
    let mut map = Map::from_str(input_lines);
    map.plot_accessible_borders();
    map.plot_routes_backwards(HashSet::from([map.end]), 0);

    let answer1 = map.grid[map.start.1][map.start.0].steps_from_end.unwrap();
    let answer2 = map
        .grid
        .iter()
        .flat_map(|row| row.iter())
        .filter_map(|square| {
            if square.steps_from_end.is_none() || square.height != 0 {
                None
            } else {
                square.steps_from_end
            }
        })
        .min()
        .unwrap();
    (format!("{}", answer1), format!("{}", answer2))
}

pub struct Map {
    grid: Vec<Vec<Square>>,
    max_x: usize,
    max_y: usize,
    start: (usize, usize),
    end: (usize, usize),
}

impl Map {
    fn from_str(input_lines: &str) -> Self {
        let mut start = (0, 0);
        let mut end = (0, 0);
        let max_x = input_lines.lines().next().unwrap().len() - 1;
        let max_y = input_lines.lines().count() - 1;
        let grid = input_lines
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
                                0
                            }
                            'E' => {
                                end = (x, y);
                                'z' as i32 - 'a' as i32
                            }
                            h => panic!("Unexpected height: {}", h),
                        };
                        Square::new(height, Map::cell_neighbours((x, y), max_x, max_y))
                    })
                    .collect::<Vec<Square>>()
            })
            .collect::<Vec<Vec<Square>>>();
        Map {
            grid,
            max_x,
            max_y,
            start,
            end,
        }
    }

    fn cell_neighbours(
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

    // Go through every square on a populated map and mark which squares are accessible from where.
    // (Like drawing little connecting lines on a paper map).
    pub fn plot_accessible_borders(&mut self) {
        for x in 0..=self.max_x {
            for y in 0..=self.max_y {
                let mut accessible_from = HashSet::new();
                let square = &self.grid[y][x];
                for &(i, j) in &square.neighbours {
                    if square.height <= self.grid[j][i].height + 1 {
                        accessible_from.insert((i, j));
                    }
                }
                self.grid[y][x].accessible_from = Some(accessible_from);
            }
        }
    }

    // Visit a square, if we've not been there before or we're here faster than previously, update the square and
    // return the next next squares we could visit.
    // (Imagine keeping a little paper mark on each square of your map and updating it each time you try a new route)
    pub fn visit_square(
        &mut self,
        square: (usize, usize),
        steps: i32,
    ) -> Result<HashSet<(usize, usize)>, &str> {
        let square = &mut self.grid[square.1][square.0];
        if square.steps_from_end.is_none() || square.steps_from_end.unwrap() > steps {
            square.steps_from_end = Some(steps);
            return Ok(square.accessible_from.clone().unwrap());
        }

        Err("Already visited in a better way")
    }

    // For a possible set of coordinate squares, walk backwards through all possible routes until we've gone as far as we can for each
    // without revisiting a square.
    // (This one is just getting the red pencil out and drawing routes, giving up when it's clear a previous route was better!)
    pub fn plot_routes_backwards(
        &mut self,
        route_options: HashSet<(usize, usize)>,
        starting_steps: i32,
    ) {
        let visited = route_options.clone();
        for square in route_options {
            let next_steps = self.visit_square(square, starting_steps);

            // New neighbours will always be at least one extra step away, so don't bother visiting nodes we're going to visit at this
            // level from within them.
            let next_steps = match next_steps {
                Ok(steps) => steps
                    .difference(&visited)
                    .cloned()
                    .collect::<HashSet<(usize, usize)>>(),
                Err(_) => continue,
            };
            self.plot_routes_backwards(next_steps, starting_steps + 1);
        }
    }
}

#[derive(Debug)]
pub struct Square {
    height: i32,
    neighbours: Vec<(usize, usize)>,
    accessible_from: Option<HashSet<(usize, usize)>>,
    steps_from_end: Option<i32>,
}

impl Square {
    pub fn new(height: i32, neighbours: Vec<(usize, usize)>) -> Self {
        Self {
            height,
            neighbours,
            accessible_from: None,
            steps_from_end: None,
        }
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
