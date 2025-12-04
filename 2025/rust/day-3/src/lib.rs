pub mod solution {
    #[tracing::instrument(skip(input))]
    pub fn part_a(input: &str) -> anyhow::Result<String> {
        let total: usize = input.lines().map(jolts_a).sum();
        Ok(total.to_string())
    }

    pub(super) fn jolts_a(line: &str) -> usize {
        let chars: Vec<_> = line.char_indices().collect();
        let (first_i, first_c) = chars
            .iter()
            .take(chars.len() - 1)
            // take first max index instead of last
            .max_by(|a, b| a.1.cmp(&b.1).then_with(|| b.0.cmp(&a.0)))
            .expect("More than 1 char");
        let first_num = first_c.to_digit(10).expect("Is number");
        let second_num = chars
            .iter()
            .skip(*first_i + 1)
            .max_by_key(|(_, c)| *c)
            .expect("Should be at least 1 more char after first")
            .1
            .to_digit(10)
            .expect("Is number");

        (first_num * 10 + second_num) as usize
    }

    // pub(super) fn jolts_a(line: &str) -> usize {
    //     let mut chars: Vec<_> = line.char_indices().collect();
    //     chars.sort_by_key(|(_, c)| Reverse(*c));
    //     let mut top_chars: Vec<_> = chars.iter().take(2).collect();
    //     top_chars.sort_by_key(|(i, _)| i);
    //     let str: String = top_chars.into_iter().map(|(_, c)| c).collect();
    //     str.parse().expect("Valid num")
    // }

    #[tracing::instrument(skip(input))]
    pub fn part_b(input: &str) -> anyhow::Result<String> {
        todo!("b")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;
    use tracing_test::traced_test;

    const TEST_INPUT: &str = include_str!("../inputs/example.txt");
    const EXPECTED_A: &str = "357";
    const EXPECTED_B: &str = "todo_expected_b";

    #[test]
    #[traced_test]
    fn day_3_a() {
        let res = solution::part_a(TEST_INPUT);
        assert_eq!(EXPECTED_A, res.unwrap());
    }

    #[test]
    #[traced_test]
    fn day_3_b() {
        let res = solution::part_b(TEST_INPUT);
        assert_eq!(EXPECTED_B, res.unwrap());
    }

    #[rstest]
    #[case("987654321111111", 98)]
    #[case("811111111111119", 89)]
    #[case("234234234234278", 78)]
    #[case("818181911112111", 92)]
    #[case("8786", 88)]
    #[traced_test]
    fn day_3_a_jolts(#[case] line: &str, #[case] expected_joltage: usize) {
        assert_eq!(expected_joltage, solution::jolts_a(line))
    }
}
