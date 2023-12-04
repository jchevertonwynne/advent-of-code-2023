use bstr::ByteSlice;

use crate::{DayResult, IntoDayResult};

pub fn solve(input: &str, is_test: bool) -> anyhow::Result<DayResult> {
    let mut p1 = 0;

    let mut all_cards = vec![];
    let mut winners = vec![];

    for line in input.as_bytes().lines() {
        winners.clear();

        let mut line = &line[if is_test { 8 } else { 10 }..];

        while line[1] != b'|' {
            while line[0] == b' ' {
                line = &line[1..];
            }
            let (_line, n) = nom::character::complete::u32::<_, nom::error::Error<&[u8]>>(line)
                .map_err(|err| anyhow::anyhow!("{err}"))?;
            line = _line;
            winners.push(n);
        }

        line = &line[3..];

        let mut matches = 0;
        while !line.is_empty() {
            while line[0] == b' ' {
                line = &line[1..];
            }
            let (_line, n) = nom::character::complete::u32::<_, nom::error::Error<&[u8]>>(line)
                .map_err(|err| anyhow::anyhow!("{err}"))?;
            line = _line;
            matches += winners.contains(&n) as i32;
        }
        p1 += (1 << matches) >> 1;

        all_cards.push((1, matches));
    }

    let mut all_cards_slice = all_cards.as_mut_slice();
    while let [(count, matches), _all_cards_slice @ ..] = all_cards_slice {
        all_cards_slice = _all_cards_slice;
        for i in 0..*matches {
            all_cards_slice[i as usize].0 += *count;
        }
    }

    let p2 = all_cards.into_iter().map(|(count, _)| count).sum::<i32>();

    (p1, p2).into_result()
}

#[cfg(test)]
mod tests {
    use crate::{days::day04::solve, IntoDayResult};

    #[test]
    fn works_for_example() {
        const INPUT: &str = include_str!("../../input/day04_test.txt");
        let solution = solve(INPUT, true).unwrap();
        assert_eq!((13, 30).into_day_result(), solution);
    }

    #[test]
    fn works_for_input() {
        const INPUT: &str = include_str!("../../input/day04.txt");
        let solution = solve(INPUT, false).unwrap();
        assert_eq!((32_609, 14_624_680).into_day_result(), solution);
    }
}
