use std::cmp::{Ord, Ordering};

use crate::{DayResult, IntoDayResult};

pub fn solve(input: &str) -> anyhow::Result<DayResult> {
    let mut hands = vec![];

    let mut input = input.as_bytes();
    while !input.is_empty() {
        let (_input, hand, cards, bet) = parse_hand(input);
        input = _input;
        let hand_joker = HandJoker::from((hand.card_powers, cards));
        hands.push((hand, hand_joker, bet));
    }

    hands.sort_unstable_by(|(a, _, _), (b, _, _)| Ord::cmp(&a, &b));
    let p1: usize = hands.iter().zip(1..).map(|(&(_, _, b), i)| b * i).sum();

    hands.sort_unstable_by(|&(_, a, _), &(_, b, _)| Ord::cmp(&a, &b));
    let p2: usize = hands.iter().zip(1..).map(|(&(_, _, b), i)| b * i).sum();

    (p1, p2).into_result()
}

const fn table() -> [u8; 64] {
    let mut table = [0; 64];
    const MAPPING: [u8; 13] = [
        b'2', b'3', b'4', b'5', b'6', b'7', b'8', b'9', b'T', b'J', b'Q', b'K', b'A',
    ];

    let mut i = 0;
    while i < MAPPING.len() {
        table[(MAPPING[i] - b'0') as usize] = i as u8;
        i += 1;
    }

    table
}

fn parse_hand(input: &[u8]) -> (&[u8], Hand, [u8; 13], usize) {
    const LOOKUP: [u8; 64] = table();
    let mut raw_cards = [0; 5];
    let mut cards = [0; 13];
    let mut card_powers = [0; 5];
    for (i, &c) in input[0..5].iter().enumerate() {
        raw_cards[i] = c;
        let power = LOOKUP[(c - b'0') as usize];
        card_powers[i] = power;
        cards[power as usize] += 1;
    }

    let mut input = &input[6..];
    let mut num = 0;
    while input[0] != b'\n' {
        num = num * 10 + (input[0] - b'0') as usize;
        input = &input[1..];
    }

    let hand = Hand {
        card_powers,
        hand_type: Hand::hand_type(&cards),
    };

    (&input[1..], hand, cards, num)
}

#[derive(Clone, Copy, PartialEq, Eq)]
struct Hand {
    card_powers: [u8; 5],
    hand_type: u8,
}

impl Hand {
    fn collate(cards: &[u8; 13]) -> [u8; 6] {
        let mut counts = [0; 6];
        for &c in cards {
            counts[c as usize] += 1;
        }

        counts
    }

    fn hand_type(cards: &[u8; 13]) -> u8 {
        let counts = Hand::collate(cards);

        if counts[5] == 1 {
            return 7;
        }

        if counts[4] == 1 {
            return 6;
        }

        if counts[3] == 1 && counts[2] == 1 {
            return 5;
        }

        if counts[3] == 1 && counts[1] == 2 {
            return 4;
        }

        if counts[2] == 2 && counts[1] == 1 {
            return 3;
        }

        if counts[2] == 1 && counts[1] == 3 {
            return 2;
        }

        1
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        Ord::cmp(
            &(self.hand_type, &self.card_powers),
            &(other.hand_type, &other.card_powers),
        )
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
struct HandJoker {
    card_powers: [u8; 5],
    hand_type: u8,
}

impl std::fmt::Debug for HandJoker {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let chars = b"J23456789TQKA";

        for p in self.card_powers {
            write!(f, "{}", chars[p as usize] as char)?;
        }

        Ok(())
    }
}

impl From<([u8; 5], [u8; 13])> for HandJoker {
    fn from((card_powers, mut cards): ([u8; 5], [u8; 13])) -> Self {
        cards[..10].rotate_right(1);

        let card_powers = card_powers.map(|v| match v {
            9 => 0,
            v if v < 9 => v + 1,
            _ => v,
        });

        let hand_type = HandJoker::hand_type(&mut cards);

        HandJoker {
            card_powers,
            hand_type,
        }
    }
}

impl HandJoker {
    fn collate(cards: &mut [u8; 13]) -> [u8; 6] {
        let jokers = std::mem::take(&mut cards[0]);

        let mut counts = [0; 6];
        for &mut c in cards {
            counts[c as usize] += 1;
        }

        if jokers != 0 {
            let mut update = 0;
            for (i, &count) in counts.iter().enumerate().rev() {
                if count != 0 {
                    update = i;
                    break;
                }
            }

            counts[update] -= 1;
            counts[update + jokers as usize] += 1;
        }

        counts
    }

    fn hand_type(cards: &mut [u8; 13]) -> u8 {
        let counts = HandJoker::collate(cards);

        if counts[5] == 1 {
            return 7;
        }

        if counts[4] == 1 {
            return 6;
        }

        if counts[3] == 1 {
            return 4 | (counts[2] == 1) as u8;

            // return 5;
        }

        // if counts[3] == 1 && counts[1] == 2 {
        //     return 4;
        // }

        if counts[2] == 2 && counts[1] == 1 {
            return 3;
        }

        if counts[2] == 1 && counts[1] == 3 {
            return 2;
        }

        1
    }
}

impl Ord for HandJoker {
    fn cmp(&self, other: &Self) -> Ordering {
        Ord::cmp(
            &(self.hand_type, &self.card_powers),
            &(other.hand_type, &other.card_powers),
        )
    }
}

impl PartialOrd for HandJoker {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[cfg(test)]
mod tests {
    use crate::{days::day07::solve, IntoDayResult};

    #[test]
    fn works_for_example() {
        const INPUT: &str = include_str!("../../input/day07_test.txt");
        let solution = solve(INPUT).unwrap();
        assert_eq!((6_440, 5_905).into_day_result(), solution);
    }

    #[test]
    fn works_for_input() {
        const INPUT: &str = include_str!("../../input/day07.txt");
        let solution = solve(INPUT).unwrap();
        assert_eq!((251_136_060, 249_400_220).into_day_result(), solution);
    }
}
