use std::cmp::{max, min};

use anyhow::Context;
use itertools::Itertools;
use nom::bytes::complete::tag;

use crate::{DayResult, IntoDayResult};

pub fn solve(input: &str) -> anyhow::Result<DayResult> {
    let input = &input.as_bytes()[7..];
    let (input, seeds) = nom::multi::separated_list1::<_, _, _, nom::error::Error<&[u8]>, _, _>(
        tag(" "),
        nom::character::complete::u64,
    )(input)
    .map_err(|err| anyhow::anyhow!("{err}"))?;

    let mut entries = vec![];

    let mut block_maps = arrayvec::ArrayVec::<BlockMap, 7>::new();
    let input = &input["seed-to-soil-map:".len() + 3..];
    let (input, mapping) = parse_block_map_2(input, &mut entries);
    block_maps.push(mapping);
    let input = &input["soil-to-fertilizer map:".len() + 1..];
    let (input, mapping) = parse_block_map_2(input, &mut entries);
    block_maps.push(mapping);
    let input = &input["fertilizer-to-water map:".len() + 1..];
    let (input, mapping) = parse_block_map_2(input, &mut entries);
    block_maps.push(mapping);
    let input = &input["water-to-light map:".len() + 1..];
    let (input, mapping) = parse_block_map_2(input, &mut entries);
    block_maps.push(mapping);
    let input = &input["light-to-temperature map:".len() + 1..];
    let (input, mapping) = parse_block_map_2(input, &mut entries);
    block_maps.push(mapping);
    let input = &input["temperature-to-humidity map:".len() + 1..];
    let (input, mapping) = parse_block_map_2(input, &mut entries);
    block_maps.push(mapping);
    let input = &input["humidity-to-location map:".len() + 1..];
    let (_, mapping) = parse_block_map_2(input, &mut entries);
    block_maps.push(mapping);

    let mut offset = 0;
    for bm in block_maps.iter_mut() {
        entries[offset..offset + bm.map_count].sort_unstable_by_key(|m| m.src_start);
        offset += bm.map_count;
    }

    let mut offset = 0;
    for bm in block_maps.iter_mut() {
        bm.mappings = &entries[offset..offset + bm.map_count];
        offset += bm.map_count;
    }

    let p1 = seeds
        .iter()
        .map(|&seed| block_maps.iter().fold(seed, |seed, b| b.apply(seed)))
        .min()
        .context("expected at least 1 seed")?;

    let p2 = solve_p2(&seeds, &block_maps).context("there shoule be seeds")?;

    (p1, p2).into_result()
}

fn solve_p2(seeds: &[u64], block_maps: &[BlockMap]) -> Option<u64> {
    seeds
        .iter()
        .tuples()
        .map(|(&seed, &len)| solve_p2_2(block_maps, seed, seed + len))
        .min()
        .flatten()
}

macro_rules! subsolve_and_update {
    ($rest:tt, $new_min:tt, $new_max:tt, $res:tt) => {
        if let Some(new_res) = solve_p2_2($rest, $new_min, $new_max) {
            $res = match $res {
                Some(res) => Some(min(res, new_res)),
                None => Some(new_res),
            }
        }
    };
}

fn solve_p2_2(block_maps: &[BlockMap], curr_min: u64, curr_max: u64) -> Option<u64> {
    if let [bm, rest @ ..] = block_maps {
        let mut res = None;

        let fmap = &bm.mappings[0];
        let new_min = curr_min;
        let new_max = min(curr_max, fmap.src_start);
        if new_min < new_max {
            subsolve_and_update!(rest, new_min, new_max, res);
        }

        let lmap = &bm.mappings[bm.mappings.len() - 1];
        let lmin = lmap.src_start + lmap.width;
        let new_min = max(curr_min, lmin);
        let new_max = curr_max;
        if new_min < new_max {
            subsolve_and_update!(rest, new_min, new_max, res);
        }

        for m in bm.mappings {
            let start = m.src_start;
            if start > curr_max {
                break;
            }
            let end = m.src_start + m.width;
            let clamped_min = max(curr_min, start);
            let clamped_max = min(curr_max, end);
            if clamped_min < clamped_max {
                let new_min = clamped_min - m.src_start + m.dst_start;
                let new_max = clamped_max - m.src_start + m.dst_start;
                subsolve_and_update!(rest, new_min, new_max, res);
            }
        }

        for (a, b) in bm.mappings.iter().tuple_windows() {
            let start = a.src_start + a.width;
            if start > curr_max {
                break;
            }
            let end = b.src_start;
            let new_min = max(curr_min, start);
            let new_max = min(curr_max, end);
            if new_min < new_max {
                subsolve_and_update!(rest, new_min, new_max, res);
            }
        }

        res
    } else {
        Some(curr_min)
    }
}

fn parse_block_map_2<'a>(
    mut input: &'a [u8],
    entries: &mut Vec<Mapping>,
) -> (&'a [u8], BlockMap<'a>) {
    let mut map_count = 0;
    while !input.is_empty() && input[0] != b'\n' {
        let mut dst_start = 0;
        while input[0].is_ascii_digit() {
            dst_start = dst_start * 10 + (input[0] - b'0') as u64;
            input = &input[1..];
        }
        input = &input[1..];
        let mut src_start = 0;
        while input[0].is_ascii_digit() {
            src_start = src_start * 10 + (input[0] - b'0') as u64;
            input = &input[1..];
        }
        input = &input[1..];
        let mut width = 0;
        while input[0].is_ascii_digit() {
            width = width * 10 + (input[0] - b'0') as u64;
            input = &input[1..];
        }
        entries.push(Mapping {
            src_start,
            dst_start,
            width,
        });
        map_count += 1;
        input = &input[1..];
    }
    if !input.is_empty() {
        input = &input[1..];
    }
    (
        input,
        BlockMap {
            map_count,
            mappings: &[],
        },
    )
}

#[derive(Debug)]
struct BlockMap<'a> {
    map_count: usize,
    mappings: &'a [Mapping],
}

impl BlockMap<'_> {
    fn apply(&self, val: u64) -> u64 {
        for mapping in self.mappings {
            if let Some(val2) = mapping.map(val) {
                return val2;
            }
        }
        val
    }
}

#[derive(Debug, Clone, Copy)]
struct Mapping {
    src_start: u64,
    dst_start: u64,
    width: u64,
}

impl Mapping {
    fn map(self, val: u64) -> Option<u64> {
        let r = self.src_start..=(self.src_start + self.width);
        if r.contains(&val) {
            let n = val - self.src_start;
            Some(self.dst_start + n)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{days::day05::solve, IntoDayResult};

    #[test]
    fn works_for_example() {
        const INPUT: &str = include_str!("../../input/day05_test.txt");
        let solution = solve(INPUT).unwrap();
        assert_eq!((35, 46).into_day_result(), solution);
    }

    #[test]
    fn works_for_input() {
        const INPUT: &str = include_str!("../../input/day05.txt");
        let solution = solve(INPUT).unwrap();
        assert_eq!((251_346_198, 72_263_011).into_day_result(), solution);
    }
}
