pub fn day25(input_lines: &str) -> (String, String) {
    let total = input_lines.lines().map(SnafuConverter::to_decimal).sum();
    let answer1 = SnafuConverter::to_snafu(total);
    let answer2 = 0;
    (answer1.to_string(), format!("{}", answer2))
}

pub struct SnafuConverter;

impl SnafuConverter {
    pub fn to_decimal(snafu: &str) -> i64 {
        snafu.chars().rev().enumerate().fold(0, |acc, (i, c)| {
            let value = match c {
                '0' => 0,
                '1' => 5i64.pow(i as u32),
                '2' => 2 * 5i64.pow(i as u32),
                '-' => -(5i64.pow(i as u32)),
                '=' => -(2 * 5i64.pow(i as u32)),
                d => panic!("Invalid SNAFU character: {}", d),
            };
            acc + value
        })
    }

    pub fn largest(sig_figs: u32) -> i64 {
        (0..sig_figs).fold(0, |acc, i| acc + 2 * 5i64.pow(i))
    }

    pub fn to_snafu(dec_value: i64) -> String {
        let mut sig_figs = 1;
        while (0..(sig_figs)).map(|i| 2 * 5i64.pow(i)).sum::<i64>() < dec_value.abs() {
            sig_figs += 1;
        }

        (0..sig_figs)
            .rev()
            .scan(0, |value, index| {
                let mut digit = -2;
                while digit <= 2
                    && *value + (digit * 5i64.pow(index)) + SnafuConverter::largest(index)
                        < dec_value.abs()
                {
                    digit += 1;
                }
                *value += digit * 5i64.pow(index);

                match digit {
                    -2 => Some(if dec_value.signum() == -1 { '2' } else { '=' }),
                    -1 => Some(if dec_value.signum() == -1 { '1' } else { '-' }),
                    0 => Some('0'),
                    1 => Some(if dec_value.signum() == -1 { '-' } else { '1' }),
                    2 => Some(if dec_value.signum() == -1 { '=' } else { '2' }),
                    d => panic!("Incorrect digit value: {d}"),
                }
            })
            .collect::<String>()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_snafu_to_decimal() {
        assert_eq!(SnafuConverter::to_decimal("0"), 0);
        assert_eq!(SnafuConverter::to_decimal("1"), 1);
        assert_eq!(SnafuConverter::to_decimal("2"), 2);
        assert_eq!(SnafuConverter::to_decimal("-"), -1);
        assert_eq!(SnafuConverter::to_decimal("="), -2);
        assert_eq!(SnafuConverter::to_decimal("10"), 5);
        assert_eq!(SnafuConverter::to_decimal("11"), 6);
        assert_eq!(SnafuConverter::to_decimal("12"), 7);
        assert_eq!(SnafuConverter::to_decimal("20"), 10);
        assert_eq!(SnafuConverter::to_decimal("21"), 11);
        assert_eq!(SnafuConverter::to_decimal("22"), 12);
        assert_eq!(SnafuConverter::to_decimal("1-"), 4);
        assert_eq!(SnafuConverter::to_decimal("1="), 3);
        assert_eq!(SnafuConverter::to_decimal("2-"), 9);
        assert_eq!(SnafuConverter::to_decimal("2="), 8);
        assert_eq!(SnafuConverter::to_decimal("100"), 25);
        assert_eq!(SnafuConverter::to_decimal("101"), 26);
        assert_eq!(SnafuConverter::to_decimal("110"), 30);
        assert_eq!(SnafuConverter::to_decimal("111"), 31);
        assert_eq!(SnafuConverter::to_decimal("112"), 32);
        assert_eq!(SnafuConverter::to_decimal("1-0"), 20);
        assert_eq!(SnafuConverter::to_decimal("1-1"), 21);
        assert_eq!(SnafuConverter::to_decimal("1-2"), 22);
        assert_eq!(SnafuConverter::to_decimal("1=0"), 15);
        assert_eq!(SnafuConverter::to_decimal("1=1"), 16);
        assert_eq!(SnafuConverter::to_decimal("1=2"), 17);
        assert_eq!(SnafuConverter::to_decimal("200"), 50);
        assert_eq!(SnafuConverter::to_decimal("201"), 51);
        assert_eq!(SnafuConverter::to_decimal("210"), 55);
        assert_eq!(SnafuConverter::to_decimal("211"), 56);
        assert_eq!(SnafuConverter::to_decimal("212"), 57);
        assert_eq!(SnafuConverter::to_decimal("2-0"), 45);
        assert_eq!(SnafuConverter::to_decimal("2-1"), 46);
        assert_eq!(SnafuConverter::to_decimal("2-2"), 47);
        assert_eq!(SnafuConverter::to_decimal("2=0"), 40);
        assert_eq!(SnafuConverter::to_decimal("2=1"), 41);
        assert_eq!(SnafuConverter::to_decimal("2=2"), 42);
    }

    #[test]
    fn check_decimal_to_snafu() {
        assert_eq!(SnafuConverter::to_snafu(0), "0");
        assert_eq!(SnafuConverter::to_snafu(1), "1");
        assert_eq!(SnafuConverter::to_snafu(2), "2");
        assert_eq!(SnafuConverter::to_snafu(-1), "-");
        assert_eq!(SnafuConverter::to_snafu(-2), "=");
        assert_eq!(SnafuConverter::to_snafu(5), "10");
        assert_eq!(SnafuConverter::to_snafu(6), "11");
        assert_eq!(SnafuConverter::to_snafu(7), "12");
        assert_eq!(SnafuConverter::to_snafu(10), "20");
        assert_eq!(SnafuConverter::to_snafu(11), "21");
        assert_eq!(SnafuConverter::to_snafu(12), "22");
        assert_eq!(SnafuConverter::to_snafu(4), "1-");
        assert_eq!(SnafuConverter::to_snafu(3), "1=");
        assert_eq!(SnafuConverter::to_snafu(9), "2-");
        assert_eq!(SnafuConverter::to_snafu(8), "2=");
        assert_eq!(SnafuConverter::to_snafu(25), "100");
        assert_eq!(SnafuConverter::to_snafu(26), "101");
        assert_eq!(SnafuConverter::to_snafu(30), "110");
        assert_eq!(SnafuConverter::to_snafu(31), "111");
        assert_eq!(SnafuConverter::to_snafu(32), "112");
        assert_eq!(SnafuConverter::to_snafu(20), "1-0");
        assert_eq!(SnafuConverter::to_snafu(21), "1-1");
        assert_eq!(SnafuConverter::to_snafu(22), "1-2");
        assert_eq!(SnafuConverter::to_snafu(15), "1=0");
        assert_eq!(SnafuConverter::to_snafu(16), "1=1");
        assert_eq!(SnafuConverter::to_snafu(17), "1=2");
        assert_eq!(SnafuConverter::to_snafu(50), "200");
        assert_eq!(SnafuConverter::to_snafu(51), "201");
        assert_eq!(SnafuConverter::to_snafu(55), "210");
        assert_eq!(SnafuConverter::to_snafu(56), "211");
        assert_eq!(SnafuConverter::to_snafu(57), "212");
        assert_eq!(SnafuConverter::to_snafu(45), "2-0");
        assert_eq!(SnafuConverter::to_snafu(46), "2-1");
        assert_eq!(SnafuConverter::to_snafu(47), "2-2");
        assert_eq!(SnafuConverter::to_snafu(40), "2=0");
        assert_eq!(SnafuConverter::to_snafu(41), "2=1");
        assert_eq!(SnafuConverter::to_snafu(42), "2=2");
    }

    #[test]
    fn check_day25_part1_case1() {
        assert_eq!(
            day25(
                "1=-0-2
12111
2=0=
21
2=01
111
20012
112
1=-1=
1-12
12
1=
122"
            )
            .0,
            "2=-1=0".to_string()
        )
    }

    #[test]
    fn check_day25_part2_case1() {
        assert_eq!(
            day25(
                "1=-0-2
12111
2=0=
21
2=01
111
20012
112
1=-1=
1-12
12
1=
122"
            )
            .1,
            "0".to_string()
        )
    }

    #[test]
    fn check_day25_both_case1() {
        assert_eq!(
            day25(
                "1=-0-2
12111
2=0=
21
2=01
111
20012
112
1=-1=
1-12
12
1=
122"
            ),
            ("2=-1=0".to_string(), "0".to_string())
        )
    }
}
