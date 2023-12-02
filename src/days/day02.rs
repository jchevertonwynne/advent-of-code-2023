use anyhow::Context;
use nom::{
    branch::alt, bytes::complete::tag, character::complete::u32 as parse_u32, combinator::map,
    multi::separated_list1, sequence::tuple, IResult,
};

use crate::{DayResult, IntoDayResult};

pub fn solve(mut input: &str) -> anyhow::Result<DayResult> {
    let mut p1 = 0;
    let mut p2 = 0;

    while !input.is_empty() {
        let (_input, (id, blocks)) = parse_day(input).map_err(|err| anyhow::anyhow!("{err}"))?;
        input = _input;

        let few_enough = blocks.iter().all(|block| {
            block.iter().all(|&(count, colour)| match colour {
                Colour::Red => count <= 12,
                Colour::Green => count <= 13,
                Colour::Blue => count <= 14,
            })
        });

        if few_enough {
            p1 += id;
        }

        let colours = blocks.into_iter().fold(Colours::default(), |c, block| {
            block.into_iter().fold(c, |mut c, (count, colour)| {
                match colour {
                    Colour::Red => c.red = c.red.map(|r| std::cmp::max(r, count)).or(Some(count)),
                    Colour::Green => {
                        c.green = c.green.map(|r| std::cmp::max(r, count)).or(Some(count))
                    }
                    Colour::Blue => {
                        c.blue = c.blue.map(|r| std::cmp::max(r, count)).or(Some(count))
                    }
                };
                c
            })
        });

        let r = colours.red.context("expected some red")?;
        let g = colours.green.context("expected some green")?;
        let b = colours.blue.context("expected some blue")?;

        p2 += r * g * b;
    }

    (p1, p2).into_result()
}

#[derive(Debug, Default)]
struct Colours {
    red: Option<u32>,
    green: Option<u32>,
    blue: Option<u32>,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Colour {
    Red,
    Green,
    Blue,
}

type ColourCount = (u32, Colour);

fn parse_day(input: &str) -> IResult<&str, (u32, Vec<Vec<ColourCount>>)> {
    map(
        nom::sequence::tuple((
            tag("Game "),
            parse_u32,
            tag(": "),
            separated_list1(tag("; "), parse_block),
            tag("\n"),
        )),
        |(_, id, _, blocks, _)| (id, blocks),
    )(input)
}

fn parse_block(input: &str) -> IResult<&str, Vec<(u32, Colour)>> {
    separated_list1(tag(", "), parse_beads)(input)
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
        assert_eq!(().into_day_result(), solution);
    }

    #[test]
    fn works_for_input() {
        const INPUT: &str = include_str!("../../input/day02.txt");
        let solution = solve(INPUT).unwrap();
        assert_eq!(().into_day_result(), solution);
    }
}
