pub fn day23(input_lines: &str) -> (String, String) {
    let mut coordinator = Coordinator::from_str(
        input_lines,
        vec![
            Direction::North,
            Direction::South,
            Direction::East,
            Direction::West,
        ],
    );
    coordinator.run_rounds(10);
    let answer1 = coordinator.progress_score();
    let answer2 = 0;
    (format!("{}", answer1), format!("{}", answer2))
}

#[derive(Debug, Clone)]
pub enum Position {
    Elf,
    Empty,
    ProposedMove(Vec<(usize, usize)>),
}

#[derive(Debug)]
pub enum Direction {
    North,
    South,
    East,
    West,
    NorthEast,
    NorthWest,
    SouthEast,
    SouthWest,
}

pub struct Coordinator {
    grid: Vec<Vec<Position>>,
    consideration_order: Vec<Direction>,
    first_consideration: usize,
    round_count: u32,
    northmost: usize,
    southmost: usize,
    eastmost: usize,
    westmost: usize,
}

impl Coordinator {
    pub fn from_str(input: &str, consideration_order: Vec<Direction>) -> Self {
        let mut northmost = usize::MAX;
        let mut southmost = 0;
        let mut eastmost = 0;
        let mut westmost = usize::MAX;
        let grid = input
            .lines()
            .enumerate()
            .map(|(y, line)| {
                line.chars()
                    .enumerate()
                    .map(|(x, c)| match c {
                        '#' => {
                            northmost = northmost.min(y);
                            southmost = southmost.max(y);
                            eastmost = eastmost.max(x);
                            westmost = westmost.min(x);
                            Position::Elf
                        }
                        '.' => Position::Empty,
                        _ => panic!("Invalid character in input"),
                    })
                    .collect()
            })
            .collect();
        Self {
            grid,
            consideration_order,
            first_consideration: 0,
            round_count: 0,
            northmost,
            southmost,
            eastmost,
            westmost,
        }
    }

    pub fn progress_score(&self) -> u32 {
        let mut score = 0;
        for y in self.northmost..=self.southmost {
            for x in self.westmost..=self.eastmost {
                if !matches!(&self.grid[y][x], Position::Elf) {
                    score += 1;
                }
            }
        }
        score
    }

    pub fn run_rounds(&mut self, rounds: u32) {
        self.print_grid();
        for _ in 0..rounds {
            self.run_round();
            self.print_grid();
        }
    }

    fn run_round(&mut self) {
        // This ensures every elf can move in every direction.
        if self.should_reallocate() {
            self.reallocate();
        }
        for y in self.northmost..=self.southmost {
            for x in self.westmost..=self.eastmost {
                match self.grid[y][x] {
                    Position::Elf => self.make_proposal((x, y)),
                    _ => (),
                }
            }
        }
        self.move_elves();
        self.complete_round();
        self.round_count += 1;
    }

    // Make the grid bigger by half again in each direction. This is expensive as we have to allocate at
    // both ends of the vectors and also then need to update any proposed moves to reflect the new origin.
    // Hence why we do a lot of it at once when needed.
    fn reallocate(&mut self) -> (usize, usize) {
        let height = self.grid.len();
        let width = self.grid[0].len();
        let empty_row = vec![Position::Empty; width + 2 * (width / 2)];
        let mut new_grid = vec![empty_row.clone(); height / 2];
        for row in &mut self.grid {
            let mut new_row = vec![Position::Empty; width / 2];
            for pos in row.iter_mut() {
                if let Position::ProposedMove(ref mut moves) = pos {
                    for (x, y) in moves.iter_mut() {
                        *x += width / 2;
                        *y += height / 2;
                    }
                }
            }
            new_row.append(row);
            new_row.extend(vec![Position::Empty; width / 2]);
            new_grid.push(new_row);
        }
        new_grid.extend(vec![empty_row; height / 2]);
        self.grid = new_grid;
        self.northmost += height / 2;
        self.southmost += height / 2;
        self.eastmost += width / 2;
        self.westmost += width / 2;
        (width / 2, height / 2)
    }

    fn should_reallocate(&self) -> bool {
        if self.northmost == 0
            || self.southmost == self.grid.len() - 1
            || self.eastmost == self.grid[0].len() - 1
            || self.westmost == 0
        {
            true
        } else {
            false
        }
    }

