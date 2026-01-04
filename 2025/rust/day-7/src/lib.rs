pub mod solution {
    use glam::UVec2;
    use std::collections::{HashMap, HashSet};

    use anyhow::Context;

    #[tracing::instrument(skip(input))]
    pub fn part_a(input: &str) -> anyhow::Result<String> {
        let lines: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
        let start_line = &lines[0];
        let width = start_line.len();
        let start_x = start_line
            .iter()
            .position(|c| *c == 'S')
            .context("start tile")?;
        let mut beams = HashSet::with_capacity(width);
        beams.insert(start_x);
        let mut line_beams = Vec::with_capacity(width);
        let mut split_count = 0;
        for l in lines.iter().skip(1) {
            for x in &beams {
                if l[*x] == '^' {
                    split_count += 1;
                    line_beams.push(x - 1);
                    line_beams.push(x + 1);
                } else {
                    line_beams.push(*x);
                }
            }
            beams.clear();
            beams.extend(line_beams.drain(..));
        }
        Ok(split_count.to_string())
    }

    #[derive(Debug, PartialEq, Eq, Clone)]
    struct BeamNode {
        pos: UVec2,
        neighbours: Vec<UVec2>,
    }
    impl BeamNode {
        pub fn new(x: usize, y: usize) -> Self {
            Self {
                pos: (x as u32, y as u32).into(),
                neighbours: Vec::with_capacity(2),
            }
        }

        pub fn is_leaf(&self) -> bool {
            self.neighbours.is_empty()
        }

        pub fn dfs(&self, nodes: &HashMap<UVec2, BeamNode>) -> HashSet<Vec<UVec2>> {
            let mut paths = HashSet::new();

            fn walk(
                node: &BeamNode,
                nodes: &HashMap<UVec2, BeamNode>,
                paths: &mut HashSet<Vec<UVec2>>,
                path: Vec<UVec2>,
            ) {
                if node.is_leaf() {
                    paths.insert(path);
                } else {
                    for neighbour_pos in &node.neighbours {
                        let mut path = path.to_vec();
                        path.push(*neighbour_pos);
                        let neighbour = nodes.get(neighbour_pos).expect("Missing neigbour");
                        walk(neighbour, nodes, paths, path);
                    }
                }
            }

            walk(self, nodes, &mut paths, Vec::new());
            paths
        }
    }

    #[tracing::instrument(skip(input))]
    pub fn part_b(input: &str) -> anyhow::Result<String> {
        let lines: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
        let start_line = &lines[0];
        let width = start_line.len();
        let start_x = start_line
            .iter()
            .position(|c| *c == 'S')
            .context("start tile")?;
        let start_beam = BeamNode::new(start_x, 0);
        let start_pos = start_beam.pos;
        let mut beams = HashSet::with_capacity(width);
        beams.insert(start_beam.pos);
        let mut nodes = HashMap::with_capacity(width / 2 * lines.len());
        nodes.insert(start_beam.pos, start_beam);
        for (y, l) in lines.iter().enumerate().skip(1) {
            let mut line_beams = Vec::with_capacity(width);

            for beam in &beams {
                let x = beam.x as usize;
                if l[x] == '^' {
                    let left = BeamNode::new(x - 1, y);
                    let right = BeamNode::new(x + 1, y);
                    let beam = nodes.get_mut(&beam).expect("Beam exists");
                    beam.neighbours.push(left.pos);
                    beam.neighbours.push(right.pos);
                    line_beams.push(left.pos);
                    nodes.insert(left.pos, left);
                    line_beams.push(right.pos);
                    nodes.insert(right.pos, right);
                } else {
                    line_beams.push(*beam);
                }
            }
            beams.clear();
            beams.extend(line_beams.drain(..));
        }

        tracing::warn!(?nodes);

        let paths = nodes.get(&start_pos).expect("Start node set").dfs(&nodes);
        Ok((paths.len()).to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tracing_test::traced_test;

    const TEST_INPUT: &str = include_str!("../inputs/example.txt");
    const EXPECTED_A: &str = "21";
    const EXPECTED_B: &str = "40";

    #[test]
    #[traced_test]
    fn day_7_a() {
        let res = solution::part_a(TEST_INPUT);
        assert_eq!(EXPECTED_A, res.unwrap());
    }

    #[test]
    #[traced_test]
    fn day_7_b() {
        let res = solution::part_b(TEST_INPUT);
        assert_eq!(EXPECTED_B, res.unwrap());
    }
}
