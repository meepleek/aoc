pub mod solution {
    use anyhow::Context;

    #[derive(Debug)]
    enum ColumnOp {
        Add,
        Mul,
    }

    #[tracing::instrument(skip(input))]
    pub fn part_a(input: &str) -> anyhow::Result<String> {
        let lines: Vec<_> = input.lines().collect();
        let ops = parse_ops_a(lines.last().context("ops line exists")?)?;
        let mut col_results: Vec<_> = ops
            .iter()
            .map(|op| match op {
                ColumnOp::Add => 0,
                ColumnOp::Mul => 1,
            })
            .collect();
        for l in lines.into_iter().rev().skip(1) {
            for (i, num) in l.split_whitespace().enumerate() {
                let num: usize = num.parse()?;
                let current = col_results[i];
                col_results[i] = match ops[i] {
                    ColumnOp::Add => current + num,
                    ColumnOp::Mul => current * num,
                };
            }
        }
        let total: usize = col_results.into_iter().sum();
        Ok(total.to_string())
    }

    fn parse_ops_a(line: &str) -> anyhow::Result<Vec<ColumnOp>> {
        Ok(line
            .split_whitespace()
            .map(|op| match op {
                "+" => ColumnOp::Add,
                "*" => ColumnOp::Mul,
                _ => panic!("Unknown op {op}"),
            })
            .collect())
    }

    #[tracing::instrument(skip(input))]
    pub fn part_b(input: &str) -> anyhow::Result<String> {
        let lines: Vec<_> = input.lines().collect();
        let ops_line = lines.last().context("ops line exists")?;
        let width = ops_line.len();
        let ops = parse_ops_b(&ops_line);
        let chars: Vec<Vec<_>> = lines
            .iter()
            .rev()
            .skip(1)
            .map(|l| l.chars().collect())
            .collect();
        let mut res = 0;
        for (i, (x_from, op)) in ops.iter().enumerate() {
            let mut col_sum: usize = match op {
                ColumnOp::Add => 0,
                ColumnOp::Mul => 1,
            };
            let x_to = ops.get(i + 1).map_or_else(|| width, |(i, _)| i - 1);
            for x in *x_from..x_to {
                let mut mult = 1;
                let mut num = 0;
                for line in &chars {
                    let c = line[x];
                    match c.to_digit(10) {
                        Some(n) => {
                            num += n as usize * mult;
                            mult *= 10;
                        }
                        None if mult > 1 => break,
                        _ => {}
                    }
                }
                match op {
                    ColumnOp::Add => col_sum += num,
                    ColumnOp::Mul => col_sum *= num,
                };
            }
            res += col_sum;
        }
        Ok(res.to_string())
    }

    fn parse_ops_b(line: &str) -> Vec<(usize, ColumnOp)> {
        line.chars()
            .enumerate()
            .filter_map(|(i, c)| match c {
                '+' => Some((i, ColumnOp::Add)),
                '*' => Some((i, ColumnOp::Mul)),
                _ => None,
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tracing_test::traced_test;

    const TEST_INPUT: &str = include_str!("../inputs/example.txt");
    const EXPECTED_A: &str = "4277556";
    const EXPECTED_B: &str = "3263827";

    #[test]
    #[traced_test]
    fn day_6_a() {
        let res = solution::part_a(TEST_INPUT);
        assert_eq!(EXPECTED_A, res.unwrap());
    }

    #[test]
    #[traced_test]
    fn day_6_b() {
        let res = solution::part_b(TEST_INPUT);
        assert_eq!(EXPECTED_B, res.unwrap());
    }
}
