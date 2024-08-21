// First wrote this fully filling out the grid (unlike day 14), but that was sloooow.
// Didn't want to delete it all though so just left it as pretty printing apparatus.
use regex::Regex;

pub fn day15(input_lines: &str) -> (String, String) {
    day_15_business(input_lines, 2_000_000, 4_000_000, false)
}

pub fn day_15_business(
    input_lines: &str,
    level_to_search: i32,
    search_size: i32,
    print_grid: bool,
) -> (String, String) {
    let mut map = SensorMap::from_sensor_output(input_lines);

    if print_grid {
        map.fill_grid();
        map.print();
    }

    let answer1 = map
        .search_level(
            level_to_search,
            vec![GridSquare::Empty, GridSquare::Sensor(Sensor::new())],
            None,
            None,
        )
        .len();

    let (x, y) = (0..search_size)
        .find_map(|level| {
            let square =
                map.search_level(level, vec![GridSquare::Unknown], Some(0), Some(search_size));
            if square.is_empty() {
                None
            } else {
                Some(square)
            }
        })
        .unwrap()[0];

    let answer2: u64 = x as u64 * 4000000 + y as u64;

    (format!("{}", answer1), format!("{}", answer2))
}

#[derive(Debug, Clone, PartialEq)]
pub enum GridSquare {
    Beacon,
    Empty,
    Sensor(Sensor),
    Unknown,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Sensor {
    x: i32,
    y: i32,
    closest_beacon: (i32, i32),
    range: i32,
}

impl Sensor {
    pub fn new() -> Self {
        Self {
            x: 0,
            y: 0,
            closest_beacon: (0, 0),
            range: 0,
        }
    }

    pub fn from_str(sensor_output: &str) -> Self {
        // Could be global not per-Sensor
        let sensor_regex = Regex::new(
            r"Sensor at x=(\-?\d+), y=(\-?\d+): closest beacon is at x=(\-?\d+), y=(\-?\d+)",
        )
        .unwrap();
        let captures = sensor_regex.captures(sensor_output).unwrap();
        let x = captures[1].parse().unwrap();
        let y = captures[2].parse().unwrap();
        let closest_beacon = (captures[3].parse().unwrap(), captures[4].parse().unwrap());
        Self {
            x,
            y,
            closest_beacon,
            range: (closest_beacon.0 - x).abs() + (closest_beacon.1 - y).abs(),
        }
    }
}

pub struct SensorMap {
    // Filling the grid can be incredibly expensive for large maps, so only do it if necessary
    grid: Option<Vec<Vec<GridSquare>>>,
    sensors: Vec<Sensor>,
    min_x: i32,
    max_x: i32,
    max_y: i32,
}

impl SensorMap {
    pub fn from_sensor_output(sensor_output: &str) -> Self {
        let mut min_x = i32::MAX;
        let mut max_x = i32::MIN;
        let mut max_y = i32::MIN;
        let sensors = sensor_output
            .lines()
            .filter(|line| !line.is_empty())
            .map(|line| {
                let sensor = Sensor::from_str(line);
                min_x = min_x.min(sensor.x - sensor.range);
                max_x = max_x.max(sensor.x + sensor.range);
                max_y = max_y.max(sensor.y + sensor.range);
                sensor
            })
            .collect::<Vec<Sensor>>();
        SensorMap {
            sensors,
            min_x,
            max_x,
            max_y,
            grid: None,
        }
    }

