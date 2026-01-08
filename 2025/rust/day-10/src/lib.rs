pub mod solution {
    use itertools::Itertools;
    use nom::{
        IResult,
        branch::alt,
        character::{
            self,
            complete::{char, multispace1},
        },
        multi::{many1, separated_list1},
        sequence::{delimited, tuple},
    };

    #[derive(Debug, PartialEq, Eq)]
    pub(super) struct Machine {
        pub light_mask: u16,
        pub buttons: Vec<Vec<u16>>,
        pub button_masks: Vec<u16>,
        pub joltage: Vec<u16>,
    }
    impl Machine {
        pub(super) fn min_presses_lights(&self) -> usize {
            for combo_size in 1..=32 {
                if self
                    .button_masks
                    .iter()
                    .combinations_with_replacement(combo_size)
                    .any(|buttons| {
                        buttons
                            .iter()
                            .fold::<u16, _>(0, |acc, btn_mask| acc ^ **btn_mask)
                            == self.light_mask
                    })
                {
                    return combo_size;
                }
            }

            panic!("Could not find combo for machine {self:?}");
        }

        // todo: this is way too slow for the actual input
        // try: switch to DFS with neibour nodes/btns ordered based on rarest joltages among them to narrow down the possibility space 
        pub(super) fn min_presses_joltage(&self) -> usize {
            let max_joltage = *self.joltage.iter().max().unwrap() as usize;
            for combo_size in max_joltage..(max_joltage * 10) {
                if self
                    .buttons
                    .iter()
                    .combinations_with_replacement(combo_size)
                    .any(|buttons| {
                        let mut joltage = self.joltage.clone();
                        for btn in buttons {
                            let mut check = false;
                            for i in btn {
                                let i = *i as usize;
                                if joltage[i] == 0 {
                                    // would go over
                                    return false;
                                }

                                let val = joltage[i] - 1;
                                joltage[i] = val;
                                if val == 0 {
                                    check = true;
                                }
                            }
                            if check && joltage.iter().all(|j| *j == 0) {
                                return true;
                            }
                        }

                        false
                    })
                {
                    return combo_size;
                }
            }

            panic!("Could not find combo for machine {self:?}");
        }

        pub(super) fn parse_list(input: &str) -> anyhow::Result<Vec<Machine>> {
            let (_, machines) = separated_list1(multispace1, Self::parse_machine)(input)
                .map_err(|e| e.to_owned())?;
            Ok(machines)
        }

        pub(super) fn parse_machine(input: &str) -> IResult<&str, Machine> {
            let btn_list = separated_list1(char(' '), Self::parse_btn);
            let (rest, (lights, buttons, joltage)) = tuple((
                Self::parse_lights,
                delimited(char(' '), btn_list, char(' ')),
                Self::parse_joltage,
            ))(input)?;
            let machine = Self {
                light_mask: lights,
                button_masks: buttons
                    .iter()
                    .map(|set_bits| set_bits.into_iter().fold(0, |acc, bit_i| acc | 1 << bit_i))
                    .collect(),
                buttons,
                joltage,
            };
            Ok((rest, machine))
        }

        pub(super) fn parse_lights(input: &str) -> IResult<&str, u16> {
            let light_pattern_parser = many1(alt((char('.'), char('#'))));
            let (rest, light_pattern) =
                delimited(char('['), light_pattern_parser, char(']'))(input)?;
            let light_pattern = light_pattern
                .into_iter()
                .map(|c| match c {
                    '.' => 0,
                    '#' => 1,
                    _ => unreachable!("Unknown char {c}"),
                })
                .rev()
                .fold(0, |acc, bit| (acc << 1) ^ bit);
            Ok((rest, light_pattern))
        }

        pub(super) fn parse_btn(input: &str) -> IResult<&str, Vec<u16>> {
            delimited(
                char('('),
                separated_list1(char(','), character::complete::u16),
                char(')'),
            )(input)
        }

        fn parse_joltage(input: &str) -> IResult<&str, Vec<u16>> {
            delimited(
                char('{'),
                separated_list1(char(','), character::complete::u16),
                char('}'),
            )(input)
        }
    }

    #[tracing::instrument(skip(input))]
    pub fn part_a(input: &str) -> anyhow::Result<String> {
        let machines = Machine::parse_list(input)?;
        let res = machines
            .into_iter()
            .map(|machine| machine.min_presses_lights())
            .sum::<usize>();
        Ok(res.to_string())
    }

    #[tracing::instrument(skip(input))]
    pub fn part_b(input: &str) -> anyhow::Result<String> {
        let machines = Machine::parse_list(input)?;
        let res = machines
            .into_iter()
            .map(|machine| machine.min_presses_joltage())
            .sum::<usize>();
        Ok(res.to_string())
    }
}

