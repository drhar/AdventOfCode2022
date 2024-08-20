use core::panic;
use std::{collections::HashMap, collections::VecDeque};

pub fn day16(input_lines: &str) -> (String, String) {
    let valve_scan = parse_scan(input_lines);
    let best_path = explore_from_node("AA", valve_scan, 30);
    println!("{:?}", best_path.path);
    let answer1 = best_path.pressure_released;
    let answer2 = 0;
    (format!("{}", answer1), format!("{}", answer2))
}

#[derive(Clone)]
pub struct Valve {
    name: String,
    flow_rate: u32,
    tunnels: Vec<String>,
    visited: Vec<(u32, u32, u8)>,
}

impl Valve {
    pub fn from_str(input_str: &str) -> Self {
        let valve_regex = regex::Regex::new(
            r"Valve ([A-Z]+) has flow rate=(\d+); tunnels? leads? to valves? ([A-Z,\s]+)",
        )
        .unwrap();
        let captures = valve_regex.captures(input_str).unwrap();
        let name: String = captures.get(1).unwrap().as_str().to_string();
        let flow_rate: u32 = captures.get(2).unwrap().as_str().parse().unwrap();
        let tunnels: Vec<String> = captures
            .get(3)
            .unwrap()
            .as_str()
            .split(", ")
            .map(|s| s.to_string())
            .collect();
        Self {
            name,
            flow_rate,
            tunnels,
            visited: Vec::new(),
        }
    }

    pub fn turn_valve(&mut self) -> u32 {
        self.flow_rate
    }

    pub fn worth_visiting(
        &self,
        pressure_released: u32,
        flow_rate: u32,
        t_remaining: u8,
        valve_open: bool,
    ) -> bool {
        // This is a heuristic for finding strictly worse paths.
        // If we've been here before with higher pressure, flow rate and time remaining, then clearly it's a worse way to get here and we won't catch-up.
        // If we check independently for each of these, we'll still end up with massive numbers of paths to check.
        // We therefore estimate the total future pressure as if we're not going to move once we've opened this valve.
        // We could do better by weighting in all the open valves with their relative distances to this valve, but lifes too short.
        let this_flow_rate = if !valve_open { self.flow_rate - 1 } else { 0 };
        !self.visited.iter().any(|(p, f, t)| {
            *p + (*f * *t as u32)
                > pressure_released + (flow_rate + this_flow_rate) * t_remaining as u32
        })
    }

    pub fn visit(&mut self, pressure_released: u32, flow_rate: u32, t_remaining: u8) -> PathResult {
        // A visit is only definitely worse time remaining is lower and flow rate hasn't increased
        self.visited
            .push((pressure_released, flow_rate, t_remaining));
        PathResult::Ok
    }
}

#[derive(Clone, Debug)]
pub struct Path {
    pub pressure_released: u32,
    pub flow_rate: u32,
    pub path: Vec<String>,
    t_remaining: u8,
    open_valves: Vec<String>,
    max_flow: u32,
}

impl Path {
    pub fn new(start: &str, t_remaining: u8, open_valves: Vec<String>, max_flow: u32) -> Self {
        Self {
            pressure_released: 0,
            flow_rate: 0,
            path: vec![start.to_string()],
            t_remaining,
            open_valves,
            max_flow,
        }
    }

    fn pass_time(&mut self) {
        self.pressure_released += self.flow_rate;
        self.t_remaining -= 1;
    }

    pub fn wait_it_out(&mut self) -> PathResult {
        self.pressure_released += self.flow_rate * self.t_remaining as u32;
        self.t_remaining = 0;
        PathResult::End
    }

    fn complete_path(&mut self) -> bool {
        if self.t_remaining == 0 || self.flow_rate == self.max_flow {
            self.wait_it_out();
            true
        } else {
            false
        }
    }

    pub fn visit_valve(&mut self, valve: &mut Valve) -> PathResult {
        self.pass_time();
        if !valve.worth_visiting(
            self.pressure_released,
            self.flow_rate,
            self.t_remaining,
            self.open_valves.contains(&valve.name),
        ) {
            self.wait_it_out();
            return PathResult::End;
        }
        valve.visit(self.pressure_released, self.flow_rate, self.t_remaining);
        self.path.push(valve.name.clone());
        if self.complete_path() {
            return PathResult::End;
        }
        PathResult::Ok
    }

    pub fn turn_valve(&mut self, valve: &mut Valve) -> PathResult {
        if valve.name != *self.path.last().unwrap() {
            panic!("Can't turn a valve that we aren't at");
        }
        self.pass_time();
        self.flow_rate += valve.turn_valve();
        valve.visit(self.pressure_released, self.flow_rate, self.t_remaining);
        self.open_valves.push(valve.name.clone());
        self.path.push("[v]".to_string());
        if self.complete_path() {
            return PathResult::End;
        }

        PathResult::Ok
    }
}

