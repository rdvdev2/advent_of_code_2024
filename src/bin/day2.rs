use advent_of_code_2024::aoc_main;

use itertools::Itertools;

fn common(input: String) -> Vec<Vec<usize>> {
    input
        .lines()
        .map(|report| {
            report
                .split_whitespace()
                .map(|x| x.parse::<usize>().unwrap())
                .collect_vec()
        })
        .collect_vec()
}

fn part1(input: &Vec<Vec<usize>>) -> usize {
    input
        .iter()
        .map(|report| {
            let equal_sign = report
                .iter()
                .tuple_windows()
                .map(|(a, b)| a.cmp(b))
                .all_equal();

            if !equal_sign {
                return false;
            }

            let valid_difference = report
                .iter()
                .tuple_windows()
                .map(|(a, b)| a.abs_diff(*b))
                .all(|x| 1 <= x && x <= 3);

            return valid_difference;
        })
        .filter(|x| *x)
        .count()
}

fn part2(input: &Vec<Vec<usize>>) -> usize {
    input
        .iter()
        .map(|report| {
            std::iter::once(report.clone())
                .chain((0..report.len()).map(|idx| {
                    let mut new = report.clone();
                    new.remove(idx);
                    new
                }))
                .map(|report| part1(&vec![report]) > 0)
                .filter(|x| *x)
                .next()
                .is_some()
        })
        .filter(|x| *x)
        .count()
}

aoc_main!(2, common, part1, part2);
