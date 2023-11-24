# Advent of Code 2023

My repository with a solution boilerplate generator built in

Previous years I've golfed solutions endlessly, this year I want to write nice
ones that aren't too unperformant. Last year I inevitably had to sacrifice
`String` and `&str` to avoid the utf8 overhead on iterating over what I know is
ascii input

## Running days

`$ cargo run --bin day01` to use real input
`$ TEST=1 cargo run --bin day01` to use test input

## `aoc` solution stub generator installation

`$ cargo install --path . --bin aoc`

## `aoc` usage

`$ aoc 2` if installed or `$ cargo run --bin aoc 2` 

For day 2 this will create the following files:

- binary in `src/bin`
- soution in `src/days`
- added to `src/days/mod.rs`
- input files for real & test inputs (real input if env vars provided)

Generated days are not automatically added to benchmarks

### `aoc` env vars

- `AOC_SESSION` - Your session cookie - equired. You can find this on the network tab in your browser when you press f12. Optional - empty file created if not provided
- `AOC_CACHE` - The location for the local input cache - required

### `aoc` cmd line args

- `-year` `-y` - year, default `2023`
- `-overwrite` `-o` by default overwrite fails if a solution file is found, this disables that
