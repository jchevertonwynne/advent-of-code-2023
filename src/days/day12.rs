use fxhash::FxHashMap;
use nom::{bytes::complete::tag, InputTakeAtPosition};

use crate::{DayResult, IntoDayResult};

pub fn solve(input: &str) -> anyhow::Result<DayResult> {
    let mut p1 = 0;
    let mut p2 = 0;

    let mut remaining_placeable_spots = vec![];
    let mut remaining_hashes = vec![];
    let mut cache = FxHashMap::default();

    for line in input.lines() {
        let (nums, line) = line
            .split_at_position::<_, nom::error::Error<&str>>(|v| v == ' ')
            .map_err(|err| anyhow::anyhow!("{err}"))?;
        let (_, mut nums) = nom::multi::separated_list1::<_, _, _, nom::error::Error<&[u8]>, _, _>(
            tag(","),
            nom::character::complete::u32,
        )(&nums.as_bytes()[1..])
        .map_err(|err| anyhow::anyhow!("{err}"))?;

        remaining_hashes.clear();
        line.as_bytes()
            .iter()
            .rev()
            .scan(0, |state, &v| {
                if v == b'#' {
                    *state += 1;
                }
                Some(*state)
            })
            .for_each(|v| remaining_hashes.push(v));
        remaining_hashes.reverse();
        remaining_placeable_spots.clear();
        line.as_bytes()
            .iter()
            .rev()
            .scan(0, |state, &v| {
                if matches!(v, b'#' | b'?') {
                    *state += 1;
                } else {
                    *state = 0;
                }
                Some(*state)
            })
            .for_each(|v| remaining_placeable_spots.push(v));
        remaining_placeable_spots.reverse();

        cache.clear();
        p1 += solver(
            line.as_bytes(),
            0,
            &nums,
            &mut cache,
            &remaining_hashes,
            &remaining_placeable_spots,
        );

        let p2_line = format!("{line}?{line}?{line}?{line}?{line}");
        let l = nums.len();
        for _ in 0..4 {
            for i in 0..l {
                nums.push(nums[i]);
            }
        }

        remaining_hashes.clear();
        p2_line
            .as_bytes()
            .iter()
            .rev()
            .scan(0, |state, &v| {
                if v == b'#' {
                    *state += 1;
                }
                Some(*state)
            })
            .for_each(|v| remaining_hashes.push(v));
        remaining_hashes.reverse();
        remaining_placeable_spots.clear();
        p2_line
            .as_bytes()
            .iter()
            .rev()
            .scan(0, |state, &v| {
                if matches!(v, b'#' | b'?') {
                    *state += 1;
                } else {
                    *state = 0;
                }
                Some(*state)
            })
            .for_each(|v| remaining_placeable_spots.push(v));
        remaining_placeable_spots.reverse();

        cache.clear();
        p2 += solver(
            p2_line.as_bytes(),
            0,
            &nums,
            &mut cache,
            &remaining_hashes,
            &remaining_placeable_spots,
        );
    }

    (p1, p2).into_result()
}

fn solver(
    line: &[u8],
    ind: usize,
    rem: &[u32],
    cache: &mut FxHashMap<(usize, usize), usize>,
    remaining_hashes: &[usize],
    remaining_placeable_spots: &[usize],
) -> usize {
    let [r, rest @ ..] = rem else {
        return (remaining_hashes.get(ind).copied().unwrap_or(0) == 0) as usize;
    };

    if let Some(entry) = cache.get(&(ind, rem.len())) {
        return *entry;
    }

    let r = *r as usize;
    let mut res = 0;

    for start_ind in ind..line.len() {
        let tile = line[start_ind];
        if remaining_placeable_spots[start_ind] >= r
            && line
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
                remaining_placeable_spots,
            );
        }

        if tile == b'#' {
            break;
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
