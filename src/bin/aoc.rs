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
    ensure_in_aoc_repository()?;
    let pkg_name = get_pkg_name()?;
    write_runner_file(&pkg_name)?;
    update_mod_file(&pkg_name)?;
    write_solver_file(&pkg_name)?;
    write_test_files(&pkg_name)?;
    Ok(())
}

fn ensure_in_aoc_repository() -> Result<(), anyhow::Error> {
    let cwd = std::env::current_dir()?;
    if !cwd.ends_with("advent-of-code-2023") {
        anyhow::bail!("you are in the wrong directory: {cwd:?}");
    };

    Ok(())
}

fn get_pkg_name() -> Result<String, anyhow::Error> {
    let pkg_name = std::env::args()
        .nth(1)
        .context("expected a second argument")?;

    let (_, pkg_name) = parse_pkg_name(&pkg_name)
        .map_err(|err| anyhow::anyhow!("failed to parse package name: {err}"))?;

    Ok(pkg_name)
}

fn parse_pkg_name(input: &str) -> IResult<&str, String> {
    all_consuming(map(nom::character::complete::u32, |num| {
        format!("day{num:0>2}")
    }))(input)
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

fn write_runner_file(pkg_name: &str) -> Result<(), anyhow::Error> {
    File::options()
        .create_new(true)
        .write(true)
        .open(format!("src/bin/{pkg_name}.rs"))
        .context("runner file already exists")?
        .write_all(format!("advent_of_code_2023::aoc!({pkg_name});").as_bytes())?;

    Ok(())
}

fn update_mod_file(pkg_name: &str) -> Result<(), anyhow::Error> {
    let days = std::fs::read_to_string("src/days/mod.rs")?;
    let mods = days
        .lines()
        .map(|line| {
            parse_mod_line(line)
                .map(|(_, day)| day)
                .map_err(|err| anyhow::anyhow!("failed to parse module line: {err}"))
        })
        .chain(std::iter::once(Ok(pkg_name.to_string())))
        .collect::<Result<HashSet<_>, _>>()?;

    let mut output = String::new();
    for m in mods.into_iter().sorted() {
        writeln!(&mut output, "pub mod {m};")?;
    }

    std::fs::write("src/days/mod.rs", output.as_bytes())?;

    Ok(())
}

fn write_solver_file(pkg_name: &str) -> Result<(), anyhow::Error> {
    let solver = format!(
        r#"
use crate::{{DayResult, IntoDayResult}};

pub fn solve(_input: &str) -> anyhow::Result<DayResult> {{
    ().into_result()
}}

#[cfg(test)]
mod tests {{
    use crate::{{days::{pkg_name}::solve, IntoDayResult}};

    #[test]
    fn works_for_example() {{
        const INPUT: &str = include_str!("../../input/{pkg_name}_test.txt");
        let solution = solve(INPUT).unwrap();
        assert_eq!(
            ().into_day_result(),
            solution
        );
    }}

    #[test]
    fn works_for_input() {{
        const INPUT: &str = include_str!("../../input/{pkg_name}.txt");
        let solution = solve(INPUT).unwrap();
        assert_eq!(
            ().into_day_result(),
            solution
        );
    }}
}}"#
    );
    std::fs::write(format!("src/days/{pkg_name}.rs"), solver.as_bytes())?;

    Ok(())
}

fn write_test_files(pkg_name: &str) -> Result<(), anyhow::Error> {
    File::create(format!("input/{pkg_name}.txt"))?;
    File::create(format!("input/{pkg_name}_test.txt"))?;

    Ok(())
}
