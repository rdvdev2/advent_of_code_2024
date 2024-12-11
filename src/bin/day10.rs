use std::collections::{HashSet, VecDeque};

use advent_of_code_2024::aoc_main;
use itertools::Itertools;

#[derive(Clone, Debug)]
struct Map(Vec<Vec<usize>>);

impl Map {
    fn try_get_height(&self, x: isize, y: isize) -> Option<usize> {
        if x < 0 || y < 0 {
            None
        } else {
            self.0
                .get(x as usize)
                .map(|v| v.get(y as usize))
                .flatten()
                .copied()
        }
    }

    fn score_tile(&mut self, x: isize, y: isize, ignore_checked: bool) -> usize {
        let mut to_check = VecDeque::new();
        let mut checked = HashSet::new();
        let mut score = 0;
        to_check.push_back((self.0[x as usize][y as usize], (x, y)));

        while let Some((height, (x, y))) = to_check.pop_front() {
            if !ignore_checked && checked.contains(&(x, y)) {
                continue;
            }

            if height == 9 {
                score += 1;
            } else {
                if Some(height + 1) == self.try_get_height(x - 1, y) {
                    to_check.push_back((height + 1, (x - 1, y)));
                }
                if Some(height + 1) == self.try_get_height(x + 1, y) {
                    to_check.push_back((height + 1, (x + 1, y)));
                }
                if Some(height + 1) == self.try_get_height(x, y - 1) {
                    to_check.push_back((height + 1, (x, y - 1)));
                }
                if Some(height + 1) == self.try_get_height(x, y + 1) {
                    to_check.push_back((height + 1, (x, y + 1)));
                }
            }

            checked.insert((x, y));
        }

        score
    }

    fn bounds(&self) -> (usize, usize) {
        (self.0.len(), self.0[0].len())
    }
}

fn common(input: String) -> Map {
    Map(input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as usize)
                .collect_vec()
        })
        .collect_vec())
}

fn part1(map: &Map) -> usize {
    let mut map = map.clone();
    let (x_bound, y_bound) = map.bounds();
    let mut score = 0;

    for x in 0..x_bound {
        for y in 0..y_bound {
            if map.0[x][y] == 0 {
                score += map.score_tile(x as isize, y as isize, false);
            }
        }
    }

    score
}

fn part2(map: &Map) -> usize {
    let mut map = map.clone();
    let (x_bound, y_bound) = map.bounds();
    let mut score = 0;

    for x in 0..x_bound {
        for y in 0..y_bound {
            if map.0[x][y] == 0 {
                score += map.score_tile(x as isize, y as isize, true);
            }
        }
    }

    score
}

aoc_main!(10, common, part1, part2);
