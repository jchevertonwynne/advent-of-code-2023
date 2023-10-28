fn main() -> anyhow::Result<()> {
    let input = std::fs::read_to_string("input/day01.txt")?;

    let (part1, part2) = advent_of_code_2023::days::day01::solve(&input)?;

    println!("part1 = {part1} part2 = {part2}");

    Ok(())
}
