use std::num::ParseIntError;

use arrayvec::ArrayVec;

use crate::{DayResult, IntoDayResult};

pub fn solve(input: &str) -> anyhow::Result<DayResult> {
    let input = input.as_bytes();
    let (input, times) = parse_numbers(input)?;
    let (_, distances) = parse_numbers(input)?;

    let p1: usize = times
        .iter()
        .zip(distances.iter())
        .map(|(&t, &d)| race(t, d))
        .product();

    let bigger_time = combine_numbers(&times);
    let bigger_dist = combine_numbers(&distances);

    let p2 = race(bigger_time, bigger_dist);

    (p1, p2).into_result()
}

fn combine_numbers(nums: &[u64]) -> u64 {
    nums.iter().fold(0, |mut acc, &n| {
        let mut t2 = n;
        while t2 > 0 {
            acc *= 10;
            t2 /= 10;
        }
        acc + n
    })
}

fn race(time: u64, distance: u64) -> usize {
    (0..time + 1).filter(|t| t * (time - t) > distance).count()
}

fn parse_numbers(mut input: &[u8]) -> Result<(&[u8], ArrayVec<u64, 4>), ParseIntError> {
    let mut res = ArrayVec::new();
    while input[0] != b'\n' {
        while !input[0].is_ascii_digit() {
            input = &input[1..];
        }
        let mut num = 0;
        while input[0].is_ascii_digit() {
            num = num * 10 + (input[0] - b'0') as u64;
            input = &input[1..];
        }
        res.push(num);
    }
    Ok((&input[1..], res))
}

#[cfg(test)]
mod tests {
    use crate::{days::day06::solve, IntoDayResult};

    #[test]
    fn works_for_example() {
        const INPUT: &str = include_str!("../../input/day06_test.txt");
        let solution = solve(INPUT).unwrap();
        assert_eq!((288, 71503).into_day_result(), solution);
    }

    #[test]
    fn works_for_input() {
        const INPUT: &str = include_str!("../../input/day06.txt");
        let solution = solve(INPUT).unwrap();
        assert_eq!((861_300, 28_101_347).into_day_result(), solution);
    }
}
