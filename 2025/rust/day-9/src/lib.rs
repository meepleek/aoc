pub mod solution {
    use glam::U64Vec2;
    use itertools::Itertools;
    use parse::vec::parse_u64vec2_res;

    #[tracing::instrument(skip(input))]
    pub fn part_a(input: &str) -> anyhow::Result<String> {
        let tiles = input
            .lines()
            .map(parse_u64vec2_res)
            .collect::<Result<Vec<_>, _>>()?;
        let res = tiles
            .iter()
            .tuple_combinations()
            .map(|(a, b)| ((a.max(*b) - a.min(*b)) + U64Vec2::ONE).element_product())
            .max()
            .unwrap();
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
    const EXPECTED_A: &str = "50";
    const EXPECTED_B: &str = "24";

    #[test]
    #[traced_test]
    fn day_9_a() {
        let res = solution::part_a(TEST_INPUT);
        assert_eq!(EXPECTED_A, res.unwrap());
    }

    #[test]
    #[traced_test]
    fn day_9_b() {
        let res = solution::part_b(TEST_INPUT);
        assert_eq!(EXPECTED_B, res.unwrap());
    }
}
