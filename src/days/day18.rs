use bstr::ByteSlice;
use itertools::Itertools;
use nom::{
    branch::alt,
    bytes::complete::tag,
    combinator::{map, map_opt},
    IResult,
};

use crate::{DayResult, IntoDayResult};

pub fn solve(input: &str) -> anyhow::Result<DayResult> {
    let input = input.as_bytes();

    let mut x1: isize = 0;
    let mut y1: isize = 0;
    let mut x2: isize = 0;
    let mut y2: isize = 0;

    let mut points1 = vec![(x1, y1)];
    let mut points2 = vec![(x1, y1)];
    let mut boundary_points1 = 0;
    let mut boundary_points2 = 0;

    for line in input.lines() {
        let (_, ((direction1, distance1), (direction2, distance2))) =
            parse_line(line).map_err(|err| anyhow::anyhow!("{err}"))?;

        let distance1 = distance1 as isize;
        let distance2 = distance2 as isize;

        (x1, y1) = match direction1 {
            Direction::Up => (x1, y1 + distance1),
            Direction::Down => (x1, y1 - distance1),
            Direction::Left => (x1 - distance1, y1),
            Direction::Right => (x1 + distance1, y1),
        };

        (x2, y2) = match direction2 {
            Direction::Up => (x2, y2 + distance2),
            Direction::Down => (x2, y2 - distance2),
            Direction::Left => (x2 - distance2, y2),
            Direction::Right => (x2 + distance2, y2),
        };

        boundary_points1 += distance1;
        boundary_points2 += distance2;

        points1.push((x1, y1));
        points2.push((x2, y2));
    }

    let area1 = area(&points1);
    let area2 = area(&points2);

    let interior1 = area1 + 1 - (boundary_points1 / 2);
    let interior2 = area2 + 1 - (boundary_points2 / 2);

    let p1 = interior1 + boundary_points1;
    let p2 = interior2 + boundary_points2;

    (p1, p2).into_result()
}

fn area(points: &[(isize, isize)]) -> isize {
    points
        .iter()
        .tuple_windows()
        .map(|(&(x1, y1), &(x2, y2))| (y1 + y2) * (x1 - x2))
        .sum::<isize>()
        .abs()
        / 2
}

fn parse_direction(line: &[u8]) -> IResult<&[u8], Direction> {
    alt((
        map(tag("U"), |_| Direction::Up),
        map(tag("D"), |_| Direction::Down),
        map(tag("L"), |_| Direction::Left),
        map(tag("R"), |_| Direction::Right),
    ))(line)
}

fn parse_direction2(d: u32) -> Option<Direction> {
    Some(match d {
        0 => Direction::Right,
        1 => Direction::Down,
        2 => Direction::Left,
        3 => Direction::Up,
        _ => return None,
    })
}

fn parse_distance_direction(line: &[u8]) -> IResult<&[u8], (Direction, u32)> {
    map_opt(nom::number::complete::hex_u32, |n| {
        let direction = n & 0b1111;
        parse_direction2(direction).map(|d| (d, n >> 4))
    })(line)
}

type DDTuple = (Direction, u32);

fn parse_line(line: &[u8]) -> IResult<&[u8], (DDTuple, DDTuple)> {
    map(
        nom::sequence::tuple((
            parse_direction,
            tag(" "),
            nom::character::complete::u32,
            tag(" (#"),
            parse_distance_direction,
            tag(")"),
        )),
        |(direction, _, distance, _, hex, _)| ((direction, distance), hex),
    )(line)
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[cfg(test)]
mod tests {
    use crate::{days::day18::solve, IntoDayResult};

    #[test]
    fn works_for_example() {
        const INPUT: &str = include_str!("../../input/day18_test.txt");
        let solution = solve(INPUT).unwrap();
        assert_eq!((62, 952_408_144_115_isize).into_day_result(), solution);
    }

    #[test]
    fn works_for_input() {
        const INPUT: &str = include_str!("../../input/day18.txt");
        let solution = solve(INPUT).unwrap();
        assert_eq!(
            (33_491, 87_716_969_654_406_isize).into_day_result(),
            solution
        );
    }
}
