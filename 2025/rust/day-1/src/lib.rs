pub mod solution {
    #[tracing::instrument(skip(input))]
    pub fn part_a(input: &str) -> anyhow::Result<String> {
        let nums: Vec<isize> = input
            .lines()
            .map(|l| {
                let (dir, num) = l.split_at(1);
                let sign = if dir == "L" { -1 } else { 1 };
                let num: isize = num.parse().expect("Valid num");
                num * sign
            })
            .collect();
        let mut dial = 50;
        let mut res = 0;
        for num in nums {
            dial = (dial + num).rem_euclid(100);
            if dial == 0 {
                res += 1;
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
    const EXPECTED_B: &str = "6";

    #[test]
    #[traced_test]
    fn day_1_a() {
        let res = solution::part_a(TEST_INPUT);
        assert_eq!(EXPECTED_A, res.unwrap());
    }

    #[test]
    #[traced_test]
    fn day_1_b() {
        let res = solution::part_b(TEST_INPUT);
        assert_eq!(EXPECTED_B, res.unwrap());
    }
}
