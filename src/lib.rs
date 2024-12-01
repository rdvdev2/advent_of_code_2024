use std::{borrow::Borrow, env};

use reqwest::cookie::Jar;

pub fn get_puzzle_input(day: usize) -> String {
    let aoc_token = env::var("AOC_TOKEN").unwrap();
    let jar = Jar::default();
    jar.add_cookie_str(
        &format!("session={aoc_token}"),
        &"https://adventofcode.com".parse().unwrap(),
    );

    let client = reqwest::blocking::Client::builder()
        .cookie_provider(jar.into())
        .build()
        .unwrap();

    assert_ne!(day, 0);

    client
        .get(format!("https://adventofcode.com/2024/day/{day}/input"))
        .send()
        .unwrap()
        .text()
        .unwrap()
}

pub fn main<C, CB, FC, F1, F2>(day: usize, common: FC, part1: F1, part2: F2)
where
    C: Borrow<CB>,
    FC: FnOnce(String) -> C,
    F1: FnOnce(&CB) -> usize,
    F2: FnOnce(&CB) -> usize,
{
    let input = get_puzzle_input(day);
    let c = common(input);

    println!("Output for day {day}");
    println!("Part 1: {}", part1(c.borrow()));
    println!("Part 2: {}", part2(c.borrow()));
}

#[macro_export]
macro_rules! aoc_main {
    ($day:literal, $common:ident, $part1:ident, $part2:ident) => {
        fn main() {
            advent_of_code_2024::main($day, $common, $part1, $part2)
        }
    };
}
