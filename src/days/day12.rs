use std::collections::VecDeque;

use fxhash::FxHashMap;
use nom::{bytes::complete::tag, InputTakeAtPosition};

use crate::{DayResult, IntoDayResult};

pub fn solve(input: &str) -> anyhow::Result<DayResult> {
    let mut p1 = 0;
    let mut p2 = 0;

    let mut tiles_to_next_placeable = VecDeque::new();
    let mut remaining_consecutive_placeable_spots = VecDeque::new();
    let mut remaining_hashes = VecDeque::new();
    let mut remaining_placeable_tiles = VecDeque::new();
    let mut cache = FxHashMap::default();

    for line in input.lines() {
        let (nums, line) = line
            .split_at_position::<_, nom::error::Error<&str>>(|v| v == ' ')
            .map_err(|err| anyhow::anyhow!("{err}"))?;
        let (_, mut nums) = nom::multi::separated_list1::<_, _, _, nom::error::Error<&[u8]>, _, _>(
            tag(","),
            nom::character::complete::u64,
        )(&nums.as_bytes()[1..])
        .map_err(|err| anyhow::anyhow!("{err}"))?;

        run_part(
            line.as_bytes(),
            &mut remaining_hashes,
            &mut remaining_consecutive_placeable_spots,
            &mut tiles_to_next_placeable,
            &mut remaining_placeable_tiles,
            &mut cache,
            &nums,
            &mut p1,
        );

        let mut p2_line = Vec::with_capacity(5 * line.len() + 4);
        p2_line.extend_from_slice(line.as_bytes());
        for _ in 0..4 {
            p2_line.push(b'?');
            p2_line.extend_from_slice(line.as_bytes());
        }
        let l = nums.len();
        nums.reserve(4 * l);
        for _ in 0..4 {
            for i in 0..l {
                nums.push(nums[i]);
            }
        }

        run_part(
            &p2_line,
            &mut remaining_hashes,
            &mut remaining_consecutive_placeable_spots,
            &mut tiles_to_next_placeable,
            &mut remaining_placeable_tiles,
            &mut cache,
            &nums,
            &mut p2,
        );
    }

    (p1, p2).into_result()
}

#[allow(clippy::too_many_arguments)]
fn run_part(
    line: &[u8],
    remaining_hashes: &mut VecDeque<usize>,
    remaining_consecutive_placeable_spots: &mut VecDeque<usize>,
    tiles_to_next_placeable: &mut VecDeque<usize>,
    remaining_placeable_tiles: &mut VecDeque<usize>,
    cache: &mut FxHashMap<(usize, usize), usize>,
    nums: &[u64],
    part_accum: &mut usize,
) {
    prepare_lookup_tables(
        line,
        remaining_hashes,
        remaining_consecutive_placeable_spots,
        tiles_to_next_placeable,
        remaining_placeable_tiles,
    );

    cache.clear();
    *part_accum += solver(
        line,
        0,
        nums,
        cache,
        remaining_hashes.make_contiguous(),
        remaining_consecutive_placeable_spots.make_contiguous(),
        tiles_to_next_placeable.make_contiguous(),
        remaining_placeable_tiles.make_contiguous(),
        nums.iter().map(|&v| v as usize).sum::<usize>(),
    );
}

