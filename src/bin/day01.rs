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
        T: Ord;
}

impl<I> CollectN<I::Item> for I
where
    I: std::iter::Iterator,
{
    fn collect_largest<const N: usize>(&mut self) -> arrayvec::ArrayVec<I::Item, N>
    where
        I::Item: Ord,
    {
        let mut res = arrayvec::ArrayVec::new();

        for item in self {
            if let Err(err) = res.try_push(item) {
                let item = err.element();
                let last = res.pop().expect("should always have a value");
                let largest = std::cmp::max(item, last);
                res.push(largest);
                res.sort_unstable_by(|a, b| b.cmp(a));
            }
        }

        res
    }
}
