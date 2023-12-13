use anyhow::Context;

use crate::{DayResult, IntoDayResult};

pub fn solve(input: &str) -> anyhow::Result<DayResult> {
    let mut p1 = 0;
    let mut p2 = 0;

    for block in input.split("\n\n") {
        let block = block.as_bytes();
        let width = block
            .iter()
            .position(|&b| b == b'\n')
            .context("there is a newline")?;
        let height = (block.len() + 1) / (width + 1);

        let mut p1_match = false;
        let mut p2_match = false;

        let vertical_match = (1..height).find(|&h| {
            let left = h;
            let right = height - h;
            let m = std::cmp::min(left, right);

            (0..width).all(|w| {
                (h..height)
                    .map(|h| block[w + h * (width + 1)])
                    .take(m)
                    .eq((0..h).rev().map(|h| block[w + h * (width + 1)]).take(m))
            })
        });

        if let Some(vertical_split) = vertical_match {
            p1 += 100 * vertical_split;
            p1_match = true;
        }

        let vertical_match_2 = (1..height).find(|&h| {
            let left = h;
            let right = height - h;
            let m = std::cmp::min(left, right);

            (0..width)
                .map(|w| {
                    (h..height)
                        .map(|h| block[w + h * (width + 1)])
                        .take(m)
                        .eq((0..h).rev().map(|h| block[w + h * (width + 1)]).take(m))
                        as usize
                })
                .sum::<usize>()
                == width - 1
        });

        if let Some(vertical_split) = vertical_match_2 {
            p2 += 100 * vertical_split;
            p2_match = true;
        }

        if !p1_match {
            let horizontal_match = (1..width).find(|&w| {
                let left = w;
                let right = width - w;
                let m = std::cmp::min(left, right);

                (0..height).all(|h| {
                    (w..width)
                        .map(|w| block[w + h * (width + 1)])
                        .take(m)
                        .eq((0..w).rev().map(|w| block[w + h * (width + 1)]).take(m))
                })
            });

            p1 += horizontal_match.context("this should always match")?;
        }

        if !p2_match {
            let horizontal_match_2 = (1..width).find(|&w| {
                let left = w;
                let right = width - w;
                let m = std::cmp::min(left, right);

                (0..height)
                    .map(|h| {
                        (w..width)
                            .map(|w| block[w + h * (width + 1)])
                            .take(m)
                            .eq((0..w).rev().map(|w| block[w + h * (width + 1)]).take(m))
                            as usize
                    })
                    .sum::<usize>()
                    == height - 1
            });

            p2 += horizontal_match_2.context("this should always match")?;
        }
    }

    (p1, p2).into_result()
}

#[cfg(test)]
mod tests {
    use crate::{days::day13::solve, IntoDayResult};

    #[test]
    fn works_for_example() {
        const INPUT: &str = include_str!("../../input/day13_test.txt");
        let solution = solve(INPUT).unwrap();
        assert_eq!((405, 400).into_day_result(), solution);
    }

    #[test]
    fn works_for_input() {
        const INPUT: &str = include_str!("../../input/day13.txt");
        let solution = solve(INPUT).unwrap();
        assert_eq!((30_518, 36_735).into_day_result(), solution);
    }
}
