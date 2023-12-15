use std::{borrow::BorrowMut, cell::Cell, collections::hash_map::Entry};

use fxhash::FxHashMap;

use crate::{DayResult, IntoDayResult};

pub fn solve(input: &str) -> anyhow::Result<DayResult> {
    let mut moveable = vec![];
    let mut world = input
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.as_bytes()
                .iter()
                .enumerate()
                .map(|(x, &t)| {
                    if t == b'O' {
                        let rock = Moveable {
                            x: Cell::new(x),
                            y: Cell::new(y),
                        };
                        moveable.push(rock);
                    }

                    t == b'.'
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    north(&mut moveable, &mut world);
    let p1 = score(&moveable, world.len());
    west(&mut moveable, &mut world);
    south(&mut moveable, &mut world);
    east(&mut moveable, &mut world);

    let mut seen = FxHashMap::default();

    let mut p2 = 0;

    for cycles in 1.. {
        match seen.entry((repr(&moveable), score(&moveable, world.len()))) {
            Entry::Occupied(prev_cycles) => {
                let prev_cycles = prev_cycles.get();
                let cycle_period = cycles - prev_cycles;
                let rem = (1_000_000_000 - prev_cycles) % cycle_period;
                for _ in 0..rem {
                    cycle(&mut moveable, &mut world);
                }
                p2 = score(&moveable, world.len());
                break;
            }
            Entry::Vacant(v) => {
                v.insert(cycles);
            }
        }

        cycle(&mut moveable, &mut world);
    }

    (p1, p2).into_result()
}

struct Moveable {
    x: Cell<usize>,
    y: Cell<usize>,
}

fn repr(moveable: &[Moveable]) -> Vec<usize> {
    let mut res = vec![];

    for m in moveable {
        res.push((m.x.get() << 32) + m.y.get());
    }

    res.sort_unstable();

    res
}

fn score(moveable: &[Moveable], world_height: usize) -> usize {
    moveable.iter().map(|m| world_height - m.y.get()).sum()
}

fn cycle(moveable: &mut [Moveable], world: &mut [impl BorrowMut<[bool]>]) {
    north(moveable, world);
    west(moveable, world);
    south(moveable, world);
    east(moveable, world);
}

fn north(moveable: &mut [Moveable], world: &mut [impl BorrowMut<[bool]>]) {
    moveable.sort_unstable_by_key(|m| m.y.get());
    for m in moveable {
        let mut y = m.y.get();
        while y != 0 {
            if world[m.y.get() - 1].borrow_mut()[m.x.get()] {
                world[m.y.get() - 1].borrow_mut()[m.x.get()] = false;
                world[m.y.get()].borrow_mut()[m.x.get()] = true;
                m.y.set(m.y.get() - 1);
            } else {
                break;
            }
            y -= 1;
        }
    }
}

fn south(moveable: &mut [Moveable], world: &mut [impl BorrowMut<[bool]>]) {
    moveable.sort_unstable_by_key(|m| m.y.get());
    for m in moveable.iter_mut().rev() {
        let mut y = m.y.get();
        while y != world.len() - 1 {
            if world[m.y.get() + 1].borrow_mut()[m.x.get()] {
                world[m.y.get() + 1].borrow_mut()[m.x.get()] = false;
                world[m.y.get()].borrow_mut()[m.x.get()] = true;
                m.y.set(m.y.get() + 1);
            } else {
                break;
            }
            y += 1;
        }
    }
}

fn west(moveable: &mut [Moveable], world: &mut [impl BorrowMut<[bool]>]) {
    moveable.sort_unstable_by_key(|m| m.x.get());
    for m in moveable {
        let mut x = m.x.get();
        while x != 0 {
            if world[m.y.get()].borrow_mut()[m.x.get() - 1] {
                world[m.y.get()].borrow_mut()[m.x.get() - 1] = false;
                world[m.y.get()].borrow_mut()[m.x.get()] = true;
                m.x.set(m.x.get() - 1);
            } else {
                break;
            }
            x -= 1;
        }
    }
}

fn east(moveable: &mut [Moveable], world: &mut [impl BorrowMut<[bool]>]) {
    moveable.sort_unstable_by_key(|m| m.x.get());
    for m in moveable.iter_mut().rev() {
        let mut x = m.x.get();
        while x != world[0].borrow().len() - 1 {
            if world[m.y.get()].borrow_mut()[m.x.get() + 1] {
                world[m.y.get()].borrow_mut()[m.x.get() + 1] = false;
                world[m.y.get()].borrow_mut()[m.x.get()] = true;
                m.x.set(m.x.get() + 1);
            } else {
                break;
            }
            x += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{days::day14::solve, IntoDayResult};

    #[test]
    fn works_for_example() {
        const INPUT: &str = include_str!("../../input/day14_test.txt");
        let solution = solve(INPUT).unwrap();
        assert_eq!((136, 64).into_day_result(), solution);
    }

    #[test]
    fn works_for_input() {
        const INPUT: &str = include_str!("../../input/day14.txt");
        let solution = solve(INPUT).unwrap();
        assert_eq!((108_813, 104_533).into_day_result(), solution);
    }
}
