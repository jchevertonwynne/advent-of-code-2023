use crate::{DayResult, IntoDayResult};

pub fn solve(input: &str) -> anyhow::Result<DayResult> {
    let mut p1 = 0;


    let world = input.lines().map(|l| l.as_bytes()).collect::<Vec<_>>();

    let mut building_number = false;
    let mut found_symbol = false;

    for (y, line) in world.iter().enumerate() {
        for (x, &b) in line.iter().enumerate() {
            if b.is_ascii_digit() {
                    if !building_number {
                    found_symbol = true;
                }
                    building_number = true;
                    for (i, j) in (-1_isize..=1).zip(-1_isize..=1) {
                        let nx: usize = match ((x as isize) - i).try_into() {
                            Ok(nx) => nx,
                            Err(_) => continue,
                        };
                        let ny: usize = match ((y as isize) - j).try_into() {
                            Ok(ny) => ny,
                            Err(_) => continue,
                        };
                        let t = world[ny][nx];
                        if t != b'.' && !t.is_ascii_digit() {
                            found_symbol = true;
                        }
                    }
            } else {
                building_number = false;
            }
        }
    }
    
    p1.into_result()
}

#[cfg(test)]
mod tests {
    use crate::{days::day03::solve, IntoDayResult};

    #[test]
    fn works_for_example() {
        const INPUT: &str = include_str!("../../input/day03_test.txt");
        let solution = solve(INPUT).unwrap();
        assert_eq!(
            ().into_day_result(),
            solution
        );
    }

    #[test]
    fn works_for_input() {
        const INPUT: &str = include_str!("../../input/day03.txt");
        let solution = solve(INPUT).unwrap();
        assert_eq!(
            ().into_day_result(),
            solution
        );
    }
}
