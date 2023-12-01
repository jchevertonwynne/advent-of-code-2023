use anyhow::Context;

use crate::{DayResult, IntoDayResult};

pub fn solve(input: &str) -> anyhow::Result<DayResult> {
    let p1 = part_1(input)?;
    let p2 = part_2(input)?;
    (p1, p2).into_result()
}

fn part_2(input: &str) -> Result<u32, anyhow::Error> {
    let mut p2 = 0;
    for line in input.lines() {
        const NUMS: [&str; 9] = [
            "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
        ];
        let first_indices = NUMS.map(|n| line.find(n));
        let last_indices = NUMS.map(|n| line.rfind(n));
        let (first, last) = (first_indices.into_iter().zip(last_indices.into_iter()))
            .zip(1..)
            .try_fold(
                (None, None),
                |(mut first, mut last), ((first_match_index, last_match_index), num_index)| {
                    let c = char::try_from('0' as u32 + num_index as u32)?;
                    if let Some(index) = first_match_index {
                        first = first
                            .filter(|f: &(char, usize)| f.1 < index)
                            .or(Some((c, index)));
                    }
                    if let Some(index) = last_match_index {
                        last = last
                            .filter(|f: &(char, usize)| f.1 > index)
                            .or(Some((c, index)));
                    }
                    Ok::<_, anyhow::Error>((first, last))
                },
            )?;

        let (first, last) =
            line.chars()
                .enumerate()
                .fold((first, last), |(mut first, mut last), (index, c)| {
                    if c.is_ascii_digit() {
                        first = first.filter(|f| f.1 < index).or(Some((c, index)));
                        last = last.filter(|f| f.1 > index).or(Some((c, index)));
                    }
                    (first, last)
                });
        let first = first.context("expected a first value")?;
        let last = last.context("expected a last value")?;

        let num = ((first.0 as u32 - '0' as u32) * 10) + (last.0 as u32 - '0' as u32);
        p2 += num;
    }
    Ok(p2)
}

fn part_1(input: &str) -> anyhow::Result<u32> {
    let mut p1 = 0;
    for line in input.lines() {
        let (first, last) =
            line.chars()
                .enumerate()
                .fold((None, None), |(mut first, mut last), (index, c)| {
                    if c.is_ascii_digit() {
                        first = first.filter(|f: &(_, _)| f.1 < index).or(Some((c, index)));
                        last = last.filter(|f: &(_, _)| f.1 > index).or(Some((c, index)));
                    }
                    (first, last)
                });
        let first = first.context("expected a first value")?;
        let last = last.context("expected a last value")?;
        let num = ((first.0 as u32 - '0' as u32) * 10) + (last.0 as u32 - '0' as u32);
        p1 += num;
    }

    Ok(p1)
}

#[cfg(test)]
mod tests {
    use crate::{days::day01::solve, IntoDayResult};

    #[test]
    fn works_for_example() {
        const INPUT: &str = include_str!("../../input/day01_test.txt");
        let solution = solve(INPUT).unwrap();
        assert_eq!((209, 198).into_day_result(), solution);
    }

    #[test]
    fn works_for_input() {
        const INPUT: &str = include_str!("../../input/day01.txt");
        let solution = solve(INPUT).unwrap();
        assert_eq!((54_390, 54_277).into_day_result(), solution);
    }
}
