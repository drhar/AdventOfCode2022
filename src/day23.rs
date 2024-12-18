pub fn day23(input_lines: &str) -> (String, String) {
    let mut coordinator = Coordinator::from_str(
        input_lines,
        vec![
            Direction::North,
            Direction::South,
            Direction::West,
            Direction::East,
        ],
    );
    coordinator.print_scoring_grid();
    coordinator.run_rounds(10);
    coordinator.print_scoring_grid();
    let answer1 = coordinator.progress_score();
    coordinator.run_to_completion();
    coordinator.print_scoring_grid();
    let answer2 = coordinator.round_count;
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
}

pub struct Coordinator {
    grid: Vec<Vec<Position>>,
    elf_count: usize,
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
        let mut elf_count = 0;
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
                            elf_count += 1;
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
            elf_count,
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
        (self.southmost - self.northmost + 1) as u32
            * (self.eastmost as u32 - self.westmost as u32 + 1)
            - self.elf_count as u32
    }

    pub fn run_rounds(&mut self, rounds: u32) {
        for _ in 0..rounds {
            self.run_round();
        }
    }

    pub fn run_to_completion(&mut self) {
        while self.run_round() > 0 {}
    }

    fn run_round(&mut self) -> u32 {
        // This ensures every elf can move in every direction.
        if self.should_reallocate() {
            self.reallocate();
        }
        for y in self.northmost..=self.southmost {
            for x in self.westmost..=self.eastmost {
                if let Position::Elf = self.grid[y][x] {
                    if let Some((prop_x, prop_y)) = self.make_proposal((x, y)) {
                        if let Position::ProposedMove(ref mut moves) = self.grid[prop_y][prop_x] {
                            moves.push((x, y));
                        } else {
                            self.grid[prop_y][prop_x] = Position::ProposedMove(vec![(x, y)]);
                        }
                    }
                }
            }
        }
        let round_efficacy = self.move_elves();
        self.complete_round();
        self.round_count += 1;
        round_efficacy
    }

    fn should_reallocate(&self) -> bool {
        self.northmost == 0
            || self.southmost == self.grid.len() - 1
            || self.eastmost == self.grid[0].len() - 1
            || self.westmost == 0
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
        println!("Reallocated grid");
        self.print_grid();
        (width / 2, height / 2)
    }

    fn make_proposal(&mut self, elf_start: (usize, usize)) -> Option<(usize, usize)> {
        let (x, y) = elf_start;
        if !matches!(&self.grid[y - 1][x], Position::Elf)
            && !matches!(&self.grid[y + 1][x], Position::Elf)
            && !matches!(&self.grid[y][x - 1], Position::Elf)
            && !matches!(&self.grid[y][x + 1], Position::Elf)
            && !matches!(&self.grid[y - 1][x - 1], Position::Elf)
            && !matches!(&self.grid[y - 1][x + 1], Position::Elf)
            && !matches!(&self.grid[y + 1][x - 1], Position::Elf)
            && !matches!(&self.grid[y + 1][x + 1], Position::Elf)
        {
            return None;
        }
        let consideration_count = self.consideration_order.len();
        let mut consideration = self.first_consideration;
        for _ in 0..consideration_count {
            match &self.consideration_order[consideration] {
                Direction::North => {
                    if !matches!(&self.grid[y - 1][x], Position::Elf)
                        && !matches!(&self.grid[y - 1][x - 1], Position::Elf)
                        && !matches!(&self.grid[y - 1][x + 1], Position::Elf)
                    {
                        return Some((x, y - 1));
                    }
                }
                Direction::South => {
                    if !matches!(&self.grid[y + 1][x], Position::Elf)
                        && !matches!(&self.grid[y + 1][x - 1], Position::Elf)
                        && !matches!(&self.grid[y + 1][x + 1], Position::Elf)
                    {
                        return Some((x, y + 1));
                    }
                }
                Direction::East => {
                    if !matches!(&self.grid[y][x + 1], Position::Elf)
                        && !matches!(&self.grid[y - 1][x + 1], Position::Elf)
                        && !matches!(&self.grid[y + 1][x + 1], Position::Elf)
                    {
                        return Some((x + 1, y));
                    }
                }
                Direction::West => {
                    if !matches!(&self.grid[y][x - 1], Position::Elf)
                        && !matches!(&self.grid[y - 1][x - 1], Position::Elf)
                        && !matches!(&self.grid[y + 1][x - 1], Position::Elf)
                    {
                        return Some((x - 1, y));
                    }
                }
            };
            consideration = (consideration + 1) % consideration_count;
        }
        None
    }

    fn move_elves(&mut self) -> u32 {
        let mut moves_proposed = 0;
        let mut nm = self.northmost;
        let mut sm = self.southmost;
        let mut em = self.eastmost;
        let mut wm = self.westmost;
        for y in self.northmost - 1..=self.southmost + 1 {
            for x in self.westmost - 1..=self.eastmost + 1 {
                if let Position::ProposedMove(ref moves) = self.grid[y][x] {
                    moves_proposed += 1;

                    if moves.len() == 1 {
                        let move_from = moves[0];
                        self.grid[y][x] = Position::Elf;
                        self.grid[move_from.1][move_from.0] = Position::Empty;
                        nm = nm.min(y);
                        sm = sm.max(y);
                        em = em.max(x);
                        wm = wm.min(x);
                    } else {
                        self.grid[y][x] = Position::Empty;
                    }
                }
            }
        }
        // We need to check that all our elves haven't moved out of the extreme rows (inwards). Things can
        // only move at most one row and we don't need to check if the extreme has got more extreme.
        if self.northmost == nm && self.grid[nm].iter().all(|p| matches!(p, Position::Empty)) {
            nm += 1;
        }
        if self.southmost == sm && self.grid[sm].iter().all(|p| matches!(p, Position::Empty)) {
            sm -= 1;
        }
        if self.eastmost == em && self.grid.iter().all(|r| matches!(r[em], Position::Empty)) {
            em -= 1;
        }
        if self.westmost == wm && self.grid.iter().all(|r| matches!(r[wm], Position::Empty)) {
            wm += 1;
        }
        self.northmost = nm;
        self.southmost = sm;
        self.eastmost = em;
        self.westmost = wm;

        moves_proposed
    }

    fn complete_round(&mut self) {
        self.first_consideration = (self.first_consideration + 1) % self.consideration_order.len();
    }

    pub fn print_scoring_grid(&self) {
        println!("Scoring Grid:\n");
        for y in self.northmost..=self.southmost {
            for x in self.westmost..=self.eastmost {
                match self.grid[y][x] {
                    Position::Elf => print!("#"),
                    Position::Empty => print!("."),
                    Position::ProposedMove(_) => print!("?"), // Assuming '?' for proposed moves
                }
            }
            println!();
        }
        println!();
    }

    pub fn print_grid(&self) {
        println!("Grid:\n");
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
            "20".to_string()
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
            ("110".to_string(), "20".to_string())
        )
    }
}
