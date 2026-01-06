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

    // todo: currently the output result is too high
    // 4_566_760_900 => too high
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
        tiles
            .iter()
            .tuple_combinations()
            .filter(|(a, b)| {
                // falling diagonal as in \
                let falling = a.x >= b.x && a.y >= b.y || b.x > a.x && b.y > a.y;
                if falling {
                    let (top_left, btm_right) = if a.x >= b.x { (b, a) } else { (a, b) };
                    tiles
                        .iter()
                        .any(|t| t.x <= top_left.x && t.y >= btm_right.y)
                        && tiles
                            .iter()
                            .any(|t| t.x >= btm_right.x && t.y <= top_left.y)
                        // no points inside the rectangle
                        && !tiles.iter().any(|t| {
                            t.x > top_left.x
                                && t.x < btm_right.x
                                && t.y > top_left.y
                                && t.y < btm_right.y
                        })
                }
                // raising diagonal as in /
                else {
                    let (top_right, btm_left) = if a.x >= b.x { (a, b) } else { (b, a) };
                    tiles
                        .iter()
                        .any(|t| t.x >= top_right.x && t.y >= btm_left.y)
                        && tiles
                            .iter()
                            .any(|t| t.x <= btm_left.x && t.y <= top_right.y)
                        // no points inside the rectangle
                        && !tiles.iter().any(|t| {
                            t.x > btm_left.x
                                && t.x < top_right.x
                                && t.y > top_right.y
                                && t.y < btm_left.y
                        })
                }
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
