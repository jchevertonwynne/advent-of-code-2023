use std::collections::hash_map::Entry;

use anyhow::Context;
use fxhash::FxHashMap;

use crate::{DayResult, IntoDayResult};

pub fn solve(input: &str, is_test: bool) -> anyhow::Result<DayResult> {
    let input = input.as_bytes();
    let mut it = input.split(|&b| b == b'\n');
    let instructions = it.next().context("should have first line")?;
    let mut input = &input[instructions.len() + 2..];

    let mut map = fxhash::FxHashMap::default();

    while !input.is_empty() {
        let (n, node) = parse_line(input);
        input = &input[17..];
        map.insert(n, node);
    }

    let mut p1 = 0;
    if !is_test {
        let instructions_iter = instructions.iter().cycle();
        let mut curr = if is_test { b"11A" } else { b"AAA" }.as_slice();
        for &instruction in instructions_iter {
            if instruction == b'L' {
                curr = map[curr].left;
            } else {
                curr = map[curr].right;
            }
            p1 += 1;
            if curr == b"ZZZ" {
                break;
            }
        }
    }

    let mut cache = FxHashMap::default();
    let p2 = map
        .keys()
        .filter_map(|k| if k.ends_with(b"A") { Some(*k) } else { None })
        .map(|k| to_z_loop(instructions, k, &map, &mut cache))
        .reduce(num::integer::lcm)
        .context("iter has 1+ elements")?;

    (p1, p2).into_result()
}

fn to_z_loop<'a>(
    instructions: &'a [u8],
    start: &'a [u8],
    map: &FxHashMap<&'a [u8], Node<'a>>,
    cache: &mut FxHashMap<&'a [u8], usize>,
) -> i128 {
    cache.clear();
    let instructions_iter = instructions.iter().cycle();
    let mut curr = start;
    for (i, &instruction) in instructions_iter.enumerate() {
        if curr[2] == b'Z' {
            match cache.entry(curr) {
                Entry::Occupied(entry) => {
                    let last_seen_at = *entry.get();
                    return last_seen_at as i128;
                }
                Entry::Vacant(v) => v.insert(i),
            };
        }
        curr = if instruction == b'L' {
            map[curr].left
        } else {
            map[curr].right
        };
    }

    unreachable!("lmao")
}

fn parse_line(line: &[u8]) -> (&[u8], Node) {
    (
        &line[..3],
        Node {
            left: &line[7..10],
            right: &line[12..15],
        },
    )
}

#[derive(Debug, Clone, Copy)]
struct Node<'a> {
    left: &'a [u8],
    right: &'a [u8],
}

#[cfg(test)]
mod tests {
    use crate::{days::day08::solve, IntoDayResult};

    #[test]
    fn works_for_example() {
        const INPUT: &str = include_str!("../../input/day08_test.txt");
        let solution = solve(INPUT, true).unwrap();
        assert_eq!((0, 6).into_day_result(), solution);
    }

    #[test]
    fn works_for_input() {
        const INPUT: &str = include_str!("../../input/day08.txt");
        let solution = solve(INPUT, false).unwrap();
        assert_eq!(
            (12_083, 13_385_272_668_829_i128).into_day_result(),
            solution
        );
    }
}
