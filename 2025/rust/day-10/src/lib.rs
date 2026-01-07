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
        pub lights: u16,
        pub buttons: Vec<u16>,
        pub joltage: Vec<u16>,
    }
    impl Machine {
        pub(super) fn min_presses(&self) -> usize {
            for combo_size in 1..=32 {
                if self
                    .buttons
                    .iter()
                    .combinations_with_replacement(combo_size)
                    .any(|buttons| {
                        buttons.iter().fold::<u16, _>(0, |acc, btn_bitmap| {
                            // tracing::warn!(
                            //     ?buttons,
                            //     lights = self.lights,
                            //     acc,
                            //     btn_bitmap,
                            //     new = acc ^ **btn_bitmap
                            // );
                            acc ^ **btn_bitmap
                        }) == self.lights
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
                lights,
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

        pub(super) fn parse_btn(input: &str) -> IResult<&str, u16> {
            let (rest, set_bits) = delimited(
                char('('),
                separated_list1(char(','), character::complete::u16),
                char(')'),
            )(input)?;
            let mask = set_bits.into_iter().fold(0, |acc, bit_i| acc | 1 << bit_i);
            Ok((rest, mask))
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
            .map(|machine| machine.min_presses())
            .sum::<usize>();
        Ok(res.to_string())
    }

    #[tracing::instrument(skip(input))]
    pub fn part_b(input: &str) -> anyhow::Result<String> {
        todo!("b")
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
    const EXPECTED_B: &str = "todo_expected_b";

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

    #[test_case("(3)" => 0b1000)]
    #[test_case("(0,1)" => 0b0011)]
    #[test_case("(1,3)" => 0b1010)]
    #[traced_test]
    fn parse_btn(input: &str) -> u16 {
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
                    lights: 0b0110,
                    buttons: vec![0b1000, 0b1010, 0b0100, 0b1100, 0b0101, 0b0011],
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
                    lights: 0b1000,
                    buttons: vec![0b0001],
                    joltage: vec![1]
                },
                Machine {
                    lights: 0b0100,
                    buttons: vec![0b0010],
                    joltage: vec![2]
                }
            ],
            machines
        );
    }

    #[test_case(
        Machine {
            lights: 6,
            buttons: vec![8, 10, 4, 12, 5, 3],
            joltage: vec![3, 5, 4, 7]
        } => 2; "row 1")]
    #[test_case(
        Machine {
            lights: 8,
            buttons: vec![29, 12, 17, 7, 30],
            joltage: vec![7, 5, 12, 7, 2]
        } => 3; "row 2")]
    #[test_case(
        Machine {
            lights: 46,
            buttons: vec![31, 25, 55, 6],
            joltage: vec![10, 11, 11, 5, 10, 5]
        } => 2; "row 3")]
    #[traced_test]
    fn min_presses(machine: Machine) -> usize {
        machine.min_presses()
    }
    //

    // [.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
    // todo tests:
    // parser
    // bitmask conversion
    // applying a bitmask/button
    // resolve single line/machine
}
