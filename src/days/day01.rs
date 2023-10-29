use itertools::Itertools;

use crate::{CollectN, DayResult, IntoDayResult};

pub fn solve(input: &str) -> anyhow::Result<DayResult> {
    let top_three = input
        .lines()
        .map(|line| line.parse::<usize>().ok())
        .batching(|it| it.map_while(|num| num).sum1::<usize>())
        .collect_largest::<3>();

    let part1 = top_three[0];
    let part2 = top_three.iter().sum::<usize>();

    (part1, part2).into_result()
}

#[cfg(test)]
mod tests {
    use crate::{days::day01::solve, Answers, DayResult};

    #[test]
    fn works_for_example() {
        const INPUT: &str = include_str!("../../input/day01_test.txt");
        let solution = solve(INPUT).unwrap();
        assert_eq!(
            DayResult {
                part1: Some(Answers::Usize(24000)),
                part2: Some(Answers::Usize(45000))
            },
            solution
        );
    }
    #[test]
    fn works_for_input() {
        const INPUT: &str = include_str!("../../input/day01.txt");
        let solution = solve(INPUT).unwrap();
        assert_eq!(
            DayResult {
                part1: Some(Answers::Usize(69836)),
                part2: Some(Answers::Usize(207968))
            },
            solution
        );
    }
}
