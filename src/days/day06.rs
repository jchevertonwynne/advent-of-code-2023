use arrayvec::ArrayVec;

use crate::{DayResult, IntoDayResult};

pub fn solve(input: &str) -> anyhow::Result<DayResult> {
    let input = input.as_bytes();
    let (input, times, bigger_time) = parse_numbers(input);
    let (_, distances, bigger_dist) = parse_numbers(input);

    let p1: u64 = times
        .iter()
        .zip(distances.iter())
        .map(|(&time, &dist)| race(time, dist))
        .product();

    let p2 = race(bigger_time, bigger_dist);

    (p1, p2).into_result()
}

fn race(time: u64, distance: u64) -> u64 {
    let mut start = 0;
    while (start + 1) * (time - (start + 1)) < distance {
        let mut incr = 1;
        loop {
            let curr_start = start + incr;

            if curr_start * (time - curr_start) > distance {
                break;
            }

            incr <<= 1;
        }

        start += incr >> 1;
    }

    time - (start + 1) * 2 + 1
}

fn parse_numbers(mut input: &[u8]) -> (&[u8], ArrayVec<u64, 4>, u64) {
    let mut res = ArrayVec::new();
    let mut big_num = 0;

    while input[0] != b'\n' {
        while !input[0].is_ascii_digit() {
            input = &input[1..];
        }

        let mut num = 0;

        while input[0].is_ascii_digit() {
            let digit = (input[0] - b'0') as u64;
            num = num * 10 + digit;
            big_num = big_num * 10 + digit;
            input = &input[1..];
        }

        res.push(num);
    }

    (&input[1..], res, big_num)
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
