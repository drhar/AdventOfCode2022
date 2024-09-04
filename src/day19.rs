// Firt I didn't read the problem properly and thought I could build as many robots a day as I could,
// then I worked out the optimisation of stepping by robot rather than by minute. But I didn't want to
// rip out that code (I would on a prdouction project, but always useful to have this stuff for my future use).
use std::collections::{HashMap, HashSet, VecDeque};
use std::ops::{Add, AddAssign, Mul, Sub, SubAssign};

use regex::Regex;

pub fn day19(input_lines: &str) -> (String, String) {
    let factories = input_lines
        .lines()
        .map(RobotFactory::from_blueprint)
        .collect::<Vec<RobotFactory>>();
    let resource_inventory = Inventory::new();
    let mut robot_inventory = Inventory::new();
    robot_inventory.add_resource(&Material::Ore, 1u32);

    let answer1 = factories
        .iter()
        .map(|factory| {
            let score =
                maximise_geodes(24, factory, resource_inventory, robot_inventory) * factory.id;
            println!("Factory: {}, Score: {}", factory.id, score);
            score
        })
        .sum::<u32>();
    let answer2 = factories
        .iter()
        .take(3)
        .map(|factory| maximise_geodes(32, factory, resource_inventory, robot_inventory))
        .product::<u32>();
    (format!("{}", answer1), format!("{}", answer2))
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum Material {
    Ore,
    Clay,
    Obsidian,
    Geode,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub struct Inventory {
    ore: u32,
    clay: u32,
    obsidian: u32,
    geode: u32,
}

impl Add for Inventory {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            ore: self.ore + other.ore,
            clay: self.clay + other.clay,
            obsidian: self.obsidian + other.obsidian,
            geode: self.geode + other.geode,
        }
    }
}

impl AddAssign for Inventory {
    fn add_assign(&mut self, other: Self) {
        self.ore += other.ore;
        self.clay += other.clay;
        self.obsidian += other.obsidian;
        self.geode += other.geode;
    }
}

impl Sub for Inventory {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            ore: self.ore - other.ore,
            clay: self.clay - other.clay,
            obsidian: self.obsidian - other.obsidian,
            geode: self.geode - other.geode,
        }
    }
}
impl SubAssign for Inventory {
    fn sub_assign(&mut self, other: Self) {
        self.ore -= other.ore;
        self.clay -= other.clay;
        self.obsidian -= other.obsidian;
        self.geode -= other.geode;
    }
}

impl Mul<u32> for Inventory {
    type Output = Self;

    fn mul(self, other: u32) -> Self {
        Self {
            ore: self.ore * other,
            clay: self.clay * other,
            obsidian: self.obsidian * other,
            geode: self.geode * other,
        }
    }
}

// impl Ord for Inventory {
//     fn cmp(&self, other: &Self) -> std::cmp::Ordering {
//         if self.geode == other.geode
//             && self.ore == other.ore
//             && self.clay == other.clay
//             && self.obsidian == other.obsidian
//         {
//             std::cmp::Ordering::Equal
//         } else if self.geode >= other.geode
//             && self.ore >= other.ore
//             && self.clay >= other.clay
//             && self.obsidian >= other.obsidian
//         {
//             std::cmp::Ordering::Greater
//         } else {
//             std::cmp::Ordering::Less
//         }
//     }
// }

impl PartialOrd for Inventory {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self.geode == other.geode
            && self.ore == other.ore
            && self.clay == other.clay
            && self.obsidian == other.obsidian
        {
            Some(std::cmp::Ordering::Equal)
        } else if self.geode >= other.geode
            && self.ore >= other.ore
            && self.clay >= other.clay
            && self.obsidian >= other.obsidian
        {
            Some(std::cmp::Ordering::Greater)
        } else if self.geode <= other.geode
            && self.ore <= other.ore
            && self.clay <= other.clay
            && self.obsidian <= other.obsidian
        {
            Some(std::cmp::Ordering::Less)
        } else {
            None
        }
    }
}

impl Inventory {
    pub fn new() -> Self {
        Self {
            ore: 0,
            clay: 0,
            obsidian: 0,
            geode: 0,
        }
    }

    pub fn from_vec(items: Vec<(&Material, u32)>) -> Self {
        let mut inventory = Inventory::new();
        for (material, value) in items {
            match *material {
                Material::Ore => inventory.ore = value,
                Material::Clay => inventory.clay = value,
                Material::Obsidian => inventory.obsidian = value,
                Material::Geode => inventory.geode = value,
            }
        }
        inventory
    }

    pub fn iter(&self) -> impl Iterator<Item = (Material, &u32)> {
        vec![
            (Material::Ore, &self.ore),
            (Material::Clay, &self.clay),
            (Material::Obsidian, &self.obsidian),
            (Material::Geode, &self.geode),
        ]
        .into_iter()
    }

