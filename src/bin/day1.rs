use advent_of_code_2024::aoc_main;
use itertools::{Either, Itertools};

pub fn common(input: String) -> (Vec<usize>, Vec<usize>) {
    input
        .lines()
        .flat_map(|x| {
            x.split_whitespace()
                .map(|x| x.parse::<usize>().unwrap())
                .zip(0..)
        })
        .partition_map(|(value, list)| {
            if list == 0 {
                Either::Left(value)
            } else {
                Either::Right(value)
            }
        })
}

pub fn part1((left, right): &(Vec<usize>, Vec<usize>)) -> usize {
    left.iter()
        .sorted_unstable()
        .zip(right.iter().sorted_unstable())
        .map(|(x, y)| x.abs_diff(*y))
        .sum()
}

pub fn part2((left, right): &(Vec<usize>, Vec<usize>)) -> usize {
    let histogram = right.iter().counts();
    left.iter()
        .map(|x| x * histogram.get(x).unwrap_or(&0))
        .sum()
}

aoc_main!(1, common, part1, part2);
