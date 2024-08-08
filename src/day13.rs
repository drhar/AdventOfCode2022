use serde_json::Value;
use std::cmp::Ordering;

pub fn day13(input_lines: &str) -> (String, String) {
    let mut packets = input_lines
        .lines()
        .filter(|line| !line.is_empty())
        .map(Packet::from_str)
        .collect::<Vec<Packet>>();

    let packet_pairs = packets.chunks(2).map(|pair| {
        let left = &pair[0];
        let right = &pair[1];
        left.cmp(right)
    });

    let answer1 = packet_pairs
        .enumerate()
        .filter_map(|(index, ordering)| match ordering {
            Ordering::Less => Some(index + 1),
            _ => None,
        })
        .sum::<usize>();

    let dividor1 = Packet::from_str("[[2]]");
    let dividor2 = Packet::from_str("[[6]]");
    packets.push(dividor1.clone());
    packets.push(dividor2.clone());
    packets.sort_unstable();
    let answer2 = packets
        .iter()
        .enumerate()
        .filter_map(|(index, packet)| {
            if packet == &dividor1 || packet == &dividor2 {
                Some(index + 1)
            } else {
                None
            }
        })
        .product::<usize>();
    (format!("{}", answer1), format!("{}", answer2))
}

#[derive(PartialEq, Eq, Clone)]
pub struct Packet(Value);

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        match (&self.0, &other.0) {
            (Value::Number(left), Value::Number(right)) => {
                left.as_i64().unwrap().cmp(&right.as_i64().unwrap())
            }
            (Value::Number(left), Value::Array(right)) => {
                Packet::compare_packet_lists(&vec![Value::Number(left.clone())], right)
            }
            (Value::Array(left), Value::Number(right)) => {
                Packet::compare_packet_lists(left, &vec![Value::Number(right.clone())])
            }
            (Value::Array(left), Value::Array(right)) => Packet::compare_packet_lists(left, right),
            (value1, value2) => panic!("Unexpected packet pair: {:?}, {:?}", value1, value2),
        }
    }
}

impl Packet {
    pub fn from_str(packet: &str) -> Self {
        Packet(serde_json::from_str(packet).unwrap())
    }

    fn compare_packet_lists(left: &Vec<Value>, right: &Vec<Value>) -> Ordering {
        for (left, right) in left.iter().zip(right.iter()) {
            let l = Packet(left.clone());
            let r = Packet(right.clone());
            match l.cmp(&r) {
                Ordering::Less => return Ordering::Less,
                Ordering::Greater => return Ordering::Greater,
                Ordering::Equal => continue,
            }
        }
        // We've reached the end of at least one list without conclusive packet ordering.
        left.len().cmp(&right.len())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_day13_part1_case1() {
        assert_eq!(
            day13(
                "[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]"
            )
            .0,
            "13".to_string()
        )
    }

    #[test]
    fn check_day13_part2_case1() {
        assert_eq!(
            day13(
                "[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]"
            )
            .1,
            "140".to_string()
        )
    }

    #[test]
    fn check_day13_both_case1() {
        assert_eq!(
            day13(
                "[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]"
            ),
            ("13".to_string(), "140".to_string())
        )
    }
}
