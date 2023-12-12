use fxhash::FxHashMap;
use nom::{bytes::complete::tag, InputTakeAtPosition};

use crate::{DayResult, IntoDayResult};

pub fn solve(input: &str) -> anyhow::Result<DayResult> {
    let mut p1 = 0;
    let mut p2 = 0;

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

        cache.clear();
        p1 += solver(line.as_bytes(), 0, &nums, &mut cache);

        let p2_line = format!("{line}?{line}?{line}?{line}?{line}");
        let l = nums.len();
        for _ in 0..4 {
            for i in 0..l {
                nums.push(nums[i]);
            }
        }

        cache.clear();
        p2 += solver(p2_line.as_bytes(), 0, &nums, &mut cache);
    }

    (p1, p2).into_result()
}

fn solver(
    line: &[u8],
    ind: usize,
    rem: &[u32],
    cache: &mut FxHashMap<(usize, usize), usize>,
) -> usize {
    if let Some(entry) = cache.get(&(ind, rem.len())) {
        return *entry;
    }

    let [r, rest @ ..] = rem else {
        if line[std::cmp::min(ind, line.len())..]
            .iter()
            .all(|&b| b != b'#')
        {
            return 1;
        } else {
            return 0;
        }
    };

    let r = *r as usize;
    let mut res = 0;

    for start_ind in ind..line.len() {
        let tile = line[start_ind];
        if (start_ind..start_ind + r).all(|group_ind| {
            line.get(group_ind)
                .map(|&t| matches!(t, b'?' | b'#'))
                .unwrap_or(false)
        }) && line
            .get(start_ind + r)
            .map(|&b| matches!(b, b'.' | b'?'))
            .unwrap_or(true)
            && start_ind
                .checked_sub(1)
                .and_then(|i| line.get(i).map(|&t| matches!(t, b'.' | b'?')))
                .unwrap_or(true)
        {
            res += solver(line, start_ind + r + 1, rest, cache);
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
