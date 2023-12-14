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

        let mut done_p1 = false;
        let mut done_p2 = false;
        for h in 1..height {
            let tot = (0..width)
                .map(|w| {
                    let a = (h..height).map(|h| block[w + h * (width + 1)]);
                    let b = (0..h).rev().map(|h| block[w + h * (width + 1)]);
                    (a.zip(b).map(|(a, b)| (a != b) as usize).sum::<usize>() == 0) as usize
                })
                .sum::<usize>();

            if tot == width {
                done_p1 = true;
                p1 += 100 * h;
            } else if tot == width - 1 {
                done_p2 = true;
                p2 += 100 * h;
            }

            if done_p1 && done_p2 {
                break;
            }
        }

        for w in 1..width {
            let tot = (0..height)
                .map(|h| {
                    let a = (w..width).map(|w| block[w + h * (width + 1)]);
                    let b = (0..w).rev().map(|w| block[w + h * (width + 1)]);
                    (a.zip(b).map(|(a, b)| (a != b) as usize).sum::<usize>() == 0) as usize
                })
                .sum::<usize>();

            if tot == height {
                done_p1 = true;
                p1 += w;
            } else if tot == height - 1 {
                done_p2 = true;
                p2 += w;
            }

            if done_p1 && done_p2 {
                break;
            }
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
