use anyhow::Context;
use itertools::Itertools;
use nom::{
    bytes::complete::tag,
    combinator::{all_consuming, map},
    sequence::{delimited, preceded},
    IResult,
};

use std::{collections::HashSet, fmt::Write as _, fs::File, io::Write as _};

fn main() -> anyhow::Result<()> {
    let cwd = std::env::current_dir()?;
    if !cwd.ends_with("advent-of-code-2023") {
        anyhow::bail!("you are in the wrong directory: {cwd:?}");
    }

    let pkg_name = std::env::args()
        .nth(1)
        .context("expected a second argument")?;

    let (_, pkg_name) = parse_pkg_name(&pkg_name)
        .map_err(|err| anyhow::anyhow!("failed to parse package name: {err}"))?;

    File::options()
        .create_new(true)
        .write(true)
        .open(format!("src/bin/{pkg_name}.rs"))
        .context("runner file already exists")?
        .write_all("advent_of_code_2023::aoc!(day01);".as_bytes())?;

    let days = std::fs::read_to_string("src/days/mod.rs")?;

    let mods = days
        .lines()
        .map(|line| {
            parse_mod_line(line)
                .map(|(_, day)| day)
                .map_err(|err| anyhow::anyhow!("failed to parse module line: {err}"))
        })
        .chain(Some(Ok(pkg_name.clone())).into_iter())
        .collect::<Result<HashSet<_>, _>>()?;

    let mut output = String::new();
    for m in mods.into_iter().sorted() {
        writeln!(&mut output, "pub mod {m};")?;
    }

    std::fs::write("src/days/mod.rs", output.as_bytes())?;

    let solver = format!(
        r#"
use crate::{{DayResult, IntoDayResult}};

pub fn solve(_input: &str) -> anyhow::Result<DayResult> {{
    ().into_result()
}}

#[cfg(test)]
mod tests {{
    use crate::{{days::{pkg_name}::solve, Answers, DayResult}};

    #[test]
    fn works_for_example() {{
        const INPUT: &str = include_str!("../../input/{pkg_name}_test.txt");
        let solution = solve(INPUT).unwrap();
        assert_eq!(
            DayResult {{
                part1: None,
                part2: None,
            }},
            solution
        );
    }}
    #[test]
    fn works_for_input() {{
        const INPUT: &str = include_str!("../../input/{pkg_name}.txt");
        let solution = solve(INPUT).unwrap();
        assert_eq!(
            DayResult {{
                part1: None,
                part2: None,
            }},
            solution
        );
    }}
}}"#
    );

    std::fs::write(format!("src/days/{pkg_name}.rs"), solver.as_bytes())?;

    File::create(format!("input/{pkg_name}.txt"))?;
    File::create(format!("input/{pkg_name}_test.txt"))?;

    Ok(())
}

fn parse_pkg_name(input: &str) -> IResult<&str, String> {
    all_consuming(map(
        preceded(tag("day"), nom::character::complete::u32),
        |num| format!("day{num:0>2}"),
    ))(input)
}

fn parse_mod_line(input: &str) -> IResult<&str, String> {
    all_consuming(delimited(
        tag("pub mod "),
        map(preceded(tag("day"), nom::character::complete::u32), |num| {
            format!("day{num:0>2}")
        }),
        tag(";"),
    ))(input)
}
