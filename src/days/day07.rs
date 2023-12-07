use std::{
    cmp::{Ord, Ordering},
    fmt::Debug,
};

use crate::{DayResult, IntoDayResult};

pub fn solve(input: &str) -> anyhow::Result<DayResult> {
    let mut hands = vec![];

    let mut input = input.as_bytes();
    while !input.is_empty() {
        let (_input, hand, bet) = parse_hand(input);
        input = _input;
        hands.push((hand, bet));
    }

    hands.sort_unstable_by(|(a, _), (b, _)| Ord::cmp(&a, &b));
    let p1: usize = hands.iter().zip(1..).map(|(&(_, b), i)| b * i).sum();

    hands.sort_unstable_by(|&(a, _), &(b, _)| {
        let a: HandJoker = a.into();
        let b: HandJoker = b.into();
        Ord::cmp(&a, &b)
    });
    let p2: usize = hands.iter().zip(1..).map(|(&(_, b), i)| b * i).sum();

    (p1, p2).into_result()
}

fn parse_hand(input: &[u8]) -> (&[u8], Hand, usize) {
    let mut hand = Hand {
        raw_cards: [0; 5],
        card_powers: [0; 5],
        hand_type: 0,
    };
    let mut cards = [0; 13];
    for (i, &c) in input[0..5].iter().enumerate() {
        hand.raw_cards[i] = c;
        let power = match c {
            b'2'..=b'9' => (c - b'2') as usize,
            b'A' => 12,
            b'K' => 11,
            b'Q' => 10,
            b'J' => 9,
            b'T' => 8,
            _ => unreachable!("lmao"),
        };
        hand.card_powers[i] = power as u8;
        cards[power] += 1;
    }

    let mut input = &input[6..];
    let mut num = 0;
    while input[0] != b'\n' {
        num = num * 10 + (input[0] - b'0') as usize;
        input = &input[1..];
    }

    hand.hand_type = hand.hand_type(&cards);

    (&input[1..], hand, num)
}

#[derive(Clone, Copy, PartialEq, Eq)]
struct Hand {
    raw_cards: [u8; 5],
    card_powers: [u8; 5],
    hand_type: u8,
}

impl Debug for Hand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for c in self.raw_cards {
            write!(f, "{}", c as char)?;
        }
        Ok(())
    }
}

impl Hand {
    fn collate(&self, cards: &[u8; 13]) -> [u8; 6] {
        let mut counts = [0; 6];
        for &c in cards {
            counts[c as usize] += 1;
        }

        counts
    }

    fn hand_type(self, cards: &[u8; 13]) -> u8 {
        let counts = self.collate(cards);

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
            &(self.hand_type, self.card_powers),
            &(other.hand_type, other.card_powers),
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

impl From<&'_ [u8]> for HandJoker {
    fn from(value: &'_ [u8]) -> Self {
        let mut hand = HandJoker {
            card_powers: [0; 5],
            hand_type: 0,
        };
        let mut cards = [0; 13];
        for (i, &c) in value.iter().enumerate() {
            let power = match c {
                b'2'..=b'9' => (c - b'1') as usize,
                b'A' => 12,
                b'K' => 11,
                b'Q' => 10,
                b'J' => 0,
                b'T' => 9,
                _ => unreachable!("lmao"),
            };
            hand.card_powers[i] = power as u8;
            cards[power] += 1;
        }

        hand.hand_type = hand.hand_type(&mut cards);

        hand
    }
}

impl From<Hand> for HandJoker {
    fn from(value: Hand) -> Self {
        let Hand { raw_cards, .. } = value;
        HandJoker::from(raw_cards.as_slice())
    }
}

impl HandJoker {
    fn collate(self, cards: &mut [u8; 13]) -> [u8; 6] {
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

    fn hand_type(self, cards: &mut [u8; 13]) -> u8 {
        let counts = self.collate(cards);

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

impl Ord for HandJoker {
    fn cmp(&self, other: &Self) -> Ordering {
        Ord::cmp(
            &(self.hand_type, self.card_powers),
            &(other.hand_type, other.card_powers),
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
