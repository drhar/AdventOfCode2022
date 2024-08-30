use std::{
    collections::VecDeque,
    iter::{repeat, Repeat},
};

const SURFACE_DEPTH: usize = 16;

pub fn day17(input_lines: &str) -> (String, String) {
    let mut chamber = Chamber::new(
        7,
        VecDeque::from([
            RockType::HLine,
            RockType::Cross,
            RockType::LShape,
            RockType::VLine,
            RockType::Box,
        ]),
        Chamber::scan_jets(input_lines),
    );
    chamber.add_rocks(2022);
    let answer1 = chamber.rock_height;
    // chamber.print();

    let answer2 = chamber.simulate_rocks(1000000000000 - chamber.added_rocks);
    (format!("{}", answer1), format!("{}", answer2))
}

#[derive(Clone, Debug, PartialEq)]
struct Rock {
    pub height: usize,
    pub width: usize,
    internal_make_up: Vec<u32>,
}

impl Rock {
    pub fn new(rock_type: RockType) -> Self {
        match rock_type {
            RockType::HLine => Rock {
                height: 1,
                width: 4,
                internal_make_up: vec![0b1111],
            },
            RockType::Cross => Rock {
                height: 3,
                width: 3,
                internal_make_up: vec![0b010, 0b111, 0b010],
            },
            RockType::LShape => Rock {
                height: 3,
                width: 3,
                internal_make_up: vec![0b111, 0b001, 0b001],
            },
            RockType::VLine => Rock {
                height: 4,
                width: 1,
                internal_make_up: vec![1, 1, 1, 1],
            },
            RockType::Box => Rock {
                height: 2,
                width: 2,
                internal_make_up: vec![0b11, 0b11],
            },
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RockType {
    HLine,
    Cross,
    LShape,
    VLine,
    Box,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Jet {
    Left,
    Right,
}

#[derive(Clone)]
struct Chamber {
    contents: Vec<u32>,
    rock_height: usize,
    chamber_width: usize,
    added_rocks: u64,
    rock_generator: (VecDeque<RockType>, Repeat<VecDeque<RockType>>),
    jet_generator: (VecDeque<Jet>, Repeat<VecDeque<Jet>>),
    empty_layer: u32,
    repeating_unit_layer_start: usize,
    repeating_unit_height: usize,
    repeating_unit_rock_count: u64,
    repeating_unit_rock_count_start: u64,
}

impl Chamber {
    pub fn new(width: usize, rock_wave: VecDeque<RockType>, jet_scan: VecDeque<Jet>) -> Self {
        // We're using bits as bits of space, with 1 as a rock and 0 as empty space. The floor and walls are all rock.
        // We want the cavern width + 2 walls all set to 1.
        let floor: u32 = (1 << (width + 2)) - 1;

        Self {
            contents: vec![floor],
            rock_height: 0,
            chamber_width: width,
            added_rocks: 0,
            rock_generator: (rock_wave.clone(), repeat(rock_wave)),
            jet_generator: (jet_scan.clone(), repeat(jet_scan)),
            // 1000....0001
            empty_layer: 1 << (width + 1) | 1,
            repeating_unit_layer_start: 0,
            repeating_unit_height: 0,
            repeating_unit_rock_count: 0,
            repeating_unit_rock_count_start: 0,
        }
    }

    pub fn clear(&mut self) {
        self.contents = vec![self.contents[0]];
        self.rock_height = 0;
        self.added_rocks = 0;
        self.repeating_unit_layer_start = 0;
        self.repeating_unit_height = 0;
        self.repeating_unit_rock_count = 0;
        self.repeating_unit_rock_count_start = 0;
        self.rock_generator.0 = self.rock_generator.1.next().unwrap();
        self.jet_generator.0 = self.jet_generator.1.next().unwrap();
    }

    pub fn scan_jets(scan: &str) -> VecDeque<Jet> {
        scan.chars()
            .map(|c| match c {
                '>' => Jet::Right,
                '<' => Jet::Left,
                j => panic!("Invalid character in jet scan: {}", j),
            })
            .collect()
    }

    pub fn add_rocks(&mut self, count: u64) {
        for _ in 0..count {
            self.add_rock(false);
        }
    }

    pub fn simulate_rocks(&mut self, count: u64) -> usize {
        let starting_count = self.added_rocks;

        if self.repeating_unit_height == 0 {
            // We do a Brent search for repeated patterns as we add rocks. We can then repeat
            // to build up the tower rather than adding every rock.
            // How heuristic for repeated patterns is that the next rock to drop, the next jet to trigger
            // and the "surface" of the chamber must match. The surface depth may need tuning, and we could
            // be more specific about how many of the next jets need to match.
            let mut tortoise = Chamber::new(
                self.chamber_width,
                self.rock_generator.1.next().unwrap(),
                self.jet_generator.1.next().unwrap(),
            );

            while tortoise.rock_height < SURFACE_DEPTH + 1 {
                tortoise.add_rock(false);
                if tortoise.added_rocks == count {
                    return tortoise.rock_height;
                }
                println!("Build up tortoise");
            }
            let mut hare = tortoise.clone();
            hare.add_rock(false);

            let mut two_power = 1u64;
            self.repeating_unit_rock_count = 1;

            while tortoise.current_state() != hare.current_state() {
                if hare.added_rocks % 100 == 0 {
                    println!("Hare: {}", hare.added_rocks);
                }
                if two_power == self.repeating_unit_rock_count {
                    tortoise = hare.clone();
                    two_power *= 2;
                    self.repeating_unit_rock_count = 0;
                }
                hare.add_rock(false);
                self.repeating_unit_rock_count += 1;
            }

            self.repeating_unit_height = hare.rock_height - tortoise.rock_height;
            println!(
                "Repeating unit height: {}, Repeating unit count {}",
                self.repeating_unit_height, self.repeating_unit_rock_count
            );

            tortoise.clear();
            hare.clear();
            hare.add_rocks(self.repeating_unit_rock_count);

            self.repeating_unit_rock_count_start = 0;
            while tortoise.current_state() != hare.current_state() {
                tortoise.add_rock(false);
                hare.add_rock(false);
                self.repeating_unit_rock_count_start += 1;
            }
            self.repeating_unit_layer_start = tortoise.rock_height - SURFACE_DEPTH;
            println!(
                "Repeating unit layer start: {}",
                self.repeating_unit_layer_start
            );
        }

        // The tower height will be the repeating unit start + k repeating units + the extra layers added by remaining rocks.
        let number_repeating_units = (count + starting_count
            - self.repeating_unit_rock_count_start)
            / self.repeating_unit_rock_count;
        let extra_rocks = (count + starting_count - self.repeating_unit_rock_count_start)
            % self.repeating_unit_rock_count;

        let mut sim_chamber = Chamber::new(
            self.chamber_width,
            self.rock_generator.1.next().unwrap(),
            self.jet_generator.1.next().unwrap(),
        );

        // Get to the boundary of a repeating unit.
        sim_chamber.add_rocks(self.repeating_unit_rock_count_start);
        println!("Start repeating unit");
        sim_chamber.print();

        // Add the extra rocks to see what height they add. The simulated chamber's height is now the height before and after
        // any repeating units
        sim_chamber.add_rocks(extra_rocks);

        sim_chamber.rock_height + number_repeating_units as usize * self.repeating_unit_height
    }

    fn add_rock(&mut self, print: bool) {
        let rock = self.next_rock();
        if print {
            self.print();
        }
        // Rocks are added with 3 empty rows below them.
        let layers_to_add = 0.max(3 - (self.contents.len() - 1 - self.rock_height) as i32);
        if print {
            // println!("Layers to add: {}", layers_to_add);
        }
        self.contents
            .extend(vec![self.empty_layer; layers_to_add as usize]);

        let mut rocks_bottom = self.rock_height + 3 + 1;
        let mut rock_space = Vec::new();
        // Rocks are added with 2 empty columns to the left of them. We then need to account for the right wall.
        let left_move = self.chamber_width - rock.width - 2 + 1;
        for level in 0..rock.height {
            if rocks_bottom + level >= self.contents.len() {
                self.contents.push(self.empty_layer);
            }
            let layer = rock.internal_make_up[level] << left_move;
            rock_space.push(layer);
        }
        let mut at_rest = false;
        while !at_rest {
            let jet = self.next_jet();
            if print {
                println!("{:?}", jet);
                for row in &rock_space {
                    println!("{:b}", row);
                }
            }
            match jet {
                Jet::Left => {
                    if !rock_space
                        .iter()
                        .enumerate()
                        .any(|(i, &layer)| layer << 1 & self.contents[rocks_bottom + i] != 0)
                    {
                        for rock_layer in rock_space.iter_mut() {
                            *rock_layer <<= 1;
                        }
                    }
                }
                Jet::Right => {
                    if !rock_space
                        .iter()
                        .enumerate()
                        .any(|(i, &layer)| layer >> 1 & self.contents[rocks_bottom + i] != 0)
                    {
                        for rock_layer in rock_space.iter_mut() {
                            *rock_layer >>= 1;
                        }
                    }
                }
            }
            if rock_space
                .iter()
                .enumerate()
                .any(|(level, &layer)| layer & self.contents[rocks_bottom + level - 1] != 0)
            {
                if print {
                    println!("At rest");
                    println!("Rocks Bottom: {}", rocks_bottom);
                    println!(
                        "{}",
                        rock_space
                            .iter()
                            .map(|&row| row & self.contents[rocks_bottom - 1])
                            .collect::<Vec<u32>>()
                            .iter()
                            .map(|&row| format!("{:b}", row))
                            .collect::<Vec<String>>()
                            .join("\n")
                    );
                }
                at_rest = true;
            } else {
                rocks_bottom -= 1;
            }
            if print {
                // println!("Rocks Bottom: {}", rocks_bottom);
            }
        }
        // let mut floor_raise = 0;
        for (i, &layer) in rock_space.iter().enumerate() {
            self.contents[rocks_bottom + i] |= layer;
            // if self.contents[rocks_bottom + i] ^ ((1 << self.chamber_width + 2) - 1) == 0 {
            //     // New floor! We'll clean up our space later in case there are more floors caused by this rock
            //     floor_raise = rocks_bottom + i;
            // }
        }
        self.added_rocks += 1;

        self.rock_height = self.rock_height.max(rocks_bottom + rock.height - 1);
        // if floor_raise > 0 {
        //     self.floor_height += floor_raise;
        //     self.rock_height -= floor_raise;
        //     self.contents.drain(..floor_raise);
        // }
        if print {
            println!("Rock Height: {}", self.rock_height);
        }
    }

    fn next_jet(&mut self) -> Jet {
        Chamber::next_item(&mut self.jet_generator)
    }

    fn next_rock(&mut self) -> Rock {
        Rock::new(Chamber::next_item(&mut self.rock_generator))
    }

    fn next_item<T: Clone>(generator: &mut (VecDeque<T>, Repeat<VecDeque<T>>)) -> T {
        let (current_wave, wave_generator) = generator;
        if let Some(item) = current_wave.pop_front() {
            item
        } else {
            *current_wave = wave_generator.next().unwrap();
            current_wave.pop_front().unwrap()
        }
    }

    fn peek_item<T: Clone>(generator: &(VecDeque<T>, Repeat<VecDeque<T>>)) -> Option<&T> {
        generator.0.front()
    }

    fn current_state(&self) -> (&[u32], Option<&RockType>, Option<&Jet>) {
        let surface_depth = SURFACE_DEPTH.min(self.rock_height);
        (
            &self.contents[(self.rock_height + 1) - surface_depth..=self.rock_height],
            Chamber::peek_item(&self.rock_generator),
            Chamber::peek_item(&self.jet_generator),
        )
    }

    fn print(&self) {
        for row in self.contents.iter().rev() {
            let row = format!("{:b}", row);
            println!(
                "{}",
                row.chars()
                    .map(|c| if c == '1' { '#' } else { '.' })
                    .collect::<String>()
            );
        }
        println!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_day17_part1_case1() {
        assert_eq!(
            day17(">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>").0,
            "3068".to_string()
        )
    }

    #[test]
    fn check_day17_part2_case1() {
        assert_eq!(
            day17(">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>").1,
            "1514285714288".to_string()
        )
    }

    #[test]
    fn check_day17_both_case1() {
        assert_eq!(
            day17(">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>"),
            ("3068".to_string(), "1514285714288".to_string())
        )
    }
}
