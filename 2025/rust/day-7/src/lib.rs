pub mod solution {
    use std::collections::HashSet;

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
                    if *x > 0 {
                        line_beams.push(x - 1);
                    }
                    if *x < (width - 1) {
                        line_beams.push(x + 1);
                    }
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
        todo!("b")
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
