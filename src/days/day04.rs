use crate::{DayResult, IntoDayResult};

pub fn solve(input: &str, is_test: bool) -> anyhow::Result<DayResult> {
    let mut p1 = 0;

    let mut all_cards = vec![];

    let mut input = input.as_bytes();

    while !input.is_empty() {
        let mut bingo_set = BingoSet::default();

        input = &input[if is_test { 7 } else { 9 }..];

        while input[1] != b'|' {
            let tens = match input[1] {
                b' ' => 0,
                b => (b - b'0') as u64,
            };
            let digits = (input[2] - b'0') as u64;
            let n = tens * 10 + digits;
            bingo_set.set(n);
            input = &input[3..];
        }

        input = &input[2..];

        let mut matches = 0;
        while input[0] != b'\n' {
            let tens = match input[1] {
                b' ' => 0,
                b => (b - b'0') as u64,
            };
            let digits = (input[2] - b'0') as u64;
            let n = tens * 10 + digits;
            matches += bingo_set.is_set(n) as usize;
            input = &input[3..];
        }
        p1 += (1 << matches) >> 1;

        all_cards.push((1, matches));
        input = &input[1..];
    }

    let mut all_cards_slice = all_cards.as_mut_slice();
    while let [(count, matches), _all_cards_slice @ ..] = all_cards_slice {
        all_cards_slice = _all_cards_slice;
        for card in all_cards_slice.iter_mut().take(*matches) {
            card.0 += *count;
        }
    }

    let p2 = all_cards.into_iter().map(|(count, _)| count).sum::<i32>();

    (p1, p2).into_result()
}

#[derive(Default)]
struct BingoSet {
    set: u128,
}

impl BingoSet {
    fn set(&mut self, val: u64) {
        self.set |= 1_u128 << val
    }

    fn is_set(&self, val: u64) -> bool {
        self.set & (1_u128 << val) != 0
    }
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
