pub mod solution {
    use std::collections::HashSet;

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
        let init_grid = GridBuilder::<()>::build_obstacle_grid()
            .input(input)
            .obstacle('@')
            .call()?
            .grid;
        let size = init_grid.size();
        let mut obstacles: HashSet<_> = init_grid.walkable_tiles().keys().cloned().collect();
        let mut removed = 0;

        loop {
            let grid = Grid::<()>::from_obstacles(obstacles.clone(), size);
            // grid.print_debug_map(|_| Some('.'));
            let to_remove: Vec<_> = grid
                .walkable_tiles()
                .keys()
                .filter(|t| grid.neighbours_8(**t).len() < 4)
                .collect();

            if to_remove.len() == 0 {
                break;
            }

            removed += to_remove.len();
            obstacles.extend(to_remove);
        }

        Ok(removed.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tracing_test::traced_test;

    const TEST_INPUT: &str = include_str!("../inputs/example.txt");
    const EXPECTED_A: &str = "13";
    const EXPECTED_B: &str = "43";

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