// I think this is a sort of BFS
pub fn explore_from_node(
    valve_name: &str,
    mut valve_scan: HashMap<String, Valve>,
    t_remaining: u8,
) -> Path {
    let open_valves: Vec<String> = valve_scan
        .values()
        .filter_map(|v| match v.flow_rate {
            0 => Some(v.name.clone()),
            _ => None,
        })
        .collect();
    let max_flow = valve_scan.values().map(|v| v.flow_rate).max().unwrap_or(0);
    // When we visit a valve, time passes. The first valve shouldn't count as visiting as we're already there, so increment time remaining
    let path = Path::new(valve_name, t_remaining + 1, open_valves, max_flow);

    let mut best_path = path.clone();
    let mut future_paths = VecDeque::new();
    future_paths.push_back(PathAction::Visit(path, valve_name.to_string()));

    while !future_paths.is_empty() {
        let mut action = future_paths.pop_front().unwrap();
        let (result, mut path, valve) = match action {
            PathAction::Visit(mut path, valve_name) => {
                let valve = valve_scan.get_mut(&valve_name).unwrap();
                (path.visit_valve(valve), path, valve.name.clone())
            }
            PathAction::Turn(mut path, valve_name) => {
                let valve = valve_scan.get_mut(&valve_name).unwrap();
                (path.turn_valve(valve), path, valve.name.clone())
            }
        };
        match result {
            PathResult::Ok => {
                let valve = valve_scan.get(&valve).unwrap();
                if !path.open_valves.contains(&valve.name) {
                    future_paths.push_back(PathAction::Turn(path.clone(), valve.name.clone()));
                }
                for tunnel in &valve.tunnels {
                    let next_valve = valve_scan.get(tunnel).unwrap();
                    future_paths.push_back(PathAction::Visit(path.clone(), tunnel.to_string()));
                }
            }
            PathResult::End => {
                if best_path.pressure_released < path.pressure_released {
                    best_path = path;
                }
            }
        }
    }
    best_path
}

#[derive(Debug)]
pub enum PathAction {
    Visit(Path, String),
    Turn(Path, String),
}

#[derive(Debug)]
pub enum PathResult {
    Ok,
    End,
}

pub fn parse_scan(input_lines: &str) -> HashMap<String, Valve> {
    input_lines
        .lines()
        .filter(|line| !line.is_empty())
        .map(Valve::from_str)
        .map(|valve| (valve.name.clone(), valve))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_day16_part1_case1() {
        assert_eq!(
            day16(
                "Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
Valve BB has flow rate=13; tunnels lead to valves CC, AA
Valve CC has flow rate=2; tunnels lead to valves DD, BB
Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
Valve EE has flow rate=3; tunnels lead to valves FF, DD
Valve FF has flow rate=0; tunnels lead to valves EE, GG
Valve GG has flow rate=0; tunnels lead to valves FF, HH
Valve HH has flow rate=22; tunnel leads to valve GG
Valve II has flow rate=0; tunnels lead to valves AA, JJ
Valve JJ has flow rate=21; tunnel leads to valve II"
            )
            .0,
            "1651".to_string()
        )
    }

    #[test]
    fn check_day16_part2_case1() {
        assert_eq!(
            day16(
                "Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
Valve BB has flow rate=13; tunnels lead to valves CC, AA
Valve CC has flow rate=2; tunnels lead to valves DD, BB
Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
Valve EE has flow rate=3; tunnels lead to valves FF, DD
Valve FF has flow rate=0; tunnels lead to valves EE, GG
Valve GG has flow rate=0; tunnels lead to valves FF, HH
Valve HH has flow rate=22; tunnel leads to valve GG
Valve II has flow rate=0; tunnels lead to valves AA, JJ
Valve JJ has flow rate=21; tunnel leads to valve II"
            )
            .1,
            "0".to_string()
        )
    }

    #[test]
    fn check_day16_both_case1() {
        assert_eq!(
            day16(
                "Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
Valve BB has flow rate=13; tunnels lead to valves CC, AA
Valve CC has flow rate=2; tunnels lead to valves DD, BB
Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
Valve EE has flow rate=3; tunnels lead to valves FF, DD
Valve FF has flow rate=0; tunnels lead to valves EE, GG
Valve GG has flow rate=0; tunnels lead to valves FF, HH
Valve HH has flow rate=22; tunnel leads to valve GG
Valve II has flow rate=0; tunnels lead to valves AA, JJ
Valve JJ has flow rate=21; tunnel leads to valve II"
            ),
            ("1651".to_string(), "0".to_string())
        )
    }
}
