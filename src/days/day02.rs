
use crate::{DayResult, IntoDayResult};

pub fn solve(_input: &str) -> anyhow::Result<DayResult> {
    ().into_result()
}

#[cfg(test)]
mod tests {
    use crate::{days::day02::solve, Answers, DayResult};

    #[test]
    fn works_for_example() {
        const INPUT: &str = include_str!("../../input/day02_test.txt");
        let solution = solve(INPUT).unwrap();
        assert_eq!(
            DayResult {
                part1: None,
                part2: None,
            },
            solution
        );
    }
    #[test]
    fn works_for_input() {
        const INPUT: &str = include_str!("../../input/day02.txt");
        let solution = solve(INPUT).unwrap();
        assert_eq!(
            DayResult {
                part1: None,
                part2: None,
            },
            solution
        );
    }
}