    pub fn resource_stock(&self, resource: &Material) -> u32 {
        match resource {
            Material::Clay => self.clay,
            Material::Geode => self.geode,
            Material::Obsidian => self.obsidian,
            Material::Ore => self.ore,
        }
    }

    pub fn add_resource(&mut self, resource: &Material, count: u32) {
        match resource {
            Material::Ore => self.ore += count,
            Material::Clay => self.clay += count,
            Material::Obsidian => self.obsidian += count,
            Material::Geode => self.geode += count,
        }
    }

    pub fn set_resource_stock(&mut self, resource: &Material, count: u32) {
        match resource {
            Material::Ore => self.ore = count,
            Material::Clay => self.clay = count,
            Material::Obsidian => self.obsidian = count,
            Material::Geode => self.geode = count,
        }
    }
}

#[derive(Clone, Debug, Hash, Eq, PartialEq)]
pub struct ProductionState {
    time_remaining: u32,
    robot_inventory: Inventory,
    resource_inventory: Inventory,
}

impl Ord for ProductionState {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.resource_inventory
            .geode
            .cmp(&other.resource_inventory.geode)
    }
}

impl PartialOrd for ProductionState {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl ProductionState {
    fn max_possible_geodes(&self) -> u32 {
        // If we produced a Geode robot every minute, how many Geodes would we produce
        self.resource_inventory.resource_stock(&Material::Geode)
            + self.robot_inventory.resource_stock(&Material::Geode) * self.time_remaining
            + self.time_remaining * (self.time_remaining - 1) / 2
    }
}

#[derive(Debug)]
pub struct RobotFactory {
    id: u32,
    price_list: HashMap<Material, Inventory>,
    robot_maximums: Inventory,
}

impl RobotFactory {
    pub fn from_blueprint(blueprint: &str) -> Self {
        let (id, prices) = blueprint.split_once(':').unwrap();
        let id = id
            .split_once("Blueprint ")
            .unwrap()
            .1
            .parse::<u32>()
            .unwrap();
        let robot_regex =
            Regex::new(r"Each (\w+) robot costs (\d+) (\w+)(?: and (\d+) (\w+))?").unwrap();
        let mut robot_maximums = Inventory::new();
        let price_list: HashMap<Material, Inventory> = robot_regex
            .captures_iter(prices)
            .map(|captures| {
                let robot_type = match captures.get(1).unwrap().as_str() {
                    "ore" => Material::Ore,
                    "clay" => Material::Clay,
                    "obsidian" => Material::Obsidian,
                    "geode" => Material::Geode,
                    _ => panic!("Invalid robot type"),
                };
                let mut robot_costs = Inventory::new();
                let mut i = 2;
                while let Some(cost) = captures.get(i) {
                    let cost = cost.as_str().parse::<u32>().unwrap();
                    let resource = match captures.get(i + 1).unwrap().as_str() {
                        "ore" => Material::Ore,
                        "clay" => Material::Clay,
                        "obsidian" => Material::Obsidian,
                        "geode" => Material::Geode,
                        _ => panic!("Invalid resource type"),
                    };
                    robot_costs.add_resource(&resource, cost);

                    // We can only build one robot a minute, so it never makes sense to be producing
                    // more of a resource than we could consume on a single robot.
                    robot_maximums.set_resource_stock(
                        &resource,
                        cost.max(robot_maximums.resource_stock(&resource)),
                    );
                    i += 2;
                }
                (robot_type, robot_costs)
            })
            .collect();
        Self {
            id,
            price_list,
            robot_maximums,
        }
    }

    #[allow(dead_code)]
    pub fn affordable_robots(&self, resources: &Inventory) -> Vec<&Material> {
        let mut affordable_bots = vec![];
        for (robot_type, cost) in self.price_list.iter() {
            if resources >= cost {
                affordable_bots.push(robot_type);
            }
        }
        affordable_bots
    }

    pub fn buildable_robots(&self, available_robots: &Inventory) -> Vec<&Material> {
        // We can only gather resources that we have robots for. A robot is "buildable" if we have robots to gather all the resources it needs.
        let mut buildable_bots = vec![];
        // println!("{:?}", available_robots);
        for (robot_type, cost) in self.price_list.iter() {
            if cost.iter().all(|(resource, cost)| {
                if *cost > 0 {
                    available_robots.resource_stock(&resource) > 0
                } else {
                    true
                }
            }) {
                buildable_bots.push(robot_type);
            }
        }
        buildable_bots
    }

    #[allow(dead_code)]
    // This helpful factory will tell you every possible thing you can buy if you hand over your wallet :)
    pub fn possible_robots(&self, resources: &Inventory) -> Vec<Inventory> {
        let robot_purchase_order = Inventory::new();
        self.affordable_robot_combos(resources, &robot_purchase_order)
    }

