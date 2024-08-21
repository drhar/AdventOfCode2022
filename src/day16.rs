use std::collections::{HashMap, VecDeque};

pub fn day16(input_lines: &str) -> (String, String) {
    let mut tunnel_system = TunnelSystem::from_scan(input_lines);
    let answer1 = tunnel_system.clone().explore_from_node("AA", 30, 1);
    answer1.print();
    let answer1 = answer1.pressure_released;

    let answer2 = tunnel_system.explore_from_node("AA", 26, 2);
    answer2.print();
    let answer2 = answer2.pressure_released;
    // let answer2 = 0;
    (format!("{}", answer1), format!("{}", answer2))
}

#[derive(Clone, Debug, PartialEq)]
pub enum ExplorerAction {
    Visit(String),
    Open(String),
    Wait,
}

#[derive(Clone, Debug)]
pub enum ExplorerResult {
    Ok(String),
    End,
}
#[derive(Clone, Debug)]
pub struct Valve {
    name: String,
    flow_rate: u32,
    tunnels: Vec<String>,
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
        }
    }

    pub fn open_valve(&mut self) -> u32 {
        self.flow_rate
    }
}

#[derive(Clone, Debug)]
pub struct Path {
    pub pressure_released: u32,
    pub flow_rate: u32,
    pub paths: Vec<Vec<String>>,
    pub locations: Vec<String>,
    pub track_path: bool,
    t_remaining: u8,
    open_valves: Vec<String>,
    max_flow: u32,
    recent_valves: Vec<Vec<String>>,
}

impl Path {
    pub fn new(
        start: &str,
        t_remaining: u8,
        open_valves: Vec<String>,
        max_flow: u32,
        explorers: usize,
    ) -> Self {
        Self {
            pressure_released: 0,
            flow_rate: 0,
            paths: vec![vec![]; explorers],
            locations: vec![start.to_string(); explorers],
            track_path: false,
            t_remaining,
            open_valves,
            max_flow,
            recent_valves: vec![vec![]; explorers],
        }
    }

    fn pass_time(&mut self) {
        self.pressure_released += self.flow_rate;
        self.t_remaining -= 1;
    }

    fn wait_it_out(&mut self) -> ExplorerResult {
        self.pressure_released += self.flow_rate * self.t_remaining as u32;
        self.t_remaining = 0;
        ExplorerResult::End
    }

    fn complete_path(&mut self) -> bool {
        if self.t_remaining == 0 || self.flow_rate == self.max_flow {
            self.wait_it_out();
            true
        } else {
            false
        }
    }

    fn visit_valve(&mut self, explorer: usize, valve: &mut Valve) -> ExplorerResult {
        // if self.pressure_released == 102 && self.flow_rate == 54 && self.t_remaining == 21 {
        //     self.print();
        // }
        if self.recent_valves[explorer].contains(&valve.name) {
            // Local loop
            return ExplorerResult::End;
        }
        self.locations[explorer] = valve.name.clone();
        self.paths[explorer].push(valve.name.clone());
        self.recent_valves[explorer].push(valve.name.clone());
        ExplorerResult::Ok(valve.name.clone())
    }

    fn turn_valve(&mut self, explorer: usize, valve: &mut Valve) -> ExplorerResult {
        if valve.name != *self.locations[explorer] {
            panic!("Can't turn a valve that we aren't at");
        }
        self.flow_rate += valve.open_valve();
        self.open_valves.push(valve.name.clone());
        self.paths[explorer].push("[v]".to_string());
        self.recent_valves[explorer].clear();
        self.recent_valves[explorer].push(valve.name.clone());
        ExplorerResult::Ok(valve.name.clone())
    }

