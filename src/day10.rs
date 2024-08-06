pub fn day10(input_lines: &str) -> (String, String) {
    let (_, cycle_states) = input_lines
        .lines()
        .fold((0, vec![]), |(crt_pos, cycles), line| {
            let mut crt_pos = crt_pos;
            let mut cycles: Vec<Cycle> = cycles;
            let start = if cycles.is_empty() {
                1
            } else {
                cycles[cycles.len() - 1].sprite_middle_end
            };

            let mut instruction = line.split_whitespace();
            match instruction.next().unwrap() {
                "addx" => {
                    let value = instruction.next().unwrap().parse::<i32>().unwrap();
                    crt_pos = complete_cycle(&mut cycles, start, start, crt_pos);
                    crt_pos = complete_cycle(&mut cycles, start, start + value, crt_pos);
                }
                "noop" => {
                    crt_pos = complete_cycle(&mut cycles, start, start, crt_pos);
                }
                action => panic!("Unknown instruction {}", action),
            }
            (crt_pos, cycles)
        });

    let answer1 = signal_strength(&cycle_states, 20)
        + signal_strength(&cycle_states, 60)
        + signal_strength(&cycle_states, 100)
        + signal_strength(&cycle_states, 140)
        + signal_strength(&cycle_states, 180)
        + signal_strength(&cycle_states, 220);
    let answer2 = draw_screen(&cycle_states);
    (format!("{}", answer1), answer2.to_string())
}

pub fn signal_strength(cycle_states: &[Cycle], cycle_number: i32) -> i32 {
    let cycle_idx = cycle_number as usize - 1;
    cycle_number * cycle_states[cycle_idx].sprite_middle_start
}

pub fn light_pixel(sprite_middle: i32, crt_pos: i32) -> PixelState {
    if sprite_middle - 1 <= crt_pos && crt_pos <= sprite_middle + 1 {
        PixelState::Lit
    } else {
        PixelState::Unlit
    }
}

pub fn complete_cycle(
    cycle_states: &mut Vec<Cycle>,
    register_start: i32,
    register_end: i32,
    crt_pos: i32,
) -> i32 {
    let mut crt_pos = crt_pos;
    if crt_pos > 39 {
        crt_pos -= 40;
    }
    cycle_states.push(Cycle {
        sprite_middle_start: register_start,
        sprite_middle_end: register_end,
        pixel_state: light_pixel(register_start, crt_pos),
    });
    crt_pos += 1;
    crt_pos
}

pub fn draw_screen(cycle_states: &[Cycle]) -> String {
    let mut screen = String::new();
    let screen_width: usize = 40;
    let screen_height: usize = 6;
    assert_eq!(cycle_states.len(), screen_width * screen_height);
    for row in 0..screen_height {
        for pixel in 0..screen_width {
            let pixel_idx = row * screen_width + pixel;
            match cycle_states[pixel_idx].pixel_state {
                PixelState::Lit => {
                    screen.push('#');
                }
                PixelState::Unlit => {
                    screen.push('.');
                }
            }
        }
        if row < screen_height - 1 {
            screen.push('\n')
        }
    }
    println!("{}", screen);
    screen
}

#[derive(Debug)]
pub struct Cycle {
    sprite_middle_start: i32,
    sprite_middle_end: i32,
    pixel_state: PixelState,
}

#[derive(Debug)]
pub enum PixelState {
    Lit,
    Unlit,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_day10_part1_case1() {
        assert_eq!(
            day10(
                "addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop"
            )
            .0,
            "13140".to_string()
        )
    }

    #[test]
    fn check_day10_part2_case1() {
        assert_eq!(
            day10(
                "addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop"
            )
            .1,
            "##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######....."
                .to_string()
        )
    }

    #[test]
    fn check_day10_both_case1() {
        assert_eq!(
            day10(
                "addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop"
            ),
            (
                "13140".to_string(),
                "##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######....."
                    .to_string()
            )
        )
    }
}
