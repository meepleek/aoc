pub mod solution {
    use parse::range::parse_inclusive_range;

    use crate::solution;

    #[tracing::instrument(skip(input))]
    pub fn part_a(input: &str) -> anyhow::Result<String> {
        let res: usize = input
            .split(",")
            .map(|text| {
                let range = parse_inclusive_range::<usize>(text.trim_end()).expect("valid range");
                range
                    .filter(|num| {
                        let str = num.to_string();
                        let (a, b) = str.split_at(str.len() / 2);
                        a == b
                    })
                    .sum::<usize>()
            })
            .sum();

        Ok(res.to_string())
    }

    #[tracing::instrument(skip(input))]
    pub fn part_b(input: &str) -> anyhow::Result<String> {
        let res: usize = input
            .split(",")
            .map(|text| {
                let range = parse_inclusive_range::<usize>(text.trim_end()).expect("valid range");
                range
                    .filter(|num| solution::is_invalid_b(*num))
                    .sum::<usize>()
            })
            .sum();

        Ok(res.to_string())
    }

    pub(super) fn is_invalid_b(num: usize) -> bool {
        let str = num.to_string();
        for size in 1..=(str.len() / 2) {
            let chars: Vec<_> = str.chars().collect();
            let mut chunks = chars.chunks(size).into_iter();
            let part = chunks.next().expect("first chunk");
            if chunks.all(|chunk| chunk == part) {
                return true;
            }
        }

        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;
    use tracing_test::traced_test;

    const TEST_INPUT: &str = include_str!("../inputs/example.txt");
    const EXPECTED_A: &str = "1227775554";
    const EXPECTED_B: &str = "4174379265";

    #[test]
    #[traced_test]
    fn day_2_a() {
        let res = solution::part_a(TEST_INPUT);
        assert_eq!(EXPECTED_A, res.unwrap());
    }

    #[test]
    #[traced_test]
    fn day_2_b() {
        let res = solution::part_b(TEST_INPUT);
        assert_eq!(EXPECTED_B, res.unwrap());
    }

    #[rstest]
    #[case(11)]
    #[case(22)]
    #[case(111)]
    #[case(11111)]
    #[case(1010)]
    #[case(1188511885)]
    #[case(446446)]
    #[case(38593859)]
    #[traced_test]
    fn day_2_b_is_invalid_true(#[case] num: usize) {
        assert!(solution::is_invalid_b(num))
    }

    #[rstest]
    #[case(1)]
    #[case(12)]
    #[case(12121)]
    #[traced_test]
    fn day_2_b_is_invalid_false(#[case] num: usize) {
        assert_eq!(false, solution::is_invalid_b(num))
    }
}
