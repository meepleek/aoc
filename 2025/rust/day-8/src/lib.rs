pub mod solution {
    use std::collections::HashSet;

    use glam::{IVec3, Vec3};

    #[tracing::instrument(skip(input))]
    pub fn part_a(input: &str) -> anyhow::Result<String> {
        part_a_circuit_size(input, 999)
    }

    #[tracing::instrument(skip(input))]
    pub(super) fn part_a_circuit_size(
        input: &str,
        connection_count: usize,
    ) -> anyhow::Result<String> {
        let coords: Vec<_> = input
            .lines()
            .map(|l| {
                let (x, rest) = l.split_once(',').unwrap();
                let (y, z) = rest.split_once(',').unwrap();
                IVec3::new(x.parse().unwrap(), y.parse().unwrap(), z.parse().unwrap())
            })
            .collect();

        let mut circuits: Vec<HashSet<IVec3>> = Vec::with_capacity(1_000);
        for i in 0..connection_count {
            eprintln!("min {i}");
            let mut min_dist = f32::MAX;
            let mut min_coords = None;
            for a in &coords {
                for b in &coords {
                    if a == b || circuits.iter().any(|c| c.contains(a) && c.contains(b)) {
                        continue;
                    }

                    let dist = a.as_vec3().distance(b.as_vec3());
                    if dist < min_dist {
                        min_dist = dist;
                        min_coords = Some((*a, *b));
                    }
                }
            }
            eprintln!("circuit {i}");
            let (a, b) = min_coords.unwrap();
            match (
                circuits.iter().position(|circ| circ.contains(&a)),
                circuits.iter().position(|circ| circ.contains(&b)),
            ) {
                (Some(circ_a), Some(circ_b)) => {
                    if circ_a != circ_b {
                        let min = circ_a.min(circ_b);
                        let max = circ_a.max(circ_b);
                        let to_merge = circuits.remove(max);
                        circuits[min].extend(to_merge);
                    }
                }
                (Some(circ_a), None) => {
                    circuits[circ_a].insert(b);
                }
                (None, Some(circ_b)) => {
                    circuits[circ_b].insert(a);
                }
                (None, None) => {
                    circuits.push(HashSet::from([a, b]));
                }
            }
        }

        circuits.sort_unstable_by_key(|c| c.len());
        let c: Vec<_> = circuits.iter().rev().map(|c| c.len()).collect();
        tracing::warn!(?circuits, ?c);
        let res: usize = circuits.iter().rev().take(3).map(|c| c.len()).product();

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
    const EXPECTED_A: &str = "40";
    const EXPECTED_B: &str = "todo_expected_b";

    #[test]
    #[traced_test]
    fn day_8_a() {
        let res = solution::part_a_circuit_size(TEST_INPUT, 9);
        assert_eq!(EXPECTED_A, res.unwrap());
    }

    #[test]
    #[traced_test]
    fn day_8_b() {
        let res = solution::part_b(TEST_INPUT);
        assert_eq!(EXPECTED_B, res.unwrap());
    }
}
