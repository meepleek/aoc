pub mod solution {
    use anyhow::Context;

    #[derive(Debug)]
    enum ColumnOp {
        Add,
        Mul,
    }

    #[tracing::instrument(skip(input))]
    pub fn part_a(input: &str) -> anyhow::Result<String> {
        let ops: Vec<_> = input
            .lines()
            .rev()
            .next()
            .context("Ops line")?
            .split_whitespace()
            .map(|op| match op {
                "+" => ColumnOp::Add,
                "*" => ColumnOp::Mul,
                _ => panic!("Unknown op {op}"),
            })
            .collect();
        let mut col_results: Vec<_> = ops
            .iter()
            .map(|op| match op {
                ColumnOp::Add => 0,
                ColumnOp::Mul => 1,
            })
            .collect();
        for l in input.lines().rev().skip(1) {
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
    const EXPECTED_A: &str = "4277556";
    const EXPECTED_B: &str = "todo_expected_b";

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
