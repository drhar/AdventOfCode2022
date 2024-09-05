pub fn day20(input_lines: &str) -> (String, String) {
    let mut encrypted_file = File::new(input_lines, 0, 1);
    encrypted_file.mix();
    let answer1 = encrypted_file.get_coordinates().iter().sum::<i64>();
    let mut encrypted_file = File::new(input_lines, 0, 811589153);
    for _ in 0..10 {
        encrypted_file.mix();
    }
    let answer2 = encrypted_file.get_coordinates().iter().sum::<i64>();
    (format!("{}", answer1), format!("{}", answer2))
}

#[derive(Debug)]
pub struct Coordinate {
    value: i64,
    previous_cd: usize,
    next_cd: usize,
    #[allow(dead_code)]
    // Useful for debugging
    original_index: usize,
}

#[derive(Debug)]
pub struct File {
    coordinates: Vec<Coordinate>,
    start: usize,
    marker_pos: usize,
}

impl File {
    pub fn new(input_lines: &str, marker: i64, encryption_multiplier: i64) -> Self {
        let len = input_lines.lines().count();
        let mut marker_pos = 0;
        Self {
            coordinates: input_lines
                .lines()
                .enumerate()
                .map(|(i, line)| {
                    let value = line.parse::<i64>().unwrap() * encryption_multiplier;
                    if value == marker {
                        marker_pos = i
                    }
                    Coordinate {
                        value,
                        original_index: i,
                        previous_cd: if i == 0 { len - 1 } else { i - 1 },
                        next_cd: if i == len - 1 { 0 } else { i + 1 },
                    }
                })
                .collect(),
            start: 0,
            marker_pos,
        }
    }

    pub fn mix(&mut self) {
        for i in 0..self.coordinates.len() {
            let current = &self.coordinates[i];
            let next = current.next_cd;
            let previous = current.previous_cd;
            let new_previous = self.traverse_gaps(i, current.value);

            if new_previous != i {
                if self.start == i {
                    self.start = next;
                }
                self.coordinates[next].previous_cd = previous;
                self.coordinates[previous].next_cd = next;
                let new_next = self.coordinates[new_previous].next_cd;
                self.coordinates[new_previous].next_cd = i;
                self.coordinates[new_next].previous_cd = i;
                self.coordinates[i].previous_cd = new_previous;
                self.coordinates[i].next_cd = new_next;
            }
        }
    }

    pub fn traverse_gaps(&self, start: usize, steps: i64) -> usize {
        self.traverse(start, steps % (self.coordinates.len() as i64 - 1))
    }

    pub fn traverse(&self, start: usize, steps: i64) -> usize {
        let move_distance = steps % self.coordinates.len() as i64;

        let move_distance = if move_distance < 0 {
            // We have to move an extra step to get the previous rather than next.
            self.coordinates.len() as i64 + move_distance - 1
        } else {
            move_distance
        } as usize;

        let mut current = start;

        if move_distance > self.coordinates.len() / 2 {
            let move_distance = self.coordinates.len() - move_distance;
            for _ in 0..move_distance {
                current = self.coordinates[current].previous_cd;
            }
        } else {
            for _ in 0..move_distance {
                current = self.coordinates[current].next_cd;
            }
        }
        current
    }

    pub fn vec_index(&self, index: usize) -> &Coordinate {
        &self.coordinates[index]
    }

    pub fn get_coordinates(&self) -> Vec<i64> {
        [1000, 2000, 3000]
            .map(|value| self.vec_index(self.traverse(self.marker_pos, value)).value)
            .to_vec()
    }

    #[allow(dead_code)]
    pub fn print(&self) {
        let mut current = self.start;
        for _ in 0..self.coordinates.len() {
            print!("{} ", self.coordinates[current].value);
            current = self.coordinates[current].next_cd;
        }
        println!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_day20_part1_case1() {
        assert_eq!(
            day20(
                "1
2
-3
3
-2
0
4"
            )
            .0,
            "3".to_string()
        )
    }

    #[test]
    fn check_day20_part2_case1() {
        assert_eq!(
            day20(
                "1
2
-3
3
-2
0
4"
            )
            .1,
            "1623178306".to_string()
        )
    }

    #[test]
    fn check_day20_both_case1() {
        assert_eq!(
            day20(
                "1
2
-3
3
-2
0
4"
            ),
            ("3".to_string(), "1623178306".to_string())
        )
    }
}