fn prepare_lookup_tables(
    line: &[u8],
    remaining_hashes: &mut VecDeque<usize>,
    remaining_consecutive_placeable_spots: &mut VecDeque<usize>,
    tiles_to_next_placeable: &mut VecDeque<usize>,
    remaining_placeable_tiles: &mut VecDeque<usize>,
) {
    remaining_hashes.clear();
    line.iter()
        .rev()
        .scan(0, |state, &v| {
            if v == b'#' {
                *state += 1;
            }
            Some(*state)
        })
        .for_each(|v| remaining_hashes.push_front(v));

    remaining_consecutive_placeable_spots.clear();
    line.iter()
        .rev()
        .scan(0, |state, &v| {
            if matches!(v, b'#' | b'?') {
                *state += 1;
            } else {
                *state = 0;
            }
            Some(*state)
        })
        .for_each(|v| remaining_consecutive_placeable_spots.push_front(v));

    tiles_to_next_placeable.clear();
    line.iter()
        .rev()
        .scan((0, false), |(state, in_placeable), &v| {
            if matches!(v, b'#' | b'?') {
                *in_placeable = true;
                *state += 1;
            } else {
                if *in_placeable {
                    *state = 0;
                }
                *in_placeable = false;
                *state += 1;
            }
            Some(*state)
        })
        .for_each(|v| tiles_to_next_placeable.push_front(v));

    remaining_placeable_tiles.clear();
    line.iter()
        .rev()
        .scan(0, |state, &v| {
            if matches!(v, b'#' | b'?') {
                *state += 1;
            }
            Some(*state)
        })
        .for_each(|v| remaining_placeable_tiles.push_front(v));
}

#[allow(clippy::too_many_arguments)]
fn solver(
    line: &[u8],
    ind: usize,
    rem: &[u64],
    cache: &mut FxHashMap<(usize, usize), usize>,
    remaining_hashes: &[usize],
    remaining_consecutive_placeable_spots: &[usize],
    tiles_to_next_placeable: &[usize],
    remaining_placeable_tiles: &[usize],
    rem_len: usize,
) -> usize {
    let [r, rest @ ..] = rem else {
        return (remaining_hashes.get(ind).copied().unwrap_or(0) == 0) as usize;
    };

    if remaining_placeable_tiles
        .get(ind)
        .map(|&r| r < rem_len)
        .unwrap_or(true)
    {
        return 0;
    }

    if line
        .len()
        .checked_sub(ind)
        .map(|rlen| rlen < rem_len)
        .unwrap_or(false)
    {
        return 0;
    }

    if let Some(entry) = cache.get(&(ind, rem.len())) {
        return *entry;
    }

    let r = *r as usize;
    let mut res = 0;

    let mut start_ind = ind;

    while start_ind < line.len() {
        let tile = line[start_ind];
        let r2 = remaining_consecutive_placeable_spots[start_ind];
        if r2 >= r {
            if line
                .get(start_ind + r)
                .map(|&b| matches!(b, b'.' | b'?'))
                .unwrap_or(true)
                && start_ind
                    .checked_sub(1)
                    .and_then(|i| line.get(i).map(|&t| matches!(t, b'.' | b'?')))
                    .unwrap_or(true)
            {
                res += solver(
                    line,
                    start_ind + r + 1,
                    rest,
                    cache,
                    remaining_hashes,
                    remaining_consecutive_placeable_spots,
                    tiles_to_next_placeable,
                    remaining_placeable_tiles,
                    rem_len - r,
                );
            }

            if tile == b'#' {
                break;
            }

            start_ind += 1;
        } else {
            let jump = tiles_to_next_placeable[start_ind];
            let curr_rem_hashes = remaining_hashes[start_ind];
            start_ind += jump;
            if remaining_hashes
                .get(start_ind)
                .map(|&n| n < curr_rem_hashes)
                .unwrap_or(true)
            {
                break;
            }
        }
    }

    cache.insert((ind, rem.len()), res);

    res
}

#[cfg(test)]
mod tests {
    use crate::{days::day12::solve, IntoDayResult};

    #[test]
    fn works_for_example() {
        const INPUT: &str = include_str!("../../input/day12_test.txt");
        let solution = solve(INPUT).unwrap();
        assert_eq!((21, 525_152).into_day_result(), solution);
    }

    #[test]
    fn works_for_input() {
        const INPUT: &str = include_str!("../../input/day12.txt");
        let solution = solve(INPUT).unwrap();
        assert_eq!(
            (7_857, 28_606_137_449_920_usize).into_day_result(),
            solution
        );
    }
}
