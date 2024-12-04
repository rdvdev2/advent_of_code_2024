use std::{
    convert::{identity, Infallible},
    str::FromStr,
};

use advent_of_code_2024::aoc_main;
use itertools::Itertools;

#[derive(Clone, Copy)]
enum Direction {
    N,
    NE,
    E,
    SE,
    S,
    SW,
    W,
    NW,
}

impl Direction {
    const ALL: [Direction; 8] = [
        Self::N,
        Self::NE,
        Self::E,
        Self::SE,
        Self::S,
        Self::SW,
        Self::W,
        Self::NW,
    ];

    fn to_offsets(&self) -> (isize, isize) {
        match *self {
            Self::N => (0, 1),
            Self::NE => (1, 1),
            Self::E => (1, 0),
            Self::SE => (1, -1),
            Self::S => (0, -1),
            Self::SW => (-1, -1),
            Self::W => (-1, 0),
            Self::NW => (-1, 1),
        }
    }
}

struct Crosswords(Vec<Vec<char>>);

impl Crosswords {
    fn get_at_pos(&self, x: usize, y: usize) -> Option<char> {
        self.0.get(x).and_then(|row| row.get(y)).copied()
    }

    fn is_xmas(&self, x: usize, y: usize, direction: Direction) -> bool {
        let (x, y) = (x as isize, y as isize);
        let (x_offset, y_offset) = direction.to_offsets();

        let seq = [
            self.get_at_pos(x as usize, y as usize),
            self.get_at_pos((x + x_offset) as usize, (y + y_offset) as usize),
            self.get_at_pos((x + 2 * x_offset) as usize, (y + 2 * y_offset) as usize),
            self.get_at_pos((x + 3 * x_offset) as usize, (y + 3 * y_offset) as usize),
        ];

        seq == [Some('X'), Some('M'), Some('A'), Some('S')]
    }

    fn is_x_mas(&self, x: usize, y: usize) -> bool {
        if self.get_at_pos(x, y) != Some('A') {
            return false;
        }

        let (x, y) = (x as isize, y as isize);

        let corners = [
            self.get_at_pos((x - 1) as usize, (y - 1) as usize),
            self.get_at_pos((x + 1) as usize, (y - 1) as usize),
            self.get_at_pos((x - 1) as usize, (y + 1) as usize),
            self.get_at_pos((x + 1) as usize, (y + 1) as usize),
        ];

        let m_count = corners.iter().filter(|x| matches!(x, Some('M'))).count();
        let s_count = corners.iter().filter(|x| matches!(x, Some('S'))).count();

        m_count == 2 && s_count == 2 && corners[0] != corners[3]
    }

    fn count_xmas(&self, x: usize, y: usize) -> usize {
        Direction::ALL
            .iter()
            .map(|direction| self.is_xmas(x, y, *direction))
            .filter(|x| *x)
            .count()
    }

    fn get_dimensions(&self) -> (usize, usize) {
        (self.0.len(), self.0[0].len())
    }
}

impl FromStr for Crosswords {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(
            s.lines()
                .map(|line| line.chars().collect_vec())
                .collect_vec(),
        ))
    }
}

fn common(input: String) -> Crosswords {
    input.parse().unwrap()
}

fn part1(crosswords: &Crosswords) -> usize {
    let (x_size, y_size) = crosswords.get_dimensions();
    (0..x_size)
        .cartesian_product(0..y_size)
        .map(|(x, y)| crosswords.count_xmas(x, y))
        .sum()
}

fn part2(crosswords: &Crosswords) -> usize {
    let (x_size, y_size) = crosswords.get_dimensions();
    (0..x_size)
        .cartesian_product(0..y_size)
        .map(|(x, y)| crosswords.is_x_mas(x, y))
        .filter(|x| *x)
        .count()
}

aoc_main!(4, common, part1, part2);
