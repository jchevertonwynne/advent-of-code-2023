use std::cmp::max;

use nom::character::complete::u32 as parse_u32;

use crate::{DayResult, IntoDayResult};

pub fn solve(mut input: &str) -> anyhow::Result<DayResult> {
    let mut p1 = 0;
    let mut p2 = 0;

    while !input.is_empty() {
        input = &input[5..];
        let (_input, id) = parse_u32::<_, nom::error::Error<&str>>(input)
            .map_err(|err| anyhow::anyhow!("{err}"))?;
        input = &_input[2..];

        let mut few_enough = true;
        let mut red = 0;
        let mut green = 0;
        let mut blue = 0;

        loop {
            let (_input, count) = parse_u32::<_, nom::error::Error<&str>>(input)
                .map_err(|err| anyhow::anyhow!("{err}"))?;
            input = _input;
            match input.as_bytes()[1] {
                b'r' => {
                    input = &input[4..];
                    few_enough &= count <= 12;
                    red = max(red, count);
                }
                b'g' => {
                    input = &input[6..];
                    few_enough &= count <= 13;
                    green = max(green, count);
                }
                b'b' => {
                    input = &input[5..];
                    few_enough &= count <= 14;
                    blue = max(blue, count);
                }
                _ => unreachable!(),
            }

            if input.as_bytes()[0] == b'\n' {
                input = &input[1..];
                break;
            } else {
                input = &input[2..];
            }
        }

        if few_enough {
            p1 += id;
        }

        p2 += red * green * blue;
    }

    (p1, p2).into_result()
}

#[cfg(test)]
mod tests {
    use crate::{days::day02::solve, IntoDayResult};

    #[test]
    fn works_for_example() {
        const INPUT: &str = include_str!("../../input/day02_test.txt");
        let solution = solve(INPUT).unwrap();
        assert_eq!((8, 2286).into_day_result(), solution);
    }

    #[test]
    fn works_for_input() {
        const INPUT: &str = include_str!("../../input/day02.txt");
        let solution = solve(INPUT).unwrap();
        assert_eq!((2679, 77607).into_day_result(), solution);
    }
}