    #[allow(dead_code)]
    fn affordable_robot_combos(
        &self,
        resources: &Inventory,
        current_combo: &Inventory,
    ) -> Vec<Inventory> {
        let mut results = vec![];
        for (robot_type, cost) in self.price_list.iter() {
            if resources >= cost {
                let new_resources = *resources - *cost;
                let mut new_combo = *current_combo;
                new_combo.add_resource(robot_type, 1);
                results.push(new_combo);
                results.extend(self.affordable_robot_combos(&new_resources, &new_combo));
            }
        }
        results
    }

    pub fn robot_cost(&self, robot_type: &Material) -> &Inventory {
        self.price_list.get(robot_type).unwrap()
    }
}

pub fn maximise_geodes(
    time_limit: u32,
    factory: &RobotFactory,
    resource_inventory: Inventory,
    robot_inventory: Inventory,
) -> u32 {
    let mut seen = HashSet::new();
    let mut queue = VecDeque::new();
    queue.push_back(ProductionState {
        time_remaining: time_limit,
        robot_inventory,
        resource_inventory,
    });
    let mut best = ProductionState {
        time_remaining: 0,
        robot_inventory: Inventory::new(),
        resource_inventory: Inventory::new(),
    };
    let mut count = 0;

    while let Some(production_state) = queue.pop_front() {
        count += 1;
        if count % 10000 == 0 {
            println!(
                "factory: {}, count: {}, t: {}, q_len: {}, max_geodes: {}",
                &factory.id,
                count,
                production_state.time_remaining,
                queue.len(),
                best.resource_inventory.resource_stock(&Material::Geode)
            );
        }
        if production_state.time_remaining == 0 {
            if production_state > best {
                best = production_state;
            }
            continue;
        } else {
            if production_state.max_possible_geodes()
                <= best.resource_inventory.resource_stock(&Material::Geode)
            {
                continue;
            }
            if production_state > best {
                best = production_state.clone();
            }

            // If we check every minute what robots we can afford and choose to build or not build them, we'll end up with huge numbers of states
            // where we wait one turn and then build a robot. This is a waste of time. Instead, we can check what robots are possible to build and then
            // fast forward to that state.
            let buildable_robots = factory.buildable_robots(&production_state.robot_inventory);

            for robot in buildable_robots {
                if production_state.robot_inventory.resource_stock(robot)
                    >= factory.robot_maximums.resource_stock(robot)
                    && robot != &Material::Geode
                {
                    // We don't need any more of these robots.
                    continue;
                }
                let mut future_state = production_state.clone();
                let robot_cost = factory.robot_cost(robot);
                while future_state.resource_inventory.partial_cmp(robot_cost)
                    != Some(std::cmp::Ordering::Greater)
                    && future_state.resource_inventory.partial_cmp(robot_cost)
                        != Some(std::cmp::Ordering::Equal)
                    && future_state.time_remaining > 0
                {
                    future_state = mine(future_state);
                    future_state.time_remaining -= 1;
                }
                if future_state.time_remaining == 0 {
                    if future_state > best {
                        best = future_state;
                    }
                    continue;
                }
                // We fast forward to the start of a turn, so should mine, then build the robot.
                future_state = mine(future_state);
                future_state = build(future_state, Inventory::from_vec(vec![(robot, 1)]), factory);
                future_state.time_remaining -= 1;
                if seen.insert(future_state.clone()) {
                    queue.push_back(future_state);
                }
            }
        }
    }
    best.resource_inventory.resource_stock(&Material::Geode)
}

pub fn mine(mut state: ProductionState) -> ProductionState {
    state.resource_inventory += state.robot_inventory;
    state
}

pub fn build(
    mut state: ProductionState,
    robots_to_build: Inventory,
    robot_factory: &RobotFactory,
) -> ProductionState {
    state.robot_inventory += robots_to_build;
    for (robot, count) in robots_to_build.iter() {
        if count == &0 {
            continue;
        }
        state.resource_inventory -= *robot_factory.robot_cost(&robot) * *count;
    }
    state
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_day19_part1_case1() {
        assert_eq!(day19("Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.
Blueprint 2: Each ore robot costs 2 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 8 clay. Each geode robot costs 3 ore and 12 obsidian."
).0, "33".to_string())
    }

    #[test]
    fn check_day19_part2_case1() {
        assert_eq!(day19("Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.
Blueprint 2: Each ore robot costs 2 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 8 clay. Each geode robot costs 3 ore and 12 obsidian."
).1, "3472".to_string())
    }

    #[test]
    fn check_day19_both_case1() {
        assert_eq!(day19("Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.
Blueprint 2: Each ore robot costs 2 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 8 clay. Each geode robot costs 3 ore and 12 obsidian."
), ("33".to_string(), "3472".to_string()))
    }
}
