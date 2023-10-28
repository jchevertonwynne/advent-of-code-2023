use std::{
    cmp::{Ord, Reverse},
    iter::Iterator,
};

use arrayvec::ArrayVec;
use itertools::Itertools;

fn main() -> anyhow::Result<()> {
    let input = std::fs::read_to_string("input/day01.txt")?;

    let top_three = input
        .lines()
        .map(|line| line.parse::<usize>().ok())
        .batching(|it| it.map_while(|num| num).sum1::<usize>())
        .collect_largest::<3>();

    let part1 = top_three[0];
    let part2 = top_three.iter().sum::<usize>();

    println!("part1 = {part1} part2 = {part2}");

    Ok(())
}

trait CollectN<T>
where
    Self: Sized,
{
    fn collect_largest<const N: usize>(self) -> ArrayVec<T, N>
    where
        T: Ord,
    {
        self.collect_by_fn(reverse_identity)
    }

    fn collect_by_fn<const N: usize, F>(self, f: F) -> ArrayVec<T, N>
    where
        F: for<'a> Callable<&'a T>;
}

impl<I, T> CollectN<T> for I
where
    I: Iterator<Item = T>,
{
    fn collect_by_fn<const N: usize, F>(self, f: F) -> ArrayVec<T, N>
    where
        F: for<'a> Callable<&'a T>,
    {
        let mut res = ArrayVec::new();

        if N == 0 {
            return res;
        }

        for item in self {
            if let Err(err) = res.try_push(item) {
                let item = err.element();
                let last = res
                    .pop()
                    .expect("there should always be a value as res cap is > 0");
                let largest = std::cmp::min_by(item, last, |a, b| Ord::cmp(&f.call(a), &f.call(b)));

                res.push(largest);
                res.sort_unstable_by(|a, b| Ord::cmp(&f.call(a), &f.call(b)));
            }
        }

        res
    }
}

trait Callable<T> {
    type Output: Ord;

    fn call(&self, arg: T) -> Self::Output;
}

impl<F, T, U> Callable<T> for F
where
    U: Ord,
    F: Fn(T) -> U,
{
    type Output = U;

    fn call(&self, arg: T) -> Self::Output {
        (*self)(arg)
    }
}

fn reverse_identity<T>(t: &T) -> Reverse<&T> {
    Reverse(t)
}
