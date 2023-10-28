use std::cmp::{Ord, Reverse};

use itertools::Itertools;

fn main() -> anyhow::Result<()> {
    let input = std::fs::read_to_string("input/day01.txt")?;
    println!("hello world!");

    let res = input
        .lines()
        .map(|line| line.parse::<usize>().ok())
        .batching(|it| it.map_while(|num| num).sum1::<usize>())
        .collect_largest::<3>();

    let part1 = res[0];
    let part2 = res.iter().sum::<usize>();

    println!("part1 = {part1} part2 = {part2}");

    Ok(())
}

trait CollectN<T> {
    fn collect_largest<const N: usize>(&mut self) -> arrayvec::ArrayVec<T, N>
    where
        T: Ord,
    {
        self.collect_by_fn(reverse_identity)
    }

    fn collect_by_fn<'b, const N: usize, F, U>(&mut self, f: F) -> arrayvec::ArrayVec<T, N>
    where
        T: 'b,
        U: Ord,
        F: for<'a> Callable<'a, 'b, T, U>;
}

impl<I> CollectN<I::Item> for I
where
    I: std::iter::Iterator,
{
    fn collect_by_fn<'b, const N: usize, F, U>(&mut self, f: F) -> arrayvec::ArrayVec<I::Item, N>
    where
        I::Item: 'b,
        U: Ord,
        F: for<'a> Callable<'a, 'b, I::Item, U>,
    {
        let mut res = arrayvec::ArrayVec::new();

        for item in self {
            if let Err(err) = res.try_push(item) {
                let item = err.element();
                let last = res.pop().expect("should always have a value");
                let largest = std::cmp::min_by(item, last, |a, b| Ord::cmp(&f(a), &f(b)));

                res.push(largest);
                res.sort_unstable_by(|a, b| Ord::cmp(&f(a), &f(b)));
            }
        }

        res
    }
}

trait Callable<'a, 'b, T, U>: Fn(&'a T) -> U
where
    T: 'b,
    U: 'a,
    'b: 'a,
{
}

impl<'a, 'b, F, T, U> Callable<'a, 'b, T, U> for F
where
    T: 'b,
    U: 'a,
    'b: 'a,
    F: Fn(&'a T) -> U,
{
}

fn identity<T>(t: &T) -> &T {
    t
}

fn reverse_identity<T>(t: &T) -> Reverse<&T> {
    Reverse(t)
}
