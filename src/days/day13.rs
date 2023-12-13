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

        for (mirrored_segments, i) in (1..height)
            .scan((false, false), |(found_width, found_width_m1), h| {
                if *found_width && *found_width_m1 {
                    return None;
                }

                let to_take = std::cmp::min(h, height - h);

                let tot = (0..width)
                    .map(|w| {
                        (h..height)
                            .map(|h| block[w + h * (width + 1)])
                            .take(to_take)
                            .eq((0..h)
                                .rev()
                                .map(|h| block[w + h * (width + 1)])
                                .take(to_take)) as usize
                    })
                    .sum::<usize>();

                if tot == width {
                    *found_width = true;
                } else if tot == width - 1 {
                    *found_width_m1 = true;
                }

                Some(tot)
            })
            .zip(1..)
        {
            if mirrored_segments == width {
                p1 += 100 * i;
            } else if mirrored_segments == width - 1 {
                p2 += 100 * i;
            }
        }

        for (mirrored_segments, i) in (1..width)
            .scan((false, false), |(found_height, found_height_m1), w| {
                if *found_height && *found_height_m1 {
                    return None;
                }

                let to_take = std::cmp::min(w, width - w);

                let tot = (0..height)
                    .map(|h| {
                        (w..width)
                            .map(|w| block[w + h * (width + 1)])
                            .take(to_take)
                            .eq((0..w)
                                .rev()
                                .map(|w| block[w + h * (width + 1)])
                                .take(to_take)) as usize
                    })
                    .sum::<usize>();

                if tot == height {
                    *found_height = true;
                } else if tot == height - 1 {
                    *found_height_m1 = true;
                }

                Some(tot)
            })
            .zip(1..)
        {
            if mirrored_segments == height {
                p1 += i;
            } else if mirrored_segments == height - 1 {
                p2 += i;
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
