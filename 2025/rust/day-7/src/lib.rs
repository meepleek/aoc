pub mod solution {
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

    #[tracing::instrument(skip(input))]
    pub fn part_b(input: &str) -> anyhow::Result<String> {
        let lines: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
        let start_line = &lines[0];
        let start_x = start_line
            .iter()
            .position(|c| *c == 'S')
            .context("start tile")?;

        let split_counts = lines.iter().enumerate().skip(2).fold(
            HashMap::from([(start_x, 1)]),
            |counts, (_, l)| {
                let mut new_counts = HashMap::new();
                for (x, curr_count) in counts {
                    match l[x] {
                        '^' => {
                            new_counts
                                .entry(x - 1)
                                .and_modify(|c| *c += curr_count)
                                .or_insert(curr_count);
                            new_counts
                                .entry(x + 1)
                                .and_modify(|c| *c += curr_count)
                                .or_insert(curr_count);
                        }
                        _ => {
                            new_counts
                                .entry(x)
                                .and_modify(|c| *c += curr_count)
                                .or_insert(curr_count);
                        }
                    }
                }

                new_counts
            },
        );

        let count: usize = split_counts.values().sum();
        Ok((count).to_string())
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
