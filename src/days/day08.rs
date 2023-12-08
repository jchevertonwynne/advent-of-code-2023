use std::{
    cell::Cell,
    collections::{hash_map::Entry, VecDeque},
};

use anyhow::Context;
use fxhash::FxHashMap;

use crate::{DayResult, IntoDayResult};

pub fn solve(input: &str, is_test: bool) -> anyhow::Result<DayResult> {
    let input = input.as_bytes();
    let mut it = input.split(|&b| b == b'\n');
    let instructions = it.next().context("should have first line")?;
    let mut input = &input[instructions.len() + 2..];

    let mut raw_nodes = VecDeque::with_capacity(input.len() / 17);

    while !input.is_empty() {
        let raw_node = NodeRaw {
            val: &input[..3],
            left: &input[7..10],
            right: &input[12..15],
        };

        input = &input[17..];
        raw_nodes.push_back(raw_node);
    }

    let b = bumpalo::Bump::new();

    let mut nodes = FxHashMap::default();
    while let Some(raw_node) = raw_nodes.pop_front() {
        let real_node = nodes.entry(raw_node.val).or_insert_with(|| {
            b.alloc(Node {
                val: raw_node.val,
                left: Cell::new(std::ptr::null()),
                right: Cell::new(std::ptr::null()),
            }) as &Node
        }) as &Node;
        if real_node.left.get().is_null() {
            let Some(left) = nodes.get(raw_node.left) else {
                raw_nodes.push_back(raw_node);
                continue;
            };
            real_node.left.set(*left);
        }
        if real_node.right.get().is_null() {
            let Some(right) = nodes.get(raw_node.right) else {
                raw_nodes.push_back(raw_node);
                continue;
            };
            real_node.right.set(*right);
        }
    }

    let mut p1 = 0;
    if !is_test {
        let instructions_iter = instructions.iter().cycle();
        let mut curr = nodes[b"AAA".as_slice()];
        let end = nodes[b"ZZZ".as_slice()];
        for &instruction in instructions_iter {
            if instruction == b'L' {
                curr = unsafe { &*curr.left.get() };
            } else {
                curr = unsafe { &*curr.right.get() };
            }
            p1 += 1;
            if std::ptr::eq(curr, end) {
                break;
            }
        }
    }

    let mut cache = FxHashMap::default();
    let p2 = nodes
        .iter()
        .filter_map(|(k, v)| if k.ends_with(b"A") { Some(*v) } else { None })
        .map(|v| to_z_loop(instructions, v, &mut cache))
        .reduce(num::integer::lcm)
        .context("iter has 1+ elements")?;

    (p1, p2).into_result()
}

fn to_z_loop<'a>(
    instructions: &'a [u8],
    start: &'a Node<'a>,
    cache: &mut FxHashMap<*const Node<'a>, usize>,
) -> i128 {
    cache.clear();
    let instructions_iter = instructions.iter().cycle();
    let mut curr = start;
    for (i, &instruction) in instructions_iter.enumerate() {
        if curr.val[2] == b'Z' {
            match cache.entry(curr as *const _) {
                Entry::Occupied(entry) => {
                    let last_seen_at = *entry.get();
                    return last_seen_at as i128;
                }
                Entry::Vacant(v) => v.insert(i),
            };
        }
        if instruction == b'L' {
            curr = unsafe { &*curr.left.get() };
        } else {
            curr = unsafe { &*curr.right.get() };
        };
    }

    unreachable!("lmao")
}

#[derive(Debug)]
struct NodeRaw<'a> {
    val: &'a [u8],
    left: &'a [u8],
    right: &'a [u8],
}

#[derive(Debug)]
struct Node<'a> {
    val: &'a [u8],
    left: Cell<*const Node<'a>>,
    right: Cell<*const Node<'a>>,
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
