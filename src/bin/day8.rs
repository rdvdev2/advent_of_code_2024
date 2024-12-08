use std::{
    collections::{BTreeSet, HashSet},
    iter,
    ops::{Add, Sub},
};

use advent_of_code_2024::aoc_main;
use itertools::Itertools;

#[derive(Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash)]
struct Vec2(isize, isize);

impl Vec2 {
    fn to(&self, other: Self) -> Self {
        Self(other.0 - self.0, other.1 - self.1)
    }
}

impl Add<Vec2> for Vec2 {
    type Output = Self;

    fn add(self, rhs: Vec2) -> Self::Output {
        Self(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl Sub<Vec2> for Vec2 {
    type Output = Self;

    fn sub(self, rhs: Vec2) -> Self::Output {
        Self(self.0 - rhs.0, self.1 - rhs.1)
    }
}

fn char_to_antenna_index(c: char) -> usize {
    (c as u8).into()
}

fn common(input: String) -> (Vec<(Vec2, char)>, (isize, isize)) {
    let antennas = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .filter(|(_, c)| *c != '.')
                .map(move |(x, c)| (Vec2(x as isize, y as isize), c))
        })
        .collect_vec();

    let x_bound = input.lines().next().unwrap().len() as isize;
    let y_bound = input.lines().count() as isize;

    (antennas, (x_bound, y_bound))
}

fn part1((antennas, (x_bound, y_bound)): &(Vec<(Vec2, char)>, (isize, isize))) -> usize {
    let mut seen_antennas = vec![BTreeSet::new(); 123];
    let mut antinodes = HashSet::new();

    for (pos, c) in antennas {
        for other_antenna in seen_antennas[char_to_antenna_index(*c)].iter() {
            let to = pos.to(*other_antenna);
            antinodes.insert(*other_antenna + to);
            antinodes.insert(*pos - to);
        }

        seen_antennas[char_to_antenna_index(*c)].insert(*pos);
    }

    antinodes
        .iter()
        .filter(|Vec2(x, y)| 0 <= *x && x < x_bound && 0 <= *y && y < y_bound)
        .count()
}

fn part2((antennas, (x_bound, y_bound)): &(Vec<(Vec2, char)>, (isize, isize))) -> usize {
    let mut seen_antennas = vec![BTreeSet::new(); 123];
    let mut antinodes = HashSet::new();

    for (pos, c) in antennas {
        for other_antenna in seen_antennas[char_to_antenna_index(*c)].iter() {
            let to = pos.to(*other_antenna);

            antinodes.extend(
                iter::successors(Some(*other_antenna), |x| Some(*x + to))
                    .take_while(|Vec2(x, y)| 0 <= *x && x < x_bound && 0 <= *y && y < y_bound),
            );
            antinodes.extend(
                iter::successors(Some(*pos), |x| Some(*x - to))
                    .take_while(|Vec2(x, y)| 0 <= *x && x < x_bound && 0 <= *y && y < y_bound),
            );
        }

        seen_antennas[char_to_antenna_index(*c)].insert(*pos);
    }

    antinodes.len()
}

aoc_main!(8, common, part1, part2);
