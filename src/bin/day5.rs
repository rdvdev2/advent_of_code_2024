use std::{
    collections::{BTreeMap, BTreeSet},
    iter,
};

use advent_of_code_2024::aoc_main;
use itertools::Itertools;

// x maps to all elements after x
type Rules = BTreeMap<usize, BTreeSet<usize>>;
type Updates = Vec<Vec<usize>>;

fn common(input: String) -> (Rules, Updates) {
    let mut collecting_rules = true;
    let mut rules = Rules::default();
    let mut updates = Updates::default();

    for line in input.lines() {
        if line.is_empty() {
            collecting_rules = false;
            continue;
        }

        if collecting_rules {
            let (x, after_x) = line.split_once('|').unwrap();
            let x = x.parse().unwrap();
            let after_x = after_x.parse().unwrap();

            if let Some(rule) = rules.get_mut(&x) {
                rule.insert(after_x);
            } else {
                rules.insert(x, BTreeSet::from_iter(iter::once(after_x)));
            }
        } else {
            updates.push(line.split(',').map(|x| x.parse().unwrap()).collect_vec());
        }
    }

    (rules, updates)
}

fn part1((rules, updates): &(Rules, Updates)) -> usize {
    let mut total = 0;

    'next_update: for update in updates {
        let mut visited = BTreeSet::new();

        for page in update {
            if let Some(rule) = rules.get(page) {
                if !rule.is_disjoint(&visited) {
                    continue 'next_update;
                }
            }

            visited.insert(*page);
        }

        total += update[update.len() / 2];
    }

    total
}

fn part2((rules, updates): &(Rules, Updates)) -> usize {
    let mut unordered = Vec::new();

    'next_update: for update in updates {
        let mut visited = BTreeSet::new();

        for page in update {
            if let Some(rule) = rules.get(page) {
                if !rule.is_disjoint(&visited) {
                    unordered.push(update);
                    continue 'next_update;
                }
            }

            visited.insert(*page);
        }
    }

    let mut total = 0;

    for update in unordered {
        let mut pending = BTreeSet::from_iter(update.to_owned());
        let mut ordered = Vec::with_capacity(pending.len());

        while !pending.is_empty() {
            let next = pending
                .iter()
                .filter(|x| rules.get(x).map_or(true, |r| r.is_disjoint(&pending)))
                .next()
                .cloned()
                .unwrap();

            pending.remove(&next);
            ordered.push(next);
        }

        total += ordered[ordered.len() / 2];
    }

    total
}

aoc_main!(5, common, part1, part2);
