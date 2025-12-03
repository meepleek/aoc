pub mod solution {
    #[tracing::instrument(skip(input))]
    pub fn part_a(input: &str) -> anyhow::Result<String> {
        let res: usize = input
            .split(",")
            .map(|range| {
                let (from, to) = range.trim().split_once("-").expect("valid range");
                let from: usize = from.parse().expect("valid from");
                let to: usize = to.parse().expect("valid to");
                (from..=to)
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
        todo!("b")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tracing_test::traced_test;

    const TEST_INPUT: &str = include_str!("../inputs/example.txt");
    const EXPECTED_A: &str = "1227775554";
    const EXPECTED_B: &str = "todo_expected_b";

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
}
