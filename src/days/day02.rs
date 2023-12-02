use std::cmp::max;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::u32 as parse_u32,
    combinator::map,
    sequence::{delimited, tuple},
    IResult,
};

use crate::{DayResult, IntoDayResult};

use Colour::*;

pub fn solve(mut input: &str) -> anyhow::Result<DayResult> {
    let mut p1 = 0;
    let mut p2 = 0;

    while !input.is_empty() {
        let (_input, id) = parse_id(input).map_err(|err| anyhow::anyhow!("{err}"))?;
        input = _input;

        let mut few_enough = true;
        let mut colours = Colours::default();

        loop {
            let (_input, (count, colour)) =
                parse_beads(input).map_err(|err| anyhow::anyhow!("{err}"))?;
            input = _input;

            match colour {
                Red => {
                    few_enough &= count <= 12;
                    colours.red = max(colours.red, count);
                }
                Green => {
                    few_enough &= count <= 13;
                    colours.green = max(colours.green, count);
                }
                Blue => {
                    few_enough &= count <= 14;
                    colours.blue = max(colours.blue, count);
                }
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

        p2 += colours.power();
    }

    (p1, p2).into_result()
}

#[derive(Debug, Default)]
struct Colours {
    red: u32,
    green: u32,
    blue: u32,
}

impl Colours {
    fn power(self) -> u32 {
        let Colours { red, green, blue } = self;
        red * green * blue
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Colour {
    Red,
    Green,
    Blue,
}

fn parse_id(input: &str) -> IResult<&str, u32> {
    delimited(tag("Game "), parse_u32, tag(": "))(input)
}

fn parse_beads(input: &str) -> IResult<&str, (u32, Colour)> {
    map(
        tuple((parse_u32, nom::bytes::complete::tag(" "), parse_colour)),
        |(count, _, colour)| (count, colour),
    )(input)
}

fn parse_colour(input: &str) -> IResult<&str, Colour> {
    alt((
        map(tag("red"), |_| Colour::Red),
        map(tag("green"), |_| Colour::Green),
        map(tag("blue"), |_| Colour::Blue),
    ))(input)
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
