fn main() -> anyhow::Result<()> {
    let input = std::fs::read_to_string("input/day01.txt")?;

    let solution = advent_of_code_2023::days::day01::solve(&input)?;

    println!("{solution}");

    Ok(())
}
