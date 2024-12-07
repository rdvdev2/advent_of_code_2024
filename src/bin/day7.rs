use advent_of_code_2024::aoc_main;
use itertools::Itertools;

enum Operator {
    Add,
    Mul,
    Concat,
}

impl Operator {
    fn apply(&self, a: usize, b: usize) -> usize {
        match self {
            Self::Add => a + b,
            Self::Mul => a * b,
            Self::Concat => format!("{}{}", a.to_string(), b.to_string())
                .parse()
                .unwrap(),
        }
    }
}

struct Operation {
    result: usize,
    numbers: Vec<usize>,
}

impl Operation {
    fn can_be_solved_impl(
        result: usize,
        partial: usize,
        numbers: &[usize],
        operators: &[Operator],
    ) -> bool {
        if partial > result {
            false
        } else if numbers.is_empty() {
            partial == result
        } else {
            for operator in operators {
                if Self::can_be_solved_impl(
                    result,
                    operator.apply(partial, numbers[0]),
                    &numbers[1..],
                    operators,
                ) {
                    return true;
                }
            }
            return false;
        }
    }

    fn can_be_solved(&self, operators: &[Operator]) -> bool {
        Self::can_be_solved_impl(self.result, self.numbers[0], &self.numbers[1..], operators)
    }
}

fn common(input: String) -> Vec<Operation> {
    input
        .lines()
        .map(|line| line.split_once(':').unwrap())
        .map(|(result, numbers)| Operation {
            result: result.parse().unwrap(),
            numbers: numbers
                .split_whitespace()
                .map(|x| x.parse().unwrap())
                .collect_vec(),
        })
        .collect_vec()
}

fn part1(operations: &Vec<Operation>) -> usize {
    operations
        .iter()
        .filter(|op| op.can_be_solved(&[Operator::Add, Operator::Mul]))
        .map(|op| op.result)
        .sum()
}

fn part2(operations: &Vec<Operation>) -> usize {
    operations
        .iter()
        .filter(|op| op.can_be_solved(&[Operator::Add, Operator::Mul, Operator::Concat]))
        .map(|op| op.result)
        .sum()
}

aoc_main!(7, common, part1, part2);
