use std::collections::HashMap;

pub fn day21(input_lines: &str) -> (String, String) {
    let mut monkeys = Directory::from_str(input_lines, "root", "humn");
    let answer1 = monkeys.evaluate_monkey("root");
    monkeys.equalise_root();
    let answer2 = monkeys.evaluate_monkey("humn");
    (format!("{}", answer1), format!("{}", answer2))
}

pub struct Directory {
    directory: HashMap<String, Monkey>,
    root: String,
    human: String,
}

impl Directory {
    pub fn from_str(s: &str, root: &str, human: &str) -> Self {
        let directory = s
            .lines()
            .filter(|line| !line.is_empty())
            .map(Monkey::from_str)
            .map(|monkey| (monkey.name.clone(), monkey))
            .collect::<HashMap<String, Monkey>>();
        Self {
            directory,
            root: root.to_string(),
            human: human.to_string(),
        }
    }

    pub fn evaluate_monkey(&self, name: &str) -> f64 {
        let monkey = self.directory.get(name).unwrap();

        if let Some(number) = monkey.number {
            number
        } else {
            let lhs_monkey = monkey.lhs_monkey.as_ref().unwrap();
            let rhs_monkey = monkey.rhs_monkey.as_ref().unwrap();
            let lhs_monkey = self.evaluate_monkey(lhs_monkey.as_str());
            let rhs_monkey = self.evaluate_monkey(rhs_monkey.as_str());
            monkey.operation.unwrap()(lhs_monkey, rhs_monkey)
        }
    }

    pub fn equalise_root(&mut self) {
        let root = self.directory.get(&self.root).unwrap();
        let lhs = root.lhs_monkey.as_ref().unwrap().clone();
        let rhs = root.rhs_monkey.as_ref().unwrap().clone();
        let mut human_value_a = self.directory.get(&self.human).unwrap().number.unwrap();

        let mut lhs_value_a = self.evaluate_monkey(lhs.as_str());
        let mut rhs_value_a = self.evaluate_monkey(rhs.as_str());
        let mut signum_a = (lhs_value_a - rhs_value_a).signum();

        if lhs_value_a == rhs_value_a {
            return;
        }

        let mut human_value_b = 0f64;
        self.update_human(human_value_b);
        let lhs_value_b = self.evaluate_monkey(lhs.as_str());
        let rhs_value_b = self.evaluate_monkey(rhs.as_str());
        let mut signum_b = (lhs_value_b - rhs_value_b).signum();

        if lhs_value_b == rhs_value_b {
            return;
        }

        let mut count = 1;

        while signum_a == signum_b {
            count += 1;
            human_value_b = human_value_a;
            signum_b = signum_a;
            human_value_a += human_value_a;
            println!(
                "Looking for bracket. count: {}, a: {}, b: {}",
                count, human_value_a, human_value_b
            );

            self.update_human(human_value_a);
            lhs_value_a = self.evaluate_monkey(lhs.as_str());
            rhs_value_a = self.evaluate_monkey(rhs.as_str());
            signum_a = (lhs_value_a - rhs_value_a).signum();
            if lhs_value_a == rhs_value_a {
                return;
            }
        }

        count = 0;
        // We now have a bracket in which an intersection exists, so let's bisect!
        while human_value_a as i64 != human_value_b as i64 {
            let human_value_c = (human_value_a + human_value_b) / 2f64;
            println!(
                "Bisecting. count: {}, a: {}, b: {}, c: {}",
                count, human_value_a, human_value_b, human_value_c
            );
            count += 1;

            self.update_human(human_value_c);
            let lhs_value_c = self.evaluate_monkey(lhs.as_str());
            let rhs_value_c = self.evaluate_monkey(rhs.as_str());
            let signum_c = (lhs_value_c - rhs_value_c).signum();
            if lhs_value_c == rhs_value_c {
                return;
            } else if signum_c == signum_a {
                human_value_a = human_value_c;
                signum_a = signum_c;
            } else {
                human_value_b = human_value_c;
            }
        }
    }

    fn update_human(&mut self, number: f64) {
        let human = self.directory.get_mut(&self.human).unwrap();
        human.number = Some(number);
    }
}
#[derive(Debug)]
pub struct Monkey {
    name: String,
    operation: Option<fn(f64, f64) -> f64>,
    number: Option<f64>,
    lhs_monkey: Option<String>,
    rhs_monkey: Option<String>,
}

impl Monkey {
    pub fn from_str(s: &str) -> Monkey {
        let (name, operation) = s.split_once(':').unwrap();
        let operation = operation.trim();
        if let Ok(number) = operation.parse::<f64>() {
            Monkey {
                name: name.to_string(),
                operation: None,
                number: Some(number),
                lhs_monkey: None,
                rhs_monkey: None,
            }
        } else {
            let mut operation_parts = operation.split_whitespace();
            let lhs_monkey = operation_parts.next().unwrap();
            let operation: Option<fn(f64, f64) -> f64> = match operation_parts.next() {
                Some("+") => Some(add),
                Some("-") => Some(sub),
                Some("*") => Some(mul),
                Some("/") => Some(div),
                _ => panic!("Invalid operation"),
            };
            let rhs_monkey = operation_parts.next().unwrap();
            Monkey {
                name: name.to_string(),
                operation,
                number: None,
                lhs_monkey: Some(lhs_monkey.to_string()),
                rhs_monkey: Some(rhs_monkey.to_string()),
            }
        }
    }
}

pub fn add(lhs: f64, rhs: f64) -> f64 {
    lhs + rhs
}

pub fn sub(lhs: f64, rhs: f64) -> f64 {
    lhs - rhs
}

pub fn mul(lhs: f64, rhs: f64) -> f64 {
    lhs * rhs
}

pub fn div(lhs: f64, rhs: f64) -> f64 {
    lhs / rhs
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_day21_part1_case1() {
        assert_eq!(
            day21(
                "root: pppw + sjmn
dbpl: 5
cczh: sllz + lgvd
zczc: 2
ptdq: humn - dvpt
dvpt: 3
lfqf: 4
humn: 5
ljgn: 2
sjmn: drzm * dbpl
sllz: 4
pppw: cczh / lfqf
lgvd: ljgn * ptdq
drzm: hmdt - zczc
hmdt: 32"
            )
            .0,
            "152".to_string()
        )
    }

    #[test]
    fn check_day21_part2_case1() {
        assert_eq!(
            day21(
                "root: pppw + sjmn
dbpl: 5
cczh: sllz + lgvd
zczc: 2
ptdq: humn - dvpt
dvpt: 3
lfqf: 4
humn: 5
ljgn: 2
sjmn: drzm * dbpl
sllz: 4
pppw: cczh / lfqf
lgvd: ljgn * ptdq
drzm: hmdt - zczc
hmdt: 32"
            )
            .1,
            "301".to_string()
        )
    }

    #[test]
    fn check_day21_both_case1() {
        assert_eq!(
            day21(
                "root: pppw + sjmn
dbpl: 5
cczh: sllz + lgvd
zczc: 2
ptdq: humn - dvpt
dvpt: 3
lfqf: 4
humn: 5
ljgn: 2
sjmn: drzm * dbpl
sllz: 4
pppw: cczh / lfqf
lgvd: ljgn * ptdq
drzm: hmdt - zczc
hmdt: 32"
            ),
            ("152".to_string(), "301".to_string())
        )
    }
}
