pub mod solution {
    #[tracing::instrument(skip(input))]
    pub fn part_a(input: &str) -> anyhow::Result<String> {
        let mut parsing_ranges = true;
        let mut ranges = Vec::with_capacity(1000);
        // let mut ids = Vec::with_capacity(1000);
        let mut res = 0;
        for l in input.lines() {
            if l.is_empty() {
                parsing_ranges = false;
                continue;
            }

            if parsing_ranges {
                let (from, to) = l.split_once('-').expect("valid range");
                ranges.push(
                    from.parse::<usize>().expect("valid from")..=to.parse().expect("valid to"),
                );
            } else {
                let num = l.parse::<usize>().expect("is num");
                // ids.push();
                if ranges.iter().any(|r| r.contains(&num)) {
                    res += 1;
                }
            }
        }

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
    const EXPECTED_A: &str = "3";
    const EXPECTED_B: &str = "todo_expected_b";

    #[test]
    #[traced_test]
    fn day_5_a() {
        let res = solution::part_a(TEST_INPUT);
        assert_eq!(EXPECTED_A, res.unwrap());
    }

    #[test]
    #[traced_test]
    fn day_5_b() {
        let res = solution::part_b(TEST_INPUT);
        assert_eq!(EXPECTED_B, res.unwrap());
    }
}