    fn make_proposal(&mut self, elf_start: (usize, usize)) {
        let (x, y) = elf_start;
        if matches!(&self.grid[y - 1][x], Position::Empty)
            && matches!(&self.grid[y + 1][x], Position::Empty)
            && matches!(&self.grid[y][x - 1], Position::Empty)
            && matches!(&self.grid[y][x + 1], Position::Empty)
            && matches!(&self.grid[y - 1][x - 1], Position::Empty)
            && matches!(&self.grid[y - 1][x + 1], Position::Empty)
            && matches!(&self.grid[y + 1][x - 1], Position::Empty)
            && matches!(&self.grid[y + 1][x + 1], Position::Empty)
        {
            return;
        }
        let consideration_count = self.consideration_order.len();
        let mut consideration = self.first_consideration;
        for _ in 0..consideration_count {
            let proposal = match &self.consideration_order[consideration] {
                Direction::North => {
                    if !matches!(&self.grid[y - 1][x], Position::Elf)
                        && !matches!(&self.grid[y - 1][x - 1], Position::Elf)
                        && !matches!(&self.grid[y - 1][x + 1], Position::Elf)
                    {
                        (x, y - 1)
                    } else {
                        consideration = (consideration + 1) % consideration_count;
                        continue;
                    }
                }
                Direction::South => {
                    if !matches!(&self.grid[y + 1][x], Position::Elf)
                        && !matches!(&self.grid[y + 1][x - 1], Position::Elf)
                        && !matches!(&self.grid[y + 1][x + 1], Position::Elf)
                    {
                        (x, y + 1)
                    } else {
                        consideration = (consideration + 1) % consideration_count;
                        continue;
                    }
                }
                Direction::East => {
                    if !matches!(&self.grid[y][x + 1], Position::Elf)
                        && !matches!(&self.grid[y - 1][x + 1], Position::Elf)
                        && !matches!(&self.grid[y + 1][x + 1], Position::Elf)
                    {
                        (x + 1, y)
                    } else {
                        consideration = (consideration + 1) % consideration_count;
                        continue;
                    }
                }
                Direction::West => {
                    if !matches!(&self.grid[y][x - 1], Position::Elf)
                        && !matches!(&self.grid[y - 1][x - 1], Position::Elf)
                        && !matches!(&self.grid[y + 1][x - 1], Position::Elf)
                    {
                        (x - 1, y)
                    } else {
                        consideration = (consideration + 1) % consideration_count;
                        continue;
                    }
                }
                d => panic!("Not Implemented {:?}", d),
            };

            if let Position::ProposedMove(ref mut moves) = self.grid[proposal.1][proposal.0] {
                moves.push((x, y));
            } else {
                self.grid[proposal.1][proposal.0] = Position::ProposedMove(vec![(x, y)]);
            }
            return;
        }
    }

    fn move_elves(&mut self) {
        for y in self.northmost - 1..=self.southmost + 1 {
            for x in self.westmost - 1..=self.eastmost + 1 {
                match self.grid[y][x] {
                    Position::ProposedMove(ref moves) => {
                        if moves.len() == 1 {
                            let move_from = moves[0];
                            self.grid[y][x] = Position::Elf;
                            self.grid[move_from.1][move_from.0] = Position::Empty;
                            self.northmost = self.northmost.min(y);
                            self.southmost = self.southmost.max(y);
                            self.eastmost = self.eastmost.max(x);
                            self.westmost = self.westmost.min(x);
                        } else {
                            self.grid[y][x] = Position::Empty;
                        }
                    }
                    _ => (),
                }
            }
        }
    }

    fn complete_round(&mut self) {
        self.first_consideration = (self.first_consideration + 1) % self.consideration_order.len();
    }

    pub fn print_grid(&self) {
        for row in &self.grid {
            for pos in row {
                match pos {
                    Position::Elf => print!("#"),
                    Position::Empty => print!("."),
                    Position::ProposedMove(_) => print!("P"),
                }
            }
            println!();
        }
        println!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_day23_part1_case1() {
        assert_eq!(
            day23(
                ".....
..##.
..#..
.....
..##.
....."
            )
            .0,
            "25".to_string()
        )
    }

    #[test]
    fn check_day23_part2_case1() {
        assert_eq!(
            day23(
                ".....
..##.
..#..
.....
..##.
....."
            )
            .1,
            "0".to_string()
        )
    }

    #[test]
    fn check_day23_both_case1() {
        assert_eq!(
            day23(
                ".....
..##.
..#..
.....
..##.
....."
            ),
            ("25".to_string(), "0".to_string())
        )
    }
    #[test]
    fn check_day23_part1_case2() {
        assert_eq!(
            day23(
                "....#..
..###.#
#...#.#
.#...##
#.###..
##.#.##
.#..#.."
            )
            .0,
            "110".to_string()
        )
    }

    #[test]
    fn check_day23_part2_case2() {
        assert_eq!(
            day23(
                "....#..
..###.#
#...#.#
.#...##
#.###..
##.#.##
.#..#.."
            )
            .1,
            "0".to_string()
        )
    }

    #[test]
    fn check_day23_both_case2() {
        assert_eq!(
            day23(
                "....#..
..###.#
#...#.#
.#...##
#.###..
##.#.##
.#..#.."
            ),
            ("110".to_string(), "0".to_string())
        )
    }
}
