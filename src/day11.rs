use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{char, digit1, line_ending, multispace0, space0, space1},
    error::ParseError,
    multi::{many0, many1, separated_list1},
    sequence::delimited,
    IResult, Parser,
};

pub fn day11(input_lines: &str) -> (String, String) {
    let pt1_monkeys = run_simulation(input_lines, 20, 3);
    let pt2_monkeys = run_simulation(input_lines, 10000, 1);
    let answer1 = pt1_monkeys[0].inspection_count * pt1_monkeys[1].inspection_count;
    let answer2 = pt2_monkeys[0].inspection_count * pt2_monkeys[1].inspection_count;
    (format!("{}", answer1), format!("{}", answer2))
}

pub struct Monkey {
    id: usize,
    items: Vec<u64>,
    operation: Box<dyn Fn(u64) -> u64>,
    test_divisor: u64,
    throw_true: usize,
    throw_false: usize,
    inspection_count: u64,
}

impl Monkey {
    pub fn test(&self, item: u64) -> bool {
        item % self.test_divisor == 0
    }

    pub fn inspect_and_throw(
        &mut self,
        relief_factor: u64,
        worry_filter: u64,
    ) -> Vec<(usize, u64)> {
        let mut thrown_items = Vec::new();
        while !self.items.is_empty() {
            self.inspection_count += 1;
            // We could keep the total worry amount by keeping a count of the times the worry_filter has
            // reduced the worry, but don't need to for the puzzle so leaving it as an extension
            let item = self.items.pop().unwrap() % worry_filter;
            let item = (self.operation)(item) / relief_factor;
            if self.test(item) {
                thrown_items.push((self.throw_true, item));
            } else {
                thrown_items.push((self.throw_false, item));
            }
        }
        thrown_items
    }
}

pub fn run_simulation(input_lines: &str, rounds: i32, relief_factor: u64) -> Vec<Monkey> {
    let (_, mut monkeys) = many1(parse_monkey)(input_lines).unwrap();
    monkeys.sort_unstable_by(|a, b| a.id.cmp(&b.id));

    // All tests are "is divisible", we can therefore work out a "filter" where if the worry would pass all
    // tests, we can throw that amount of worry away. Applying the filter looks like taking the modulo of the worry
    // with the filter. The filter is the product of the divisors.
    // e.g. if divisors are 3 & 5, filter is 15. Applying div 5 or div 3 tests to anything % filter is the same as
    // applying it directly (e.g. if worry is 32, filtered worry is 32 & 15 = 2. 32 % 3 = 2, 2 % 3 = 2, 32 % 5 = 2, 2 % 5 = 2)
    let worry_filter = monkeys.iter().map(|m| m.test_divisor).product::<u64>();
    assert_eq!(monkeys[0].id, 0);
    for _round in 0..rounds {
        for monkey_idx in 0..monkeys.len() {
            for (destination, item) in
                monkeys[monkey_idx].inspect_and_throw(relief_factor, worry_filter)
            {
                monkeys[destination].items.push(item);
            }
        }
    }
    monkeys.sort_unstable_by(|a, b| b.inspection_count.cmp(&a.inspection_count));
    monkeys
}

pub fn parse_monkey(buf: &str) -> IResult<&str, Monkey> {
    let (buf, _) = remove_surr_whitespace(tag("Monkey ")).parse(buf)?;
    let (buf, id) = digit1(buf)?;
    let (buf, _) = char(':')(buf)?;
    let (buf, _) = remove_surr_whitespace(tag("Starting items:")).parse(buf)?;
    let (buf, items) = remove_surr_whitespace(separated_list1(tag(", "), digit1)).parse(buf)?;
    let (buf, operation) = parse_operation(buf)?;
    let (buf, _) = remove_surr_whitespace(tag("Test: divisible by ")).parse(buf)?;
    let (buf, test_divisor) = digit1(buf)?;
    let (buf, _) = remove_surr_whitespace(tag("If true: throw to monkey ")).parse(buf)?;
    let (buf, throw_true) = digit1(buf)?;
    let (buf, _) = remove_surr_whitespace(tag("If false: throw to monkey ")).parse(buf)?;
    let (buf, throw_false) = remove_surr_whitespace(digit1).parse(buf)?;
    Ok((
        buf,
        Monkey {
            id: id.parse::<usize>().unwrap(),
            items: items
                .iter()
                .map(|item| item.parse::<u64>().unwrap())
                .collect(),
            operation,
            test_divisor: test_divisor.parse::<u64>().unwrap(),
            throw_true: throw_true.parse::<usize>().unwrap(),
            throw_false: throw_false.parse::<usize>().unwrap(),
            inspection_count: 0,
        },
    ))
}

pub fn parse_operation(buf: &str) -> IResult<&str, Box<dyn Fn(u64) -> u64>> {
    let (buf, _) = remove_surr_whitespace(tag("Operation: new = ")).parse(buf)?;
    let (buf, operand1) = alt((digit1, tag("old")))(buf)?;
    let (buf, operator) =
        remove_surr_whitespace(alt((tag("+"), tag("-"), tag("*"), tag("/")))).parse(buf)?;
    let (buf, operand2) = remove_surr_whitespace(alt((digit1, tag("old")))).parse(buf)?;
    let operand1 = operand1.to_string();
    let operand2 = operand2.to_string();
    let operator = operator.to_string();
    Ok((
        buf,
        Box::new(move |old: u64| {
            let operand1 = match operand1.as_str() {
                "old" => old,
                _ => operand1.parse::<u64>().unwrap(),
            };
            let operand2 = match operand2.as_str() {
                "old" => old,
                _ => operand2.parse::<u64>().unwrap(),
            };
            match operator.as_str() {
                "+" => operand1 + operand2,
                "-" => operand1 - operand2,
                "*" => operand1 * operand2,
                "/" => operand1 / operand2,
                operator => panic!("Unknown operator: {}", operator),
            }
        }),
    ))
}

// For a parser 'inner' return a parser that consumes (and discards) leading and trailing whitespace and newlines either side of 'inner'
pub fn remove_surr_whitespace<'a, O, E: ParseError<&'a str>, F>(
    inner: F,
) -> (impl Parser<&'a str, O, E>)
where
    F: Parser<&'a str, O, E>,
{
    delimited(multispace0, inner, multispace0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_day11_part1_case1() {
        assert_eq!(
            day11(
                "Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1"
            )
            .0,
            "10605".to_string()
        )
    }

    #[test]
    fn check_day11_part2_case1() {
        assert_eq!(
            day11(
                "Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1"
            )
            .1,
            "2713310158".to_string()
        )
    }

    #[test]
    fn check_day11_both_case1() {
        assert_eq!(
            day11(
                "Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1"
            ),
            ("10605".to_string(), "2713310158".to_string())
        )
    }
}
