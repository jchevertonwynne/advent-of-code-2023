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
- empty input files for resl & test inputs

Generated days are not automatically added to benchmarks

`aoc` assumes that is the binary file exists then everything is already set up,
if you delete this & rerun generation it will overwrite any of the other files

### `aoc` env vars

- `AOC_SESSION` - Your session cookie. You can find this on the network tab in your browser when you press f12. If this is not provided an empty input file will be created instead
- `AOC_YEAR` - The year to retrieve. This defaults to 2023

If retrieving inputs, `aoc` requires the `HOME` env var to be set