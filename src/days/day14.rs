use std::collections::hash_map::Entry;

use fxhash::FxHashMap;

use crate::{DayResult, IntoDayResult};

pub fn solve(input: &str) -> anyhow::Result<DayResult> {
    let mut world = input
        .lines()
        .map(|line| line.as_bytes().to_vec())
        .collect::<Vec<_>>();

    north(&mut world);

    let p1 = score(&world);
    west(&mut world);
    south(&mut world);
    east(&mut world);

    let mut seen = FxHashMap::default();

    let mut p2 = 0;

    for cycles in 1.. {
        match seen.entry((repr(&world), score(&world))) {
            Entry::Occupied(prev_cycles) => {
                let prev_cycles = prev_cycles.get();
                let cycle_period = cycles - prev_cycles;
                let rem = (1_000_000_000 - prev_cycles) % cycle_period;
                for _ in 0..rem {
                    cycle(&mut world);
                }
                p2 = score(&world);
                break;
            }
            Entry::Vacant(v) => {
                v.insert(cycles);
            }
        }

        cycle(&mut world);
    }

    (p1, p2).into_result()
}

fn repr(world: &[Vec<u8>]) -> Vec<u8> {
    let mut res = Vec::with_capacity((world.iter().map(|l| l.len()).sum::<usize>() / 8) + 1);

    let mut curr = 0;
    for (i, &t) in world.iter().flat_map(|l| l.iter()).enumerate() {
        curr = (curr << 1) + (t == b'O') as u8;
        if i % 8 == 0 {
            res.push(curr);
            curr = 0;
        }
    }

    res.push(curr);

    res
}

fn score(world: &[Vec<u8>]) -> usize {
    world
        .iter()
        .rev()
        .zip(1..)
        .map(|(line, score)| line.iter().filter(|&&t| t == b'O').count() * score)
        .sum::<usize>()
}

fn cycle(world: &mut [Vec<u8>]) {
    north(world);
    west(world);
    south(world);
    east(world);
}

fn north(world: &mut [Vec<u8>]) {
    for y in 0..world.len() {
        for x in 0..world[0].len() {
            let mut y = y;
            while y != 0 {
                if world[y][x] == b'O' && world[y - 1][x] == b'.' {
                    world[y - 1][x] = b'O';
                    world[y][x] = b'.';
                } else {
                    break;
                }
                y -= 1;
            }
        }
    }
}

fn south(world: &mut [Vec<u8>]) {
    for y in (0..world.len()).rev() {
        for x in 0..world[0].len() {
            let mut y = y;
            while y != world.len() - 1 {
                if world[y][x] == b'O' && world[y + 1][x] == b'.' {
                    world[y + 1][x] = b'O';
                    world[y][x] = b'.';
                } else {
                    break;
                }
                y += 1;
            }
        }
    }
}

fn west(world: &mut [Vec<u8>]) {
    for x in 0..world[0].len() {
        for row in world.iter_mut() {
            let mut x = x;
            while x != 0 {
                if row[x] == b'O' && row[x - 1] == b'.' {
                    row[x - 1] = b'O';
                    row[x] = b'.';
                } else {
                    break;
                }
                x -= 1;
            }
        }
    }
}

fn east(world: &mut [Vec<u8>]) {
    for x in (0..world[0].len()).rev() {
        let row_len = world[0].len();
        for row in world.iter_mut().rev() {
            let mut x = x;
            while x != row_len - 1 {
                if row[x] == b'O' && row[x + 1] == b'.' {
                    row[x + 1] = b'O';
                    row[x] = b'.';
                } else {
                    break;
                }
                x += 1;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{days::day14::solve, IntoDayResult};

    #[test]
    fn works_for_example() {
        const INPUT: &str = include_str!("../../input/day14_test.txt");
        let solution = solve(INPUT).unwrap();
        assert_eq!((136, 64).into_day_result(), solution);
    }

    #[test]
    fn works_for_input() {
        const INPUT: &str = include_str!("../../input/day14.txt");
        let solution = solve(INPUT).unwrap();
        assert_eq!((108_813, 104_533).into_day_result(), solution);
    }
}
