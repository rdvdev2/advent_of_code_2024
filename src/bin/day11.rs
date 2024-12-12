use std::collections::LinkedList;

use advent_of_code_2024::aoc_main;

fn common(input: String) -> LinkedList<usize> {
    input
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect()
}

fn iterate(stones: &mut LinkedList<usize>) {
    let mut left = LinkedList::new();
    let mut right = stones.split_off(0);

    while !right.is_empty() {
        let tmp = right.split_off(1);
        left.append(&mut right);
        right = tmp;

        let stone = *left.back().unwrap();
        if stone == 0 {
            *left.back_mut().unwrap() = 1;
        } else {
            let len = stone.checked_ilog10().unwrap_or(0) + 1;

            if len % 2 == 0 {
                *left.back_mut().unwrap() = stone / 10usize.pow(len / 2);
                left.push_back(stone % 10usize.pow(len / 2));
            } else {
                *left.back_mut().unwrap() *= 2024;
            }
        }
    }

    stones.append(&mut left);
}

fn part1(stones: &LinkedList<usize>) -> usize {
    let mut stones = stones.clone();

    for _ in 0..25 {
        iterate(&mut stones);
    }

    stones.len()
}

fn part2(stones: &LinkedList<usize>) -> usize {
    let mut stones = stones.clone();

    for i in 0..75 {
        println!("{i}");
        iterate(&mut stones);
    }

    stones.len()
}

aoc_main!(11, common, part1, part2);