    pub fn search_level(
        &self,
        level: i32,
        search_for: Vec<GridSquare>,
        min_x: Option<i32>,
        max_x: Option<i32>,
    ) -> Vec<(i32, i32)> {
        let y = level;
        let (min_x, max_x) = if let (Some(min), Some(max)) = (min_x, max_x) {
            (min, max)
        } else {
            (self.min_x, self.max_x)
        };
        let mut row = Vec::new();
        let mut x = min_x;

        // There's a bug in this - we don't distinguish between empty space and other sensors when stepping
        // through our sensor's range. We could do some post-processing, but it's not necessary yet so leaving
        // as WIBNI.
        while x < max_x {
            let x_start = x;
            // Checking every x point is sloooow. If we go sensor by sensor we can traverse big
            // sections at once at the cost of some clarity.
            for sensor in &self.sensors {
                let x_range = sensor.range - (y - sensor.y).abs();
                if x_range < 0 || x < sensor.x - x_range || x > sensor.x + x_range {
                    // Out of range of the sensor
                    continue;
                }

                // We're in range.
                if x < sensor.x {
                    // We're behind the sensor, this is our "left edge", so could be a beacon
                    if sensor.closest_beacon == (x, y) {
                        if search_for.contains(&GridSquare::Beacon) {
                            row.push((x, y));
                        }
                        x += 1;
                    }
                    // There will be empty space up until the sensor's x now
                    let smax = max_x.min(sensor.x);
                    if search_for.contains(&GridSquare::Empty) {
                        row.append(
                            (x..smax)
                                .map(|x| (x, y))
                                .collect::<Vec<(i32, i32)>>()
                                .as_mut(),
                        );
                    }
                    x += smax - x;
                }
                if x > max_x {
                    break;
                }

                // We're at or beyond the sensor's x.
                if x == sensor.x && y == sensor.y {
                    // We're on the sensor
                    if search_for
                        .iter()
                        .any(|filter| matches!(filter, GridSquare::Sensor(_)))
                    {
                        row.push((x, y));
                    }
                    x += 1;
                }
                // Everything up to the last item in the range must be empty (can't be a beacon as that would
                // be closer than the sensor's closest beacon).
                let smax = max_x.min(sensor.x + x_range);
                if search_for.contains(&GridSquare::Empty) {
                    row.append(
                        (x..smax)
                            .map(|x| (x, y))
                            .collect::<Vec<(i32, i32)>>()
                            .as_mut(),
                    );
                }
                x += smax - x;

                // Assuming we're less than max_x, we're on the right edge of this sensor, so could be a beacon again.
                if sensor.closest_beacon == (x, y) {
                    if search_for.contains(&GridSquare::Beacon) {
                        row.push((x, y));
                    }
                } else if search_for.contains(&GridSquare::Empty) {
                    row.push((x, y));
                }
                x += 1;
                if x > max_x {
                    break;
                }
            }
            // For points that no sensor gives us data about
            if x == x_start {
                if search_for.contains(&GridSquare::Unknown) {
                    row.push((x, y));
                    println!("Adding Unknown ({}, {})", x, y);
                }
                x += 1;
            }
        }
        row
    }

    fn add_sensor(&mut self, sensor: Sensor) {
        let s_x = sensor.x;
        let s_y = sensor.y;
        let (b_x, b_y) = (sensor.closest_beacon.0, sensor.closest_beacon.1);
        *self.get_mut_square(b_x, b_y) = GridSquare::Beacon;
        let manhatten_len = (b_x - s_x).abs() + (b_y - s_y).abs();
        for y_offset in -manhatten_len..=manhatten_len {
            let max_x_offset = manhatten_len - y_offset.abs();
            for x_offset in -max_x_offset..=max_x_offset {
                let x = s_x + x_offset;
                let y = s_y + y_offset;
                if x < self.min_x
                    || x >= self.max_x
                    || y < 0
                    || y >= self.max_y
                    || (x, y) == (b_x, b_y)
                {
                    continue;
                }
                let square = self.get_mut_square(x, y);
                match square {
                    GridSquare::Unknown => {
                        *square = GridSquare::Empty;
                    }
                    // Could do some clever de-overlapping of exclusion zones, but this is fine
                    GridSquare::Sensor(_) | GridSquare::Empty => continue,
                    GridSquare::Beacon => panic!("Found closer/equal beacon than ({b_x}, {b_y}) at ({x}, {y}) for sensor at ({s_x}, {s_y})"),
                }
            }
        }
        *self.get_mut_square(s_x, s_y) = GridSquare::Sensor(sensor);
    }

    fn get_mut_square(&mut self, x: i32, y: i32) -> &mut GridSquare {
        let x = (x - self.min_x) as usize;
        let y = y as usize;
        match self.grid {
            Some(ref mut grid) => &mut grid[y][x],
            None => panic!("Grid is not initialized"),
        }
    }
    pub fn fill_grid(&mut self) {
        let x_len = (self.max_x - self.min_x) as usize;
        self.grid = Some(vec![vec![GridSquare::Unknown; x_len]; self.max_y as usize]);
        for sensor in self.sensors.clone() {
            self.add_sensor(sensor);
        }
    }

    pub fn print(&self) {
        for y in 0..self.max_y {
            for x in 0..(self.max_x - self.min_x) {
                let square = if let Some(grid) = &self.grid {
                    &grid[y as usize][x as usize]
                } else {
                    panic!("Grid is not initialized")
                };
                let c = match square {
                    GridSquare::Beacon => 'B',
                    GridSquare::Empty => '#',
                    GridSquare::Sensor(_) => 'S',
                    GridSquare::Unknown => '.',
                };
                print!("{}", c);
            }
            println!();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_day15_part1_case1() {
        assert_eq!(
            day_15_business(
                "Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3",
                10,
                20,
                true
            )
            .0,
            "26".to_string()
        )
    }

    #[test]
    fn check_day15_part2_case1() {
        assert_eq!(
            day_15_business(
                "Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3",
                10,
                20,
                true
            )
            .1,
            "56000011".to_string()
        )
    }

    #[test]
    fn check_day15_both_case1() {
        assert_eq!(
            day_15_business(
                "Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3",
                10,
                20,
                true
            ),
            ("26".to_string(), "56000011".to_string())
        )
    }
}
