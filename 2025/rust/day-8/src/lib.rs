pub mod solution {
    use glam::Vec3;
    use itertools::Itertools;
    use std::collections::HashSet;

    #[derive(Debug, Clone, PartialEq)]
    struct DistIndex {
        dist_sq: f32,
        index_a: usize,
        index_b: usize,
    }

    impl DistIndex {
        fn process_circuits(&self, circuits: &mut Vec<HashSet<usize>>) {
            match (
                circuits
                    .iter()
                    .position(|circ| circ.contains(&self.index_a)),
                circuits
                    .iter()
                    .position(|circ| circ.contains(&self.index_b)),
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
                    circuits[circ_a].insert(self.index_a);
                    circuits[circ_a].insert(self.index_b);
                }
                (None, Some(circ_b)) => {
                    circuits[circ_b].insert(self.index_a);
                    circuits[circ_b].insert(self.index_b);
                }
                (None, None) => {
                    circuits.push(HashSet::from([self.index_a, self.index_b]));
                }
            }
        }
    }

    #[tracing::instrument(skip(input))]
    pub fn part_a(input: &str) -> anyhow::Result<String> {
        part_a_circuit_size(input, 1000)
    }

    #[tracing::instrument(skip(input))]
    pub(super) fn part_a_circuit_size(
        input: &str,
        connection_count: usize,
    ) -> anyhow::Result<String> {
        let coords = parse_coords(input);
        let mut distances = distances(&coords);
        let mut circuits: Vec<HashSet<usize>> = Vec::with_capacity(connection_count);
        for _ in 0..connection_count {
            let nearest = distances.pop().expect("Nearest is available");
            nearest.process_circuits(&mut circuits);
        }

        circuits.sort_unstable_by_key(|c| c.len());
        let res: usize = circuits.iter().rev().take(3).map(|c| c.len()).product();
        Ok(res.to_string())
    }

    #[tracing::instrument(skip(input))]
    pub fn part_b(input: &str) -> anyhow::Result<String> {
        let coords = parse_coords(input);
        let mut distances = distances(&coords);
        let mut circuits: Vec<HashSet<usize>> = Vec::with_capacity(1000);
        loop {
            let nearest = distances.pop().expect("Nearest is available");
            nearest.process_circuits(&mut circuits);
            if circuits.len() == 1 && circuits[0].len() == coords.len() {
                let a_x = coords[nearest.index_a][0];
                let b_x = coords[nearest.index_b][0];
                return Ok((a_x * b_x).to_string());
            }
        }
    }

    fn parse_coords(input: &str) -> Vec<Vec3> {
        input
            .lines()
            .map(|l| {
                let (x, rest) = l.split_once(',').unwrap();
                let (y, z) = rest.split_once(',').unwrap();
                Vec3::new(x.parse().unwrap(), y.parse().unwrap(), z.parse().unwrap())
            })
            .collect()
    }

    fn distances(coords: &[Vec3]) -> Vec<DistIndex> {
        let mut distances: Vec<_> = coords
            .iter()
            .enumerate()
            .tuple_combinations()
            .map(|((a_i, a), (b_i, b))| DistIndex {
                dist_sq: a.distance_squared(*b),
                index_a: a_i,
                index_b: b_i,
            })
            // .filter(|d| d.index_a != d.index_b && d.dist_sq > 0.)
            .collect();
        distances.sort_by(|a, b| b.dist_sq.partial_cmp(&a.dist_sq).unwrap());
        distances
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tracing_test::traced_test;

    const TEST_INPUT: &str = include_str!("../inputs/example.txt");
    const EXPECTED_A: &str = "40";
    const EXPECTED_B: &str = "25272";

    #[test]
    #[traced_test]
    fn day_8_a() {
        let res = solution::part_a_circuit_size(TEST_INPUT, 10);
        assert_eq!(EXPECTED_A, res.unwrap());
    }

    #[test]
    #[traced_test]
    fn day_8_b() {
        let res = solution::part_b(TEST_INPUT);
        assert_eq!(EXPECTED_B, res.unwrap());
    }
}
