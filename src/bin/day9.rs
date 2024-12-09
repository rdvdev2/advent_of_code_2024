use advent_of_code_2024::aoc_main;
use itertools::repeat_n;

#[derive(Default, Clone)]
struct Disk {
    disk: Vec<Option<usize>>,
    next_file: usize,
}

impl Disk {
    fn add_file(&mut self, size: usize) {
        self.disk.extend(repeat_n(Some(self.next_file), size));
        self.next_file += 1;
    }

    fn add_space(&mut self, size: usize) {
        self.disk.extend(repeat_n(None, size));
    }

    fn swap_blocks(&mut self, idx1: usize, idx2: usize) {
        self.disk.swap(idx1, idx2);
    }

    fn checksum(&self) -> usize {
        self.disk
            .iter()
            .enumerate()
            .flat_map(|(pos, id)| id.map(|id| pos * id))
            .sum()
    }
}

fn common(input: String) -> Disk {
    let mut disk = Disk::default();
    let mut next_is_file = true;

    for c in input.chars() {
        let Some(size) = c.to_digit(10) else {
            continue;
        };

        if next_is_file {
            disk.add_file(size as usize);
        } else {
            disk.add_space(size as usize);
        }

        next_is_file = !next_is_file;
    }

    disk
}

fn part1(disk: &Disk) -> usize {
    let mut disk = disk.clone();

    let mut next_space = 0;
    let mut next_block = disk.disk.len() - 1;

    while next_space < next_block {
        while disk.disk[next_space].is_some() {
            next_space += 1;
        }
        while disk.disk[next_block].is_none() {
            next_block -= 1;
        }

        if next_space > next_block {
            break;
        }

        disk.swap_blocks(next_space, next_block);
    }

    disk.checksum()
}

fn part2(disk: &Disk) -> usize {
    let mut disk = disk.clone();

    let mut next_block = disk.disk.len() - 1;
    let mut first_space = 0;

    'all: while next_block > 0 {
        while disk.disk[next_block].is_none() {
            next_block -= 1;
        }
        while disk.disk[first_space].is_some() {
            first_space += 1;
        }

        let mut file_size = 1;
        while disk.disk[next_block - 1] == disk.disk[next_block] {
            next_block -= 1;
            file_size += 1;
            if next_block == 0 {
                break 'all;
            }
        }

        let mut next_space = first_space;
        'search: while next_space < next_block {
            for i in 0..file_size {
                if disk.disk[next_space + i].is_some() {
                    next_space += i + 1;
                    continue 'search;
                }
            }

            for i in 0..file_size {
                disk.swap_blocks(next_block + i, next_space + i);
            }
            break;
        }
        next_block -= 1;
    }

    disk.checksum()
}

aoc_main!(9, common, part1, part2);
