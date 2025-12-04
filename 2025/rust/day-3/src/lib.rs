pub mod solution {
    use math::POWERS_OF_10;

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

    #[tracing::instrument(skip(input))]
    pub fn part_b(input: &str) -> anyhow::Result<String> {
        let total: usize = input.lines().map(jolts_b).sum();
        Ok(total.to_string())
    }

    pub(super) fn jolts_b(line: &str) -> usize {
        let res = [0usize; 12];
        let chars: Vec<_> = line.char_indices().collect();
        let mut jolts = 0;
        let mut start = 0;

        for i in 0..res.len() {
            let from = start.min(chars.len() - 1);
            let (max_i, max_c) = chars
                .iter()
                .skip(from)
                .take(chars.len() - (res.len() - i) - from + 1)
                // take first max index instead of last
                .max_by(|a, b| a.1.cmp(&b.1).then_with(|| b.0.cmp(&a.0)))
                .expect("More than 1 char");
            let num = max_c.to_digit(10).expect("Is number") as u64;
            jolts += (num * POWERS_OF_10[res.len() - i - 1]) as usize;
            start = *max_i + 1;
        }

        jolts
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;
    use tracing_test::traced_test;

    const TEST_INPUT: &str = include_str!("../inputs/example.txt");
    const EXPECTED_A: &str = "357";
    const EXPECTED_B: &str = "3121910778619";

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

    #[rstest]
    #[case("987654321111111", 987654321111)]
    #[case("811111111111119", 811111111119)]
    #[case("234234234234278", 434234234278)]
    #[case("818181911112111", 888911112111)]
    #[traced_test]
    fn day_3_b_jolts(#[case] line: &str, #[case] expected_joltage: usize) {
        assert_eq!(expected_joltage, solution::jolts_b(line))
    }
}
