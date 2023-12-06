use anyhow::Context;
use itertools::Itertools;
use nom::{bytes::complete::tag, combinator::map, multi::separated_list1, sequence::tuple};

use crate::{DayResult, IntoDayResult};

pub fn solve(input: &str) -> anyhow::Result<DayResult> {
    let input = &input.as_bytes()[7..];
    let (input, seeds) = nom::multi::separated_list1::<_, _, _, nom::error::Error<&[u8]>, _, _>(
        tag(" "),
        nom::character::complete::u64,
    )(input)
    .map_err(|err| anyhow::anyhow!("{err}"))?;

    let mut block_maps = arrayvec::ArrayVec::<BlockMap, 7>::new();

    let input = &input["seed-to-soil-map:".len() + 3..];
    let (input, mapping) = parse_block_map(input).map_err(|err| anyhow::anyhow!("{err}"))?;
    block_maps.push(mapping);
    let input = &input["soil-to-fertilizer map:".len() + 3..];
    let (input, mapping) = parse_block_map(input).map_err(|err| anyhow::anyhow!("{err}"))?;
    block_maps.push(mapping);
    let input = &input["fertilizer-to-water map:".len() + 3..];
    let (input, mapping) = parse_block_map(input).map_err(|err| anyhow::anyhow!("{err}"))?;
    block_maps.push(mapping);
    let input = &input["water-to-light map:".len() + 3..];
    let (input, mapping) = parse_block_map(input).map_err(|err| anyhow::anyhow!("{err}"))?;
    block_maps.push(mapping);
    let input = &input["light-to-temperature map:".len() + 3..];
    let (input, mapping) = parse_block_map(input).map_err(|err| anyhow::anyhow!("{err}"))?;
    block_maps.push(mapping);
    let input = &input["temperature-to-humidity map:".len() + 3..];
    let (input, mapping) = parse_block_map(input).map_err(|err| anyhow::anyhow!("{err}"))?;
    block_maps.push(mapping);
    let input = &input["humidity-to-location map:".len() + 3..];
    let (_, mapping) = parse_block_map(input).map_err(|err| anyhow::anyhow!("{err}"))?;
    block_maps.push(mapping);

    for bm in block_maps.iter_mut() {
        bm.mappings.sort_unstable_by_key(|m| m.src_start);
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

fn solve_p2_2(block_maps: &[BlockMap], min: u64, max: u64) -> Option<u64> {
    if let [bm, rest @ ..] = block_maps {
        let mut res = None;

        let fmap = &bm.mappings[0];
        let clamped_min = min;
        let clamped_max = std::cmp::min(max, fmap.src_start);
        if clamped_min < clamped_max {
            if let Some(new_res) = solve_p2_2(rest, clamped_min, clamped_max) {
                res = match res {
                    Some(res) => Some(std::cmp::min(res, new_res)),
                    None => Some(new_res),
                }
            }
        }

        let lmap = &bm.mappings[bm.mappings.len() - 1];
        let lmin = lmap.src_start + lmap.width;
        let clamped_min = std::cmp::max(min, lmin);
        let clamped_max = max;
        if clamped_min < clamped_max {
            if let Some(new_res) = solve_p2_2(rest, clamped_min, clamped_max) {
                res = match res {
                    Some(res) => Some(std::cmp::min(res, new_res)),
                    None => Some(new_res),
                }
            }
        }

        for m in &bm.mappings {
            let start = m.src_start;
            if start > max {
                break;
            }
            let end = m.src_start + m.width;
            let clamped_min = std::cmp::max(min, start);
            let clamped_max = std::cmp::min(max, end);
            if clamped_min < clamped_max {
                let new_min = clamped_min - m.src_start + m.dst_start;
                let new_max = clamped_max - m.src_start + m.dst_start;
                if let Some(new_res) = solve_p2_2(rest, new_min, new_max) {
                    res = match res {
                        Some(res) => Some(std::cmp::min(res, new_res)),
                        None => Some(new_res),
                    }
                }
            }
        }

        for (a, b) in bm.mappings.iter().tuple_windows() {
            let start = a.src_start + a.width;
            if start > max {
                break;
            }
            let end = b.src_start;
            let new_min = std::cmp::max(min, start);
            let new_max = std::cmp::min(max, end);
            if new_min < new_max {
                if let Some(new_res) = solve_p2_2(rest, new_min, new_max) {
                    res = match res {
                        Some(res) => Some(std::cmp::min(res, new_res)),
                        None => Some(new_res),
                    }
                }
            }
        }

        res
    } else {
        Some(min)
    }
}

fn parse_block_map(input: &[u8]) -> nom::IResult<&[u8], BlockMap> {
    map(
        separated_list1(
            tag("\n"),
            map(
                tuple((
                    nom::character::complete::u64,
                    tag(" "),
                    nom::character::complete::u64,
                    tag(" "),
                    nom::character::complete::u64,
                )),
                |(dst_start, _, src_start, _, width)| Mapping {
                    src_start,
                    dst_start,
                    width,
                },
            ),
        ),
        |mappings| BlockMap { mappings },
    )(input)
}

#[derive(Debug)]
struct BlockMap {
    mappings: Vec<Mapping>,
}

impl BlockMap {
    fn apply(&self, val: u64) -> u64 {
        for mapping in &self.mappings {
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
