pub mod solution {
    pub use grid::prelude::*;

    #[tracing::instrument(skip(input))]
    pub fn part_a(input: &str) -> anyhow::Result<String> {
        let grid = GridBuilder::<()>::build_obstacle_grid()
            .input(input)
            .obstacle('.')
            .call()?
            .grid;
        let count = grid
            .walkable_tiles()
            .keys()
            .filter(|t| grid.neighbours_8(**t).len() < 4)
            .count();
        Ok(count.to_string())
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
    const EXPECTED_A: &str = "13";
    const EXPECTED_B: &str = "todo_expected_b";

    #[test]
    #[traced_test]
    fn day_4_a() {
        let res = solution::part_a(TEST_INPUT);
        assert_eq!(EXPECTED_A, res.unwrap());
    }

    #[test]
    #[traced_test]
    fn day_4_b() {
        let res = solution::part_b(TEST_INPUT);
        assert_eq!(EXPECTED_B, res.unwrap());
    }
}