    pub fn extend_path(
        &mut self,
        actions: Vec<ExplorerAction>,
        valve_map: &mut HashMap<String, Valve>,
    ) -> Vec<ExplorerResult> {
        let mut results = vec![ExplorerResult::End; actions.len()];
        if actions.len() != self.paths.len() {
            panic!(
                "We have {} actions for {} explorers at {:?}",
                actions.len(),
                self.paths.len(),
                self.paths
            );
        }
        self.pass_time();
        for (explorer, action) in actions.iter().enumerate() {
            match action {
                ExplorerAction::Visit(valve_name) => {
                    let valve = valve_map.get_mut(valve_name).unwrap();
                    results[explorer] = self.visit_valve(explorer, valve);
                }
                ExplorerAction::Open(valve_name) => {
                    if self.open_valves.contains(valve_name) {
                        panic!("Trying to open a valve a second time");
                    }
                    let valve = valve_map.get_mut(valve_name).unwrap();
                    results[explorer] = self.turn_valve(explorer, valve);
                }
                ExplorerAction::Wait => {
                    self.paths[explorer].push("[w]".to_string());
                    results[explorer] = ExplorerResult::End;
                }
            }
        }
        if self.complete_path() {
            return vec![ExplorerResult::End; actions.len()];
        }
        results
    }

    pub fn print(&self) {
        println!(
            "Path has p: {}, f: {}, t: {}",
            self.pressure_released, self.flow_rate, self.t_remaining
        );
        for (explorer, path) in self.paths.iter().enumerate() {
            println!("Explorer {explorer}: {:?}", path);
        }
    }
}

#[derive(Clone, Debug)]
pub struct TunnelSystem {
    valve_scan: HashMap<String, Valve>,
    exploration_states: HashMap<Vec<String>, Vec<(u32, u32, u8)>>,
    max_flow: u32,
}

impl TunnelSystem {
    pub fn from_scan(input_lines: &str) -> Self {
        let valve_scan = input_lines
            .lines()
            .filter(|line| !line.is_empty())
            .map(Valve::from_str)
            .map(|valve| (valve.name.clone(), valve))
            .collect::<HashMap<String, Valve>>();
        Self {
            max_flow: valve_scan.values().map(|v| v.flow_rate).sum::<u32>(),
            valve_scan,
            exploration_states: HashMap::new(),
        }
    }

    // I think this is a sort of BFS
    pub fn explore_from_node(
        &mut self,
        valve_name: &str,
        t_remaining: u8,
        explorers: usize,
    ) -> Path {
        let open_valves: Vec<String> = self
            .valve_scan
            .values()
            .filter_map(|v| match v.flow_rate {
                0 => Some(v.name.clone()),
                _ => None,
            })
            .collect();

        // When we visit a valve, time passes. The first valve shouldn't count as visiting as we're already there, so increment time remaining
        let path = Path::new(
            valve_name,
            t_remaining + 1,
            open_valves,
            self.max_flow,
            explorers,
        );

        let mut best_path = path.clone();
        let mut future_paths = VecDeque::new();
        future_paths.push_back((
            path,
            vec![ExplorerAction::Visit(valve_name.to_string()); explorers],
        ));

        let mut count = 0;
        while !future_paths.is_empty() {
            let (mut path, actions) = future_paths.pop_front().unwrap();
            if count % 100000 == 0 {
                println!(
                    "Processed {count} actions, {} in the queue, search_depth {}. Current max: {}",
                    future_paths.len(),
                    path.paths[0].len(),
                    best_path.pressure_released
                )
            }

            let results = path.extend_path(actions, &mut self.valve_scan);
            if results.iter().all(|r| matches!(r, ExplorerResult::End)) {
                if best_path.pressure_released < path.pressure_released {
                    best_path = path;
                }
                continue;
            }
            if self.is_worse_state(&path) {
                path.wait_it_out();
                if best_path.pressure_released < path.pressure_released {
                    best_path = path;
                }
                continue;
            }

            // Build a vector of the possible next actions for each explorer
            let mut explorers_actions: Vec<Vec<ExplorerAction>> = Vec::new();
            for explorer in 0..results.len() {
                let mut explorer_actions: Vec<ExplorerAction> = Vec::new();
                match &results[explorer] {
                    ExplorerResult::Ok(valve_name) => {
                        let valve = self.valve_scan.get(valve_name).unwrap();
                        if !path.open_valves.contains(&valve.name) {
                            explorer_actions.push(ExplorerAction::Open(valve.name.clone()));
                        }
                        explorer_actions.extend(
                            valve
                                .tunnels
                                .iter()
                                .map(|tunnel| ExplorerAction::Visit(tunnel.to_string()))
                                .collect::<Vec<ExplorerAction>>(),
                        );
                    }
                    ExplorerResult::End => {
                        explorer_actions.push(ExplorerAction::Wait);
                    }
                }
                explorers_actions.push(explorer_actions);
            }

            // Find all combinations of explorer actions. There's only "turn, turn", "turn, move", "move, move" (with all move combos) for 2 explorers,
            // so could easily have done that with iproduct or something, but something nice about being able to add more elephants.
            let mut next_actions = vec![vec![]];
            for explorer_actions in &explorers_actions {
                let mut new_actions = Vec::new();
                for curr_action_list in next_actions {
                    for action in explorer_actions {
                        if let ExplorerAction::Open(v) = action {
                            if curr_action_list.contains(&ExplorerAction::Open(v.clone())) {
                                continue;
                            }
                        }
                        let mut new_action = curr_action_list.clone();
                        new_action.push(action.clone());
                        new_actions.push(new_action);
                    }
                }
                next_actions = new_actions;
            }

            for actions in next_actions {
                future_paths.push_back((path.clone(), actions));
            }
            count += 1;
        }
        best_path
    }

