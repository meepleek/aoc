pub mod solution {
    use std::ops::RangeInclusive;

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
        let mut ranges: Vec<_> = input
            .lines()
            .map_while(|l| {
                if l.is_empty() {
                    return None;
                }

                let (from, to) = l.split_once('-').expect("valid range");
                Some(from.parse::<usize>().expect("valid from")..=to.parse().expect("valid to"))
            })
            .collect();
        ranges.sort_by_key(|r| *r.start());

        // iterate over all ranges in an endless loop
        // merge overlapping ranges
        // break once no merge has happened for the whole iteration
        'merge_ranges: loop {
            let ranges_range = 0..ranges.len();
            for i in ranges_range.clone() {
                for j in ranges_range.clone() {
                    if i == j {
                        continue;
                    }
                    if let Some(merged_range) = merge_ranges(&ranges[i], &ranges[j]) {
                        // remove from end
                        ranges.remove(i.max(j));
                        ranges.remove(j.min(i));
                        ranges.push(merged_range);
                        continue 'merge_ranges;
                    }
                }
            }

            break;
        }

        let total: usize = ranges.into_iter().map(|r| r.end() - r.start() + 1).sum();
        Ok(total.to_string())
    }

    pub(super) fn merge_ranges(
        a: &RangeInclusive<usize>,
        b: &RangeInclusive<usize>,
    ) -> Option<RangeInclusive<usize>> {
        match true {
            _ if a.start() <= b.start() && a.end() >= b.end() => Some(a.clone()),
            _ if b.start() <= a.start() && b.end() >= a.end() => Some(b.clone()),
            _ if a.end() >= b.start() && a.end() <= b.end() => Some(*a.start()..=*b.end()),
            _ if b.end() >= a.start() && b.end() <= a.end() => Some(*b.start()..=*a.end()),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;
    use std::ops::RangeInclusive;
    use tracing_test::traced_test;

    const TEST_INPUT: &str = include_str!("../inputs/example.txt");
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

    #[rstest]
    #[case(0..=1, 2..=3, None)]
    #[case(0..=1, 1..=3, Some(0..=3))]
    #[case(0..=5, 1..=10, Some(0..=10))]
    #[case(0..=1, 0..=3, Some(0..=3))]
    #[case(0..=100, 10..=11, Some(0..=100))]
    #[traced_test]
    fn merge_ranges(
        #[case] a: RangeInclusive<usize>,
        #[case] b: RangeInclusive<usize>,
        #[case] expected_result: Option<RangeInclusive<usize>>,
    ) {
        let merged = solution::merge_ranges(&a, &b);
        let merged_reverse = solution::merge_ranges(&b, &a);

        assert_eq!(expected_result, merged);
        assert_eq!(expected_result, merged_reverse);
    }
}
