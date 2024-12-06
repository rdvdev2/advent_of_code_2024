use std::collections::BTreeSet;

use advent_of_code_2024::aoc_main;
use itertools::Itertools;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Facing {
    Up,
    Down,
    Left,
    Right,
}

impl Facing {
    fn turn_right(&self) -> Self {
        match self {
            Self::Up => Self::Right,
            Self::Right => Self::Down,
            Self::Down => Self::Left,
            Self::Left => Self::Up,
        }
    }

    fn offset(&self) -> (isize, isize) {
        match self {
            Self::Up => (0, -1),
            Self::Down => (0, 1),
            Self::Left => (-1, 0),
            Self::Right => (1, 0),
        }
    }

    fn apply(&self, (x, y): (usize, usize)) -> (usize, usize) {
        let (x_offset, y_offset) = self.offset();
        (
            (x as isize + x_offset) as usize,
            (y as isize + y_offset) as usize,
        )
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Tile {
    Unvisited,
    Visited,
    Obstable,
    Guard(Facing),
}

impl From<char> for Tile {
    fn from(value: char) -> Self {
        match value {
            '.' => Self::Unvisited,
            '#' => Self::Obstable,
            '^' => Self::Guard(Facing::Up),
            '>' => Self::Guard(Facing::Right),
            'v' => Self::Guard(Facing::Down),
            '<' => Self::Guard(Facing::Left),
            _ => unreachable!(),
        }
    }
}

// transposed coordinate system! (y, x)
#[derive(Clone)]
struct Board {
    tiles: Vec<Vec<Tile>>,
    guard_pos: Option<(usize, usize)>,
}

impl Board {
    fn bounds(&self) -> (usize, usize) {
        (self.tiles.len(), self.tiles[0].len())
    }

    fn pos_is_valid(&self, (x, y): (usize, usize)) -> bool {
        y < self.tiles.len() && x < self.tiles[0].len()
    }

    fn get_tile(&self, (x, y): (usize, usize)) -> Tile {
        self.tiles[y][x]
    }

    fn get_guard_direction(&self) -> Option<Facing> {
        match self.get_tile(self.guard_pos?) {
            Tile::Guard(direction) => Some(direction),
            _ => unreachable!(),
        }
    }

    fn set_tile(&mut self, (x, y): (usize, usize), tile: Tile) {
        self.tiles[y][x] = tile;
    }

    fn iterate(&mut self) -> bool {
        let Some(guard_pos) = self.guard_pos else {
            return false;
        };

        let Tile::Guard(guard_direction) = self.get_tile(guard_pos) else {
            unreachable!();
        };

        let next_pos = guard_direction.apply(guard_pos);

        if !self.pos_is_valid(next_pos) {
            self.set_tile(guard_pos, Tile::Visited);
            self.guard_pos = None;
            return false;
        }

        if self.get_tile(next_pos) == Tile::Obstable {
            self.set_tile(guard_pos, Tile::Guard(guard_direction.turn_right()));
            return true;
        }

        self.set_tile(guard_pos, Tile::Visited);
        self.set_tile(next_pos, Tile::Guard(guard_direction));
        self.guard_pos = Some(next_pos);
        return true;
    }

    fn count_visited(&self) -> usize {
        self.tiles
            .iter()
            .flatten()
            .filter(|tile| **tile == Tile::Visited)
            .count()
    }
}

fn common(input: String) -> Board {
    let tiles = input
        .lines()
        .map(|line| line.chars().map(|c| c.into()).collect_vec())
        .collect_vec();

    let guard_pos = tiles
        .iter()
        .enumerate()
        .flat_map(|(y, row)| {
            row.iter()
                .enumerate()
                .filter(|(_, tile)| matches!(tile, Tile::Guard(_)))
                .map(|(x, _)| (x, y))
                .next()
        })
        .next();

    assert!(guard_pos.is_some_and(|(x, y)| matches!(tiles[y][x], Tile::Guard(_))));

    Board { tiles, guard_pos }
}

fn part1(board: &Board) -> usize {
    let mut board = board.clone();

    while board.iterate() {}

    board.count_visited()
}

fn part2(board: &Board) -> usize {
    let (x_bound, y_bound) = board.bounds();
    let mut sneaky_obstacles = 0;

    'next_pos: for (x, y) in (0..x_bound).cartesian_product(0..y_bound) {
        if board.get_tile((x, y)) != Tile::Unvisited {
            continue 'next_pos;
        }

        let mut board = board.clone();
        board.set_tile((x, y), Tile::Obstable);

        let mut visited = BTreeSet::new();
        visited.insert((
            board.guard_pos.unwrap(),
            board.get_guard_direction().unwrap(),
        ));

        while board.iterate() {
            let new_pos = (
                board.guard_pos.unwrap(),
                board.get_guard_direction().unwrap(),
            );

            if visited.contains(&new_pos) {
                sneaky_obstacles += 1;
                continue 'next_pos;
            }

            visited.insert(new_pos);
        }
    }

    sneaky_obstacles
}

aoc_main!(6, common, part1, part2);
