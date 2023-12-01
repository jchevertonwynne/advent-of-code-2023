use anyhow::Context;
use bstr::ByteSlice;
use nom::FindSubstring;

use crate::{DayResult, IntoDayResult};

pub fn solve(input: &str) -> anyhow::Result<DayResult> {
    let mut p1 = 0;
    let mut p2 = 0;

    for line in input.as_bytes().lines() {
        let (first, last) =
            line.bytes()
                .enumerate()
                .fold((None, None), |(mut first, mut last), (index, c)| {
                    if c.is_ascii_digit() {
                        let number = c as u32 - '0' as u32;
                        first = first.or(Some((number, index)));
                        last = Some((number, index));
                    }
                    (first, last)
                });

        let first1 = first.context("expected a first value")?;
        let last1 = last.context("expected a last value")?;

        let num1 = (first1.0 * 10) + last1.0;
        p1 += num1;

        const NUMS: [&str; 9] = [
            "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
        ];
        let first_indices = NUMS.map(|n| line.as_bytes().find_substring(n));
        let last_indices: [_; 9] = std::array::from_fn(|i| {
            first_indices[i].and_then(|_| {
                last.map(|(_, i)| i + 3 < line.len())
                    .unwrap_or(true)
                    .then(|| line.as_bytes().rfind(NUMS[i]))
                    .flatten()
            })
        });

        let (first, last) =
            itertools::izip!(first_indices.into_iter(), last_indices.into_iter(), 1..).fold(
                (first, last),
                |(mut first, mut last), (first_match_index, last_match_index, number)| {
                    if let Some(index) = first_match_index {
                        first = first
                            .filter(|f: &(_, _)| f.1 < index)
                            .or(Some((number, index)));
                    }
                    if let Some(index) = last_match_index {
                        last = last
                            .filter(|f: &(_, _)| f.1 > index)
                            .or(Some((number, index)));
                    }
                    (first, last)
                },
            );

        let first2 = first.context("expected a first value")?;
        let last2 = last.context("expected a last value")?;

        let num2 = (first2.0 * 10) + last2.0;
        p2 += num2;
    }

    (p1, p2).into_result()
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
