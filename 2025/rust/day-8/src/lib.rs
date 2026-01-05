pub mod solution {
    use kdtree::{KdTree, distance::squared_euclidean};
    use std::collections::{BinaryHeap, HashSet};

    #[derive(Debug, Clone, PartialEq)]
    struct DistIndex {
        dist_sq: f32,
        index_a: usize,
        index_b: usize,
    }
    impl PartialOrd for DistIndex {
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
            other.dist_sq.partial_cmp(&self.dist_sq)
        }
    }
    impl Ord for DistIndex {
        fn cmp(&self, other: &Self) -> std::cmp::Ordering {
            self.partial_cmp(other).unwrap()
        }
    }
    impl Eq for DistIndex {}

    #[tracing::instrument(skip(input))]
    pub fn part_a(input: &str) -> anyhow::Result<String> {
        part_a_circuit_size(input, 1000)
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
                [
                    x.parse::<f32>().unwrap(),
                    y.parse().unwrap(),
                    z.parse().unwrap(),
                ]
            })
            .collect();

        let mut kdtree = KdTree::new(3);
        for (i, c) in coords.iter().enumerate() {
            kdtree.add(c, i).unwrap();
        }
        let mut distances = BinaryHeap::new();

        for (a_i, a) in coords.iter().enumerate() {
            let nearest = kdtree
                .iter_nearest(a, &squared_euclidean)
                .unwrap()
                .skip(1) // skip connection to self (0 dist)
                .take((connection_count / 2).max(10))
                .map(|(dist_sq, b_i)| DistIndex {
                    dist_sq,
                    index_a: a_i,
                    index_b: *b_i,
                });
            distances.extend(nearest);
        }

        let mut circuits: Vec<HashSet<usize>> = Vec::with_capacity(connection_count);
        for _ in 0..connection_count {
            let nearest = distances.pop().expect("Nearest is available");
            _ = distances.pop(); // nearest pairs come in... um, pairs, so discard the dupe
            match (
                circuits
                    .iter()
                    .position(|circ| circ.contains(&nearest.index_a)),
                circuits
                    .iter()
                    .position(|circ| circ.contains(&nearest.index_b)),
            ) {
                (Some(circ_a), Some(circ_b)) => {
                    if circ_a != circ_b {
                        let min = circ_a.min(circ_b);
                        let max = circ_a.max(circ_b);
                        let to_merge = circuits.remove(max);
                        circuits[min].extend(to_merge);
                    } else {
                        // add_connection = false;
                    }
                }
                (Some(circ_a), None) => {
                    circuits[circ_a].insert(nearest.index_a);
                    circuits[circ_a].insert(nearest.index_b);
                }
                (None, Some(circ_b)) => {
                    circuits[circ_b].insert(nearest.index_a);
                    circuits[circ_b].insert(nearest.index_b);
                }
                (None, None) => {
                    circuits.push(HashSet::from([nearest.index_a, nearest.index_b]));
                }
            }
        }

        circuits.sort_unstable_by_key(|c| c.len());
        let res: usize = circuits.iter().rev().take(3).map(|c| c.len()).product();
        // let circuit_coords = circuits
        //     .iter()
        //     .rev()
        //     .take(3)
        //     .map(|c| c.iter().map(|i| coords[*i]).collect::<Vec<_>>())
        //     .collect::<Vec<_>>();
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
