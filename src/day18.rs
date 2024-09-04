// Used bits to represent the droplet because it was useful for part 1, then decided to just stick with it for part 2.
// Occupancy isn't any worse, but it's not as clear as it could be if we used normal coordinates and didn't have to treat z differently.
use std::{collections::VecDeque, vec};

use itertools::Itertools;

pub fn day18(input_lines: &str) -> (String, String) {
    let mut droplet = Droplet::from_scan(input_lines);
    let answer1 = Droplet::surface_area(&droplet.grid);
    let answer2 = droplet.steam_area();
    droplet.print();
    (format!("{}", answer1), format!("{}", answer2))
}

pub struct Droplet {
    pub grid: Vec<Vec<u32>>,
    steamed_grid: Option<Vec<Vec<u32>>>,
    max_z: usize,
}

impl Droplet {
    pub fn from_scan(scan: &str) -> Self {
        let mut max_x: usize = 0;
        let mut max_y: usize = 0;
        let mut max_z: usize = 0;
        let mut coords = scan
            .lines()
            .map(|l| {
                let (x, y, z) = l
                    .split(',')
                    .map(|c| c.parse::<usize>().unwrap())
                    .next_tuple()
                    .unwrap();
                max_x = max_x.max(x);
                max_y = max_y.max(y);
                max_z = max_z.max(z);
                (x, y, z)
            })
            .collect::<Vec<(usize, usize, usize)>>();

        coords.sort_unstable();

        let mut grid = vec![vec![0u32; max_x + 1]; max_y + 1];
        for (x, y, z) in coords {
            grid[y][x] |= 1 << z as u32;
        }

        Self {
            grid,
            max_z,
            steamed_grid: None,
        }
    }

    pub fn surface_area(grid: &Vec<Vec<u32>>) -> u32 {
        let unit_areas = grid.iter().fold(0, |acc, layer| {
            acc + layer.iter().fold(0, |acc, row| acc + row.count_ones())
        }) * 6;
        let shared_areas = grid.iter().enumerate().fold(0, |acc, (y, layer)| {
            acc + layer.iter().enumerate().fold(0, |acc, (x, row)| {
                let ovlp_below = if y > 0 {
                    (row & grid[y - 1][x]).count_ones()
                } else {
                    0
                };
                let ovlp_above = if y < grid.len() - 1 {
                    (row & grid[y + 1][x]).count_ones()
                } else {
                    0
                };
                let ovlp_left = if x > 0 {
                    (row & grid[y][x - 1]).count_ones()
                } else {
                    0
                };
                let ovlp_right = if x < layer.len() - 1 {
                    (row & grid[y][x + 1]).count_ones()
                } else {
                    0
                };
                let ovlp_in_front = (row & row >> 1).count_ones();
                let ovlp_behind = (row & row << 1).count_ones();

                acc + ovlp_below + ovlp_above + ovlp_left + ovlp_right + ovlp_in_front + ovlp_behind
            })
        });

        unit_areas - shared_areas
    }

    pub fn steam_area(&mut self) -> u32 {
        if self.steamed_grid.is_none() {
            // Add some space around the edge to get rid of local minima.
            let mut visited = vec![vec![0; self.grid[0].len() + 2]; self.grid.len() + 2];
            let mut steamed_grid = vec![
                vec![(1 << (self.max_z + 3)) - 1; self.grid[0].len() + 2];
                self.grid.len() + 2
            ];
            let mut queue = VecDeque::new();
            queue.push_back((0, 0, 0));

            while !queue.is_empty() {
                let (x, y, z) = queue.pop_front().unwrap();
                if (1 << z) & visited[y][x] != 0
                    || (y > 0
                        && y < self.grid.len() + 1
                        && x > 0
                        && x < self.grid[0].len() + 1
                        && z > 0
                        && z < self.max_z + 2
                        && (1 << (z - 1)) & self.grid[y - 1][x - 1] != 0)
                {
                    continue;
                }
                visited[y][x] |= 1 << z;
                steamed_grid[y][x] &= !(1 << z);
                if x > 0 {
                    queue.push_back((x - 1, y, z));
                }
                if x < self.grid[0].len() + 1 {
                    queue.push_back((x + 1, y, z));
                }
                if y > 0 {
                    queue.push_back((x, y - 1, z));
                }
                if y < self.grid.len() + 1 {
                    queue.push_back((x, y + 1, z));
                }
                if z > 0 {
                    queue.push_back((x, y, z - 1));
                }
                if z < self.max_z + 2 {
                    queue.push_back((x, y, z + 1));
                }
            }

            self.steamed_grid = Some(steamed_grid);
        }

        Self::surface_area(self.steamed_grid.as_ref().unwrap())
    }

    pub fn print(&self) {
        println!("grid:\n");
        for layer in &self.grid {
            for row in layer {
                print!("{:0>width$b} ", row, width = self.max_z + 1);
            }
            println!();
        }
        if let Some(grid) = &self.steamed_grid {
            println!("\nSteam filled grid:\n");
            for layer in grid {
                for row in layer {
                    print!("{:0>width$b} ", row, width = self.max_z + 3);
                }
                println!();
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_day18_part1_case1() {
        assert_eq!(
            day18(
                "2,2,2
1,2,2
3,2,2
2,1,2
2,3,2
2,2,1
2,2,3
2,2,4
2,2,6
1,2,5
3,2,5
2,1,5
2,3,5"
            )
            .0,
            "64".to_string()
        )
    }

    #[test]
    fn check_day18_part2_case1() {
        assert_eq!(
            day18(
                "2,2,2
1,2,2
3,2,2
2,1,2
2,3,2
2,2,1
2,2,3
2,2,4
2,2,6
1,2,5
3,2,5
2,1,5
2,3,5"
            )
            .1,
            "58".to_string()
        )
    }

    #[test]
    fn check_day18_both_case1() {
        assert_eq!(
            day18(
                "2,2,2
1,2,2
3,2,2
2,1,2
2,3,2
2,2,1
2,2,3
2,2,4
2,2,6
1,2,5
3,2,5
2,1,5
2,3,5"
            ),
            ("64".to_string(), "58".to_string())
        )
    }
}
