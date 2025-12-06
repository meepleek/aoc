pub mod solution {
    use parse::range::parse_inclusive_range;
    use range::merge_all_bounded_ranges;

    #[tracing::instrument(skip(input))]
    pub fn part_a(input: &str) -> anyhow::Result<String> {
        let var_name = true;
        let mut parsing_ranges = var_name;
        let mut ranges = Vec::with_capacity(1000);
        // let mut ids = Vec::with_capacity(1000);
        let mut res = 0;
        for l in input.lines() {
            if l.is_empty() {
                parsing_ranges = false;
                continue;
            }

            if parsing_ranges {
                ranges.push(parse_inclusive_range(l)?);
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
        let mut ranges: Vec<_> = input
            .lines()
            .map_while(|l| {
                if l.is_empty() {
                    return None;
                }

                Some(parse_inclusive_range::<usize>(l).expect("valid range"))
            })
            .collect();
        ranges.sort_by_key(|r| *r.start());
        merge_all_bounded_ranges(&mut ranges);

        let total: usize = ranges.into_iter().map(|r| r.end() - r.start() + 1).sum();
        Ok(total.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tracing_test::traced_test;

    const VAR_NAME: &str = include_str!("../inputs/example.txt");
    const TEST_INPUT: &str = VAR_NAME;
    const EXPECTED_A: &str = "3";
    const EXPECTED_B: &str = "14";

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
