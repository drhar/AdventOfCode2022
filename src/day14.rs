// Decided on HashMap over vectors filled with air to reduce memory usage at cost of speed. Would be simple to tweak based on requirements.
use std::{collections::HashMap, i32};

pub fn day14(input_lines: &str) -> (String, String) {
    let mut cave = Cave::from_path_scan(input_lines, None);
    cave.print();
    let mut answer1 = 0;
    loop {
        match cave.add_sand((500, 0)) {
            Ok(_) => {
                answer1 += 1;
            }
            Err(PlacementError::Abyss(_)) => break,
            Err(PlacementError::Blocked(_)) => break,
        }
    }
    cave.print();
    let mut cave2 = Cave::from_path_scan(input_lines, Some(2));
    let mut answer2 = 0;
    loop {
        match cave2.add_sand((500, 0)) {
            Ok(_) => {
                answer2 += 1;
            }
            Err(PlacementError::Abyss(_)) => panic!("We should never hit the abyss with a floor!"),
            Err(PlacementError::Blocked(_)) => break,
        }
    }
    (format!("{}", answer1), format!("{}", answer2))
}

pub struct Cave {
    map: HashMap<(i32, i32), TileType>,
    min_x: i32,
    max_x: i32,
    max_y: i32,
    floor: Option<i32>,
}

impl Cave {
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
            min_x: i32::MAX,
            max_x: i32::MIN,
            max_y: 0,
            floor: None,
        }
    }

    pub fn from_path_scan(scan: &str, floor: Option<i32>) -> Self {
        let mut cave = Cave::new();
        cave.floor = floor;
        cave = scan.lines().fold(cave, |mut cave, path| {
            let path: Vec<(i32, i32)> = path
                .split("->")
                .map(|tile| {
                    let coords = tile
                        .trim()
                        .split(',')
                        .map(|coord| coord.parse::<i32>().unwrap())
                        .collect::<Vec<i32>>();
                    (coords[0], coords[1])
                })
                .collect();
            for path_vec in path.windows(2) {
                let (start_x, start_y) = path_vec[0];
                let (end_x, end_y) = path_vec[1];
                let x_range = end_x - start_x;
                let y_range = end_y - start_y;
                for x_step in 0..=x_range.abs() {
                    let x = start_x + (x_step * x_range.signum());
                    for y_step in 0..=y_range.abs() {
                        let y = start_y + (y_step * y_range.signum());
                        cave.set_tile(x, y, TileType::Rock);
                        if x > cave.max_x {
                            cave.max_x = x;
                        }
                        if x < cave.min_x {
                            cave.min_x = x;
                        }
                        if y > cave.max_y {
                            cave.max_y = y;
                        }
                    }
                }
            }
            cave
        });
        cave
    }

    pub fn get_tile(&self, x: i32, y: i32) -> TileType {
        if self.floor.is_none() && (x < self.min_x || x > self.max_x || y > self.max_y)
            || (self.floor.is_some() && y > self.max_y + self.floor.unwrap())
        {
            return TileType::Abyss;
        }
        if self.floor.is_some() && y == self.max_y + self.floor.unwrap() {
            return TileType::Rock;
        }
        *self.map.get(&(x, y)).unwrap_or(&TileType::Air)
    }

    pub fn set_tile(&mut self, x: i32, y: i32, tile: TileType) {
        self.map.insert((x, y), tile);
    }

    pub fn add_sand(&mut self, spawn: (i32, i32)) -> Result<(i32, i32), PlacementError> {
        match self.floor {
            Some(f) => {
                if spawn.1 > self.max_y + f {
                    return Err(PlacementError::Abyss(spawn));
                }
            }
            None => {
                if spawn.1 > self.max_y {
                    return Err(PlacementError::Abyss(spawn));
                }
            }
        }

        match self.get_tile(spawn.0, spawn.1) {
            TileType::Air => {
                self.set_tile(spawn.0, spawn.1, TileType::Sand);
            }
            TileType::Abyss => return Err(PlacementError::Abyss(spawn)),
            _ => return Err(PlacementError::Blocked(spawn)),
        }
        let mut current = spawn;
        loop {
            match self.move_sand(current) {
                Ok(next) => current = next,
                Err(PlacementError::Blocked(_)) => return Ok(current),
                Err(PlacementError::Abyss(e)) => return Err(PlacementError::Abyss(e)),
            }
        }
    }

    fn move_sand(&mut self, start: (i32, i32)) -> Result<(i32, i32), PlacementError> {
        let (x, y) = start;
        if self.get_tile(x, y) != TileType::Sand {
            panic!("We should only be moving sand!");
        }
        for tile in [(x, y + 1), (x - 1, y + 1), (x + 1, y + 1)] {
            let tile_type = self.get_tile(tile.0, tile.1);
            match tile_type {
                TileType::Air => {
                    self.set_tile(x, y, TileType::Air);
                    self.set_tile(tile.0, tile.1, TileType::Sand);
                    return Ok(tile);
                }
                TileType::Abyss => {
                    return Err(PlacementError::Abyss(tile));
                }
                TileType::Rock | TileType::Sand => continue,
            }
        }
        Err(PlacementError::Blocked(start))
    }

    pub fn print(&self) {
        let max_y = if self.floor.is_some() {
            self.max_y + self.floor.unwrap()
        } else {
            self.max_y
        };
        for y in 0..=max_y {
            for x in self.min_x..=self.max_x {
                let tile = self.get_tile(x, y);
                let c = match tile {
                    TileType::Rock => '#',
                    TileType::Sand => 'o',
                    TileType::Air => '.',
                    TileType::Abyss => panic!("We should never be in the abyss!"),
                };
                print!("{}", c);
            }
            println!();
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum TileType {
    Abyss,
    Air,
    Sand,
    Rock,
}

#[derive(Debug)]
pub enum PlacementError {
    Blocked((i32, i32)),
    Abyss((i32, i32)),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_day14_part1_case1() {
        assert_eq!(
            day14(
                "498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9"
            )
            .0,
            "24".to_string()
        )
    }

    #[test]
    fn check_day14_part2_case1() {
        assert_eq!(
            day14(
                "498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9"
            )
            .1,
            "93".to_string()
        )
    }

    #[test]
    fn check_day14_both_case1() {
        assert_eq!(
            day14(
                "498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9"
            ),
            ("24".to_string(), "93".to_string())
        )
    }
}