#[cfg(test)]
mod tests {
    use crate::solution::Machine;

    use super::*;
    use test_case::test_case;
    use tracing_test::traced_test;

    const TEST_INPUT: &str = include_str!("../inputs/example.txt");
    const EXPECTED_A: &str = "7";
    const EXPECTED_B: &str = "33";

    #[test]
    #[traced_test]
    fn day_10_a() {
        let res = solution::part_a(TEST_INPUT);
        assert_eq!(EXPECTED_A, res.unwrap());
    }

    #[test]
    #[traced_test]
    fn day_10_b() {
        let res = solution::part_b(TEST_INPUT);
        assert_eq!(EXPECTED_B, res.unwrap());
    }

    #[test]
    #[traced_test]
    fn parse_lights() {
        let (rest, pattern) = Machine::parse_lights("[.##.]").unwrap();
        assert_eq!(rest, "");
        assert_eq!(pattern, 0b0110);
    }

    #[test_case("(3)" => vec![3])]
    #[test_case("(0,1)" => vec![0, 1])]
    #[test_case("(1,3)" => vec![1, 3])]
    #[traced_test]
    fn parse_btn(input: &str) -> Vec<u16> {
        Machine::parse_btn(input).unwrap().1
    }

    #[test]
    #[traced_test]
    fn parse_machine() {
        let result =
            Machine::parse_machine("[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}\n").unwrap();
        assert_eq!(
            (
                "\n",
                Machine {
                    light_mask: 0b0110,
                    button_masks: vec![0b1000, 0b1010, 0b0100, 0b1100, 0b0101, 0b0011],
                    buttons: vec![
                        vec![3],
                        vec![1, 3],
                        vec![2],
                        vec![2, 3],
                        vec![0, 2],
                        vec![0, 1],
                    ],
                    joltage: vec![3, 5, 4, 7]
                }
            ),
            result
        );
    }

    #[test]
    #[traced_test]
    fn parse_input() {
        let input = "[...#] (0) {1}\n[..#.] (1) {2}";
        let machines = Machine::parse_list(input).unwrap();
        assert_eq!(
            vec![
                Machine {
                    light_mask: 0b1000,
                    button_masks: vec![0b0001],
                    buttons: vec![vec![0]],
                    joltage: vec![1]
                },
                Machine {
                    light_mask: 0b0100,
                    button_masks: vec![0b0010],
                    buttons: vec![vec![1]],
                    joltage: vec![2]
                }
            ],
            machines
        );
    }

    #[test_case(
        Machine {
            light_mask: 6,
            button_masks: vec![8, 10, 4, 12, 5, 3],
            buttons: vec![],
            joltage: vec![]
        } => 2; "lights - row 1")]
    #[test_case(
        Machine {
            light_mask: 8,
            button_masks: vec![29, 12, 17, 7, 30],
            buttons: vec![],
            joltage: vec![]
        } => 3; "lights - row 2")]
    #[test_case(
        Machine {
            light_mask: 46,
            button_masks: vec![31, 25, 55, 6],
            buttons: vec![],
            joltage: vec![]
        } => 2; "lights - row 3")]
    #[traced_test]
    fn min_presses_lights(machine: Machine) -> usize {
        machine.min_presses_lights()
    }

    #[test_case(
        Machine {
            light_mask: 6,
            button_masks: vec![8, 10, 4, 12, 5, 3],
            buttons: vec![
                vec![3],
                vec![1, 3],
                vec![2],
                vec![2, 3],
                vec![0, 2],
                vec![0, 1],
            ],
            joltage: vec![3, 5, 4, 7]
        } => 10; "joltage - row 1")]
    #[test_case(
        Machine {
            light_mask: 8,
            button_masks: vec![29, 12, 17, 7, 30],
            buttons: vec![
                vec![0, 2, 3, 4],
                vec![2, 3],
                vec![0, 4],
                vec![0, 1, 2],                
                vec![1, 2, 3, 4],
            ],
            joltage: vec![7, 5, 12, 7, 2]
        } => 12; "joltage - row 2")]
    #[test_case(
        Machine {
            light_mask: 46,
            button_masks: vec![31, 25, 55, 6],
            buttons: vec![
                vec![0, 1, 2, 3, 4],
                vec![0, 3, 4],
                vec![0, 1, 2, 4, 5], 
                vec![1, 2],                
            ],
            joltage: vec![10, 11, 11, 5, 10, 5]
        } => 11; "joltage - row 3")]
    #[traced_test]
    fn min_presses_joltage(machine: Machine) -> usize {
        machine.min_presses_joltage()
    }
}