    pub fn is_worse_state(&mut self, path: &Path) -> bool {
        // This is a heuristic for finding strictly worse paths. Should probably be called "is_probably_worse_state" - I've definitely not
        // proved that it always works. In general, it will give a pretty close answer though!
        //
        // If our explorers have been in these positions before with higher pressure, flow rate and time remaining,
        // then clearly this is a worse way to get here and we won't catch-up. However, if we check independently for each of these,
        // we'll end up with massive numbers of paths to check, taking ages and probably (a la BFS) running out of memory.
        //
        // We therefore sum:
        //    - The known pressure we'll release from this path (e.g. total if we did nothing else)
        //    - The pressure we would get if we open the valves at every explorer next go (avoids us over-valuing recently turned valves)
        //    - A worst case of turning on all the remaining valves with one s to go. Effectively a "heating" factor to avoid local minima, but ~broadly makes sense
        //
        // We could do better than the bottom two estimates by finding out the distances between all nodes and weighting
        // their flow rates properly, but lifes too short.
        let mut locations = path.locations.clone();
        locations.sort();
        let prior_visits = self.exploration_states.get(&locations);
        let prior_visits = if prior_visits.is_some() {
            prior_visits.unwrap()
        } else {
            self.exploration_states.insert(
                locations,
                vec![(path.pressure_released, path.flow_rate, path.t_remaining)],
            );
            return false;
        };

        let remaining_valve_count = (self.valve_scan.keys().len() - path.open_valves.len()) as u8;

        // Our worst case: we turn on all remaining valves, but with only 1s to go.
        let mut worst_case = self.max_flow - path.flow_rate;

        // Avoid over-valuing paths where we just turned valves, by pretending all currently occupied valves are open.
        let state_potential_flow_rate = locations
            .iter()
            .map(|v| {
                if !path.open_valves.contains(v) {
                    self.valve_scan.get(v).unwrap().flow_rate
                } else {
                    0
                }
            })
            .sum::<u32>();

        // If these valves were closed, they're now open so remove from our worst case.
        worst_case -= state_potential_flow_rate;

        // Make our worst case an even worse-r case - it takes 2s to turn on a valve (move and open)
        // Again, life's too short to use actual valve's flows so let's just average the remaining over all of them.
        if remaining_valve_count > path.t_remaining / 2 {
            worst_case = (worst_case * path.t_remaining as u32 / 2) / remaining_valve_count as u32
        }

        if prior_visits.iter().any(|(p, f, t)| {
            *p + (*f * *t as u32)
                > path.pressure_released
                    + worst_case
                    + (path.flow_rate + state_potential_flow_rate) * path.t_remaining as u32
        }) {
            true
        } else {
            self.exploration_states.get_mut(&locations).unwrap().push((
                path.pressure_released,
                path.flow_rate,
                path.t_remaining,
            ));
            false
        }
    }
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
            "1707".to_string()
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
            ("1651".to_string(), "1707".to_string())
        )
    }
}
