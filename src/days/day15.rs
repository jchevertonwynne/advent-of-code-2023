use crate::{DayResult, IntoDayResult};

pub fn solve(input: &str) -> anyhow::Result<DayResult> {
    let input = input.trim().as_bytes();
    let mut p1 = 0;

    let mut boxes: [Vec<Lens>; 256] = std::array::from_fn(|_| Vec::new());

    for cmd in input.split(|&b| b == b',') {
        let mut box_to_use = 0;
        let (label, op) = if cmd[cmd.len() - 1] == b'-' {
            cmd.split_at(cmd.len() - 1)
        } else {
            cmd.split_at(cmd.len() - 2)
        };
        for &b in label {
            box_to_use += b as u64;
            box_to_use *= 17;
            box_to_use %= 256;
        }
        let mut h = box_to_use;
        for &b in op {
            h += b as u64;
            h *= 17;
            h %= 256;
        }
        p1 += h;

        if op[0] == b'=' {
            let focal_length = (op[1] - b'0') as usize;
            if let Some(lens) = boxes[box_to_use as usize]
                .iter_mut()
                .find(|l| l.label == label)
            {
                lens.focal_length = focal_length;
            } else {
                boxes[box_to_use as usize].push(Lens {
                    label,
                    focal_length,
                });
            }
        } else {
            boxes[box_to_use as usize].retain(|l| l.label != label);
        }
    }

    let p2 = boxes
        .iter()
        .enumerate()
        .flat_map(|(box_ind, _box)| {
            _box.iter()
                .enumerate()
                .map(move |(lens_ind, lens)| (1 + box_ind) * (lens_ind + 1) * lens.focal_length)
        })
        .sum::<usize>();

    (p1, p2).into_result()
}

struct Lens<'a> {
    label: &'a [u8],
    focal_length: usize,
}

#[cfg(test)]
mod tests {
    use crate::{days::day15::solve, IntoDayResult};

    #[test]
    fn works_for_example() {
        const INPUT: &str = include_str!("../../input/day15_test.txt");
        let solution = solve(INPUT).unwrap();
        assert_eq!((1_320, 145).into_day_result(), solution);
    }

    #[test]
    fn works_for_input() {
        const INPUT: &str = include_str!("../../input/day15.txt");
        let solution = solve(INPUT).unwrap();
        assert_eq!((510_801, 212_763).into_day_result(), solution);
    }
}
