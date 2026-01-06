pub mod solution {
    use glam::U64Vec2;
    use itertools::Itertools;
    use parse::vec::parse_u64vec2_res;

    #[tracing::instrument(skip(input))]
    pub fn part_a(input: &str) -> anyhow::Result<String> {
        let collect = input
            .lines()
            .map(parse_u64vec2_res)
            .collect::<Result<Vec<_>, _>>()?;
        let tiles = collect;
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
        let tiles = input
            .lines()
            .map(parse_u64vec2_res)
            .collect::<Result<Vec<_>, _>>()?;
        let res = max_area_b(&tiles);
        Ok(res.to_string())
    }

    pub(super) fn max_area_b(tiles: &[U64Vec2]) -> u64 {
        let lines: Vec<(_, _)> = tiles.iter().circular_tuple_windows().collect();
        tiles
            .iter()
            .tuple_combinations()
            .filter(|(a, b)| {
                let top_left = a.min(**b);
                let btm_right = a.max(**b);
                // no lines crossing the rectangle
                !lines.iter().any(|(a, b)| {
                    let horizontal = a.y == b.y;
                    if horizontal {
                        a.y > top_left.y
                            && a.y < btm_right.y
                            && (a.x > top_left.x && a.x < btm_right.x
                                || b.x > top_left.x && b.x < btm_right.x
                                || a.x <= top_left.x && b.x >= btm_right.x
                                || b.x <= top_left.x && a.x >= btm_right.x)
                    } else {
                        a.x > top_left.x
                            && a.x < btm_right.x
                            && (a.y > top_left.y && a.y < btm_right.y
                                || b.y > top_left.y && b.y < btm_right.y
                                || a.y <= top_left.y && b.y >= btm_right.y
                                || b.y <= top_left.y && a.y >= btm_right.y)
                    }
                })
            })
            .map(|(a, b)| (a.max(*b) - a.min(*b) + U64Vec2::ONE).element_product())
            .max()
            .unwrap()
    }
}

#[cfg(test)]
mod tests {
    use crate::solution::max_area_b;

    use super::*;
    use glam::U64Vec2;
    use test_case::test_case;
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

    #[test_case(vec![(0, 0), (1, 0), (1, 1), (0, 1)] => 4; "unit")]
    #[test_case(vec![(0, 0), (2, 0), (2, 2), (0, 2)] => 9; "2x2 square")]
    #[test_case(vec![(0, 0), (2, 0), (2, 1), (0, 1)] => 6; "2x1 rect")]
    #[test_case(vec![(0, 0), (2, 0), (2, 1), (1, 1), (1, 2), (0, 2)] => 6; "2-long L")]
    #[test_case(vec![(0, 0), (3, 0), (3, 1), (1, 1), (1, 2), (0, 2)] => 8; "3-long L")]
    #[test_case(vec![(0, 0), (3, 0), (3, 2), (2, 2), (2, 1), (1, 1), (1, 2), (0, 2)] => 6; "bridge")]
    #[test_case(vec![(7, 1), (11, 1), (11, 7), (9, 7), (9, 5), (2, 5), (2, 3), (7, 3)] => 24; "test input")]
    #[traced_test]
    fn day_area(tiles: Vec<(u64, u64)>) -> u64 {
        let tiles = tiles
            .into_iter()
            .map(|(x, y)| U64Vec2::new(x, y))
            .collect::<Vec<_>>();
        max_area_b(&tiles)
    }
}
