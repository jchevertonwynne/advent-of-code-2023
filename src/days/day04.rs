use bstr::ByteSlice;
use fxhash::{FxHashMap, FxHashSet};

use crate::{DayResult, IntoDayResult};

pub fn solve(input: &str, is_test: bool) -> anyhow::Result<DayResult> {
    let mut p1 = 0;

    let mut all_cards = vec![];

    let mut winners = FxHashSet::default();

    for (line, card_id) in input.as_bytes().lines().zip(1..) {
        winners.clear();

        let mut line = &line[if is_test { 8 } else { 10 }..];

        while line[1] != b'|' {
            while line[0] == b' ' {
                line = &line[1..];
            }
            let (_line, n) = nom::character::complete::u32::<_, nom::error::Error<&[u8]>>(line)
                .map_err(|err| anyhow::anyhow!("{err}"))?;
            line = _line;
            winners.insert(n);
        }

        line = &line[2..];

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
        if matches != 0 {
            p1 += 1 << (matches - 1);
        }

        all_cards.push((card_id, matches));
    }

    let mut to_do = FxHashMap::default();

    let mut p2 = 0;
    for &(card_id, matches) in all_cards.iter() {
        to_do.insert(card_id, (1, matches));
    }
    let mut to_do_swap = FxHashMap::with_capacity_and_hasher(to_do.len(), Default::default());
    while !to_do.is_empty() {
        for (card_id, (count, matches)) in to_do.drain() {
            p2 += count;

            for i in 0..matches {
                let new_card = card_id + i as u32 + 1;
                let new_matches = all_cards[new_card as usize - 1].1;
                to_do_swap.entry(new_card).or_insert((0, new_matches)).0 += count;
            }
        }
        std::mem::swap(&mut to_do, &mut to_do_swap);
    }

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
