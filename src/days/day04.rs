use std::collections::HashSet;

use fxhash::FxHashMap;
use nom::{
    bytes::complete::tag,
    combinator::map,
    multi::separated_list1,
    sequence::{delimited, preceded, tuple},
    IResult,
};

use crate::{DayResult, IntoDayResult};

pub fn solve(_input: &str) -> anyhow::Result<DayResult> {
    let mut p1 = 0;

    let mut all_cards = FxHashMap::default();

    for line in _input.lines() {
        let (_, (card, winners, got)) = parse_line(line).map_err(|err| anyhow::anyhow!("{err}"))?;

        let matches = winners.intersection(&got).count();
        if matches != 0 {
            p1 += 1 << (matches - 1);
        }

        all_cards.insert(card, matches);
    }

    let mut p2 = 0;
    let mut stack = all_cards
        .iter()
        .map(|(&v, &v2)| ((1, v), v2))
        .collect::<Vec<_>>();
    while let Some(((count, card), matches)) = stack.pop() {
        p2 += count;
        for i in 0..matches {
            let new_card = card + i as u32 + 1;
            stack.push(((count, new_card), all_cards[&new_card]));
        }
    }

    (p1, p2).into_result()
}

fn parse_line(input: &str) -> IResult<&str, (u32, HashSet<u32>, HashSet<u32>)> {
    map(
        tuple((
            tag("Card"),
            nom::multi::many1(tag(" ")),
            nom::character::complete::u32,
            tag(":"),
            delimited(
                tag(" "),
                separated_list1(
                    tag(" "),
                    nom::sequence::pair(
                        nom::combinator::opt(tag(" ")),
                        nom::character::complete::u32,
                    ),
                ),
                tag(" "),
            ),
            tag("|"),
            preceded(
                tag(" "),
                separated_list1(
                    tag(" "),
                    nom::sequence::pair(
                        nom::combinator::opt(tag(" ")),
                        nom::character::complete::u32,
                    ),
                ),
            ),
        )),
        |(_, _, card, _, winners, _, my_cards)| {
            (
                card,
                winners.into_iter().map(|v| v.1).collect(),
                my_cards.into_iter().map(|v| v.1).collect(),
            )
        },
    )(input)
}

#[cfg(test)]
mod tests {
    use crate::{days::day04::solve, IntoDayResult};

    #[test]
    fn works_for_example() {
        const INPUT: &str = include_str!("../../input/day04_test.txt");
        let solution = solve(INPUT).unwrap();
        assert_eq!((13, 30).into_day_result(), solution);
    }

    #[test]
    fn works_for_input() {
        const INPUT: &str = include_str!("../../input/day04.txt");
        let solution = solve(INPUT).unwrap();
        assert_eq!((32_609, 14_624_680).into_day_result(), solution);
    }
}
