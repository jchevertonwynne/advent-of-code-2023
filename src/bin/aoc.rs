use anyhow::{anyhow, bail, Context};
use chrono::{Datelike, Utc};
use clap::Parser;
use nom::{
    bytes::complete::tag,
    combinator::{all_consuming, map},
    sequence::{delimited, preceded},
    IResult,
};
use reqwest::blocking::ClientBuilder;
use tracing::info;

use std::{
    collections::BTreeSet,
    fmt::{Display, Formatter},
    fs::File,
    io::{ErrorKind, Write},
};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(default_value_t = Utc::now().day() as u8)]
    day: u8,
    #[arg(short, long, default_value_t = 2023)]
    year: usize,
    #[arg(short, long, default_value_t = false)]
    overwrite: bool,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    setup_tracing()?;
    ensure_in_aoc_repository()?;
    let pkg_name = PackageName(args.day);
    write_runner_file(pkg_name, args.overwrite).context("coult not write runner")?;
    update_mod_file(pkg_name).context("could not update mod file")?;
    write_solver_file(pkg_name).context("could not write solver file")?;
    write_input_files(pkg_name, args.year).context("could not write input files")?;
    Ok(())
}

fn setup_tracing() -> Result<(), anyhow::Error> {
    tracing_subscriber::fmt()
        .try_init()
        .map_err(|err| anyhow!("failed to setup tracing: {}", err))?;

    Ok(())
}

fn ensure_in_aoc_repository() -> Result<(), anyhow::Error> {
    let expected_dir = "advent-of-code-2023";
    let cwd = std::env::current_dir().context("failed to find current dir")?;
    if !cwd.ends_with(expected_dir) {
        bail!("not in {expected_dir}: {cwd:?}");
    };

    Ok(())
}

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
struct PackageName(u8);

impl Display for PackageName {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "day{:0>2}", self.0)
    }
}

fn parse_mod_line(input: &str) -> IResult<&str, PackageName> {
    all_consuming(delimited(
        tag("pub mod "),
        map(
            preceded(tag("day"), nom::character::complete::u8),
            PackageName,
        ),
        tag(";"),
    ))(input)
}

fn write_runner_file(pkg_name: PackageName, overwrite: bool) -> Result<(), anyhow::Error> {
    File::options()
        .create_new(!overwrite)
        .write(true)
        .open(format!("src/bin/{pkg_name}.rs"))
        .context("runner file already exists")?
        .write_all(format!("advent_of_code_2023::aoc!({pkg_name});").as_bytes())
        .context("failed to write runner file")?;

    Ok(())
}

fn update_mod_file(pkg_name: PackageName) -> Result<(), anyhow::Error> {
    let days = std::fs::read_to_string("src/days/mod.rs").context("failed to read mod file")?;
    let mods = days
        .lines()
        .map(|line| {
            parse_mod_line(line)
                .map(|(_, day)| day)
                .map_err(|err| anyhow!("failed to parse module line: {err}"))
        })
        .chain(std::iter::once(Ok(pkg_name)))
        .collect::<Result<BTreeSet<_>, _>>()
        .context("failed to parse mod.rs line")?;

    let mut output = File::options()
        .write(true)
        .open("src/days/mod.rs")
        .context("failed to open mod.rs to write updates")?;
    for m in mods.into_iter() {
        writeln!(&mut output, "pub mod {m};").context("failed to write line to mod.rs")?;
    }

    Ok(())
}

fn write_solver_file(pkg_name: PackageName) -> Result<(), anyhow::Error> {
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
    std::fs::write(format!("src/days/{pkg_name}.rs"), solver.as_bytes())
        .context("failed to write solver file")?;

    Ok(())
}

fn write_input_files(pkg_name: PackageName, year: usize) -> Result<(), anyhow::Error> {
    write_real_input(pkg_name, year)?;

    File::create(format!("input/{pkg_name}_test.txt"))
        .context("failed to write test input file")?;

    Ok(())
}

fn write_real_input(pkg_name: PackageName, year: usize) -> Result<(), anyhow::Error> {
    if let Ok(session) = std::env::var("AOC_SESSION") {
        let cache_folder =
            std::env::var("AOC_CACHE").context("failed to find AOC_CACHE env var")?;
        let cache_file = format!("{cache_folder}/{year}_{pkg_name}.txt");
        let input =
            retrieve_cached_or_fresh_input(pkg_name, year, &session, &cache_folder, &cache_file)?;

        std::fs::write(format!("input/{pkg_name}.txt"), input.as_bytes())
            .context("failed to write input file")?;
    } else {
        File::create(format!("input/{pkg_name}.txt")).context("failed to write input file")?;
    };

    Ok(())
}

fn retrieve_cached_or_fresh_input(
    pkg_name: PackageName,
    year: usize,
    session: &str,
    cache_folder: &str,
    cache_file: &str,
) -> Result<String, anyhow::Error> {
    match std::fs::read_to_string(cache_file) {
        Ok(cached) => {
            info!("serving cached input");
            return Ok(cached);
        }
        Err(err) => {
            if err.kind() != ErrorKind::NotFound {
                return Err(err).context("failed to read cache input file");
            }
        }
    }

    let input = retrieve_and_cache_fresh_input(pkg_name, year, session, cache_folder, cache_file)?;

    Ok(input)
}

fn retrieve_and_cache_fresh_input(
    pkg_name: PackageName,
    year: usize,
    session: &str,
    cache_folder: &str,
    cache_file: &str,
) -> Result<String, anyhow::Error> {
    let response = retrieve_fresh(pkg_name, year, session)?;
    cache_response(cache_folder, cache_file, &response)?;

    Ok(response)
}

fn retrieve_fresh(
    pkg_name: PackageName,
    year: usize,
    session: &str,
) -> Result<String, anyhow::Error> {
    let url = format!(
        "https://adventofcode.com/{year}/day/{day}/input",
        day = pkg_name.0
    );
    info!("retrieving input from url {url}");

    let client = ClientBuilder::new()
        .user_agent("https://github.com/jchevertonwynne/advent-of-code-2023")
        .build()
        .context("failed to build http client")?;

    let request = client
        .get(url)
        .header("Cookie", format!("session={session}"))
        .build()
        .context("failed to build http request")?;

    let response = client
        .execute(request)
        .context("failed to perform http request")?
        .error_for_status()
        .context("bad http response code returned")?
        .text()
        .context("failed to read http response body")?;
    info!("retrieved input");

    Ok(response)
}

fn cache_response(
    cache_folder: &str,
    cache_file: &str,
    response: &str,
) -> Result<(), anyhow::Error> {
    if let Err(err) = std::fs::create_dir(cache_folder) {
        if err.kind() != ErrorKind::AlreadyExists {
            return Err(err).context("failed to create aoc cache directory");
        }
    } else {
        info!("created ~/.aoc")
    }

    std::fs::write(cache_file, response.as_bytes())
        .context("failed to write aoc input to cache")?;
    info!("cached input to {cache_file}");

    Ok(())
}
