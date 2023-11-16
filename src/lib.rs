use std::{
    cmp::Reverse,
    fmt::{Debug, Display, Formatter},
};

use arrayvec::ArrayVec;

pub mod days;

macro_rules! impl_answer_enum {
    ( $( ($variant:tt, $ty:ty) ),* ) => {
        pub enum Answers {
            $(
                $variant($ty),
            )*
        }

        $(
            impl From<$ty> for Answers {
                fn from(t: $ty) -> Self {
                    Answers::$variant(t)
                }
            }
        )*

        // assumes all types impl Display
        impl Display for Answers {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                match self {
                    $(
                        Answers::$variant(t) => write!(f, "{t}"),
                    )*
                }
            }
        }

        impl Debug for Answers {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                match self {
                    $(
                        Answers::$variant(t) => write!(f, "{t}"),
                    )*
                }
            }
        }

        impl Eq for Answers {}

        impl PartialEq for Answers {
            fn eq(&self, other: &Self) -> bool {
                let val_self = match self {
                    $(
                    Answers::$variant(v) => format!("{v}"),
                    )*
                };
                let val_other = match other {
                    $(
                    Answers::$variant(v) => format!("{v}"),
                    )*
                };
                val_self == val_other
            }
        }
    }
}

impl_answer_enum! {
    (String, String),
    (Usize, usize),
    (U64, u64),
    (U32, u32),
    (U16, u16),
    (U8, u8),
    (Isize, isize),
    (I64, i64),
    (I32, i32),
    (I16, i16),
    (I8, i8)
}

impl From<&'_ str> for Answers {
    fn from(s: &'_ str) -> Self {
        Answers::String(s.to_string())
    }
}

pub trait IntoDayResult: Sized {
    fn into_result(self) -> anyhow::Result<DayResult> {
        Ok(self.into_day_result())
    }
    fn into_day_result(self) -> DayResult;
}

impl IntoDayResult for () {
    fn into_day_result(self) -> DayResult {
        DayResult {
            part1: None,
            part2: None,
        }
    }
}

impl<A> IntoDayResult for A
where
    A: Into<Answers>,
{
    fn into_day_result(self) -> DayResult {
        DayResult {
            part1: Some(self.into()),
            part2: None,
        }
    }
}

impl<A, B> IntoDayResult for (A, B)
where
    A: Into<Answers>,
    B: Into<Answers>,
{
    fn into_day_result(self) -> DayResult {
        let (a, b) = self;
        DayResult {
            part1: Some(a.into()),
            part2: Some(b.into()),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct DayResult {
    pub part1: Option<Answers>,
    pub part2: Option<Answers>,
}

impl Display for DayResult {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "DayResult {{")?;
        writeln!(
            f,
            "\tpart 1: {p1}",
            p1 = self
                .part1
                .as_ref()
                .map(|v| v.to_string())
                .unwrap_or("TBC".to_string())
        )?;
        writeln!(
            f,
            "\tpart 2: {p2}",
            p2 = self
                .part2
                .as_ref()
                .map(|v| v.to_string())
                .unwrap_or("TBC".to_string())
        )?;
        writeln!(f, "}}")?;
        Ok(())
    }
}

trait CollectN<T>
where
    Self: Sized,
{
    fn collect_largest<const N: usize>(self) -> ArrayVec<T, N>
    where
        T: Ord,
    {
        self.collect_by_fn((|v| Reverse(v)) as for<'a> fn(&'a T) -> Reverse<&'a T>)
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

        let comparer = |a: &_, b: &_| Ord::cmp(&f.call(a), &f.call(b));

        for (i, item) in self.enumerate() {
            if i >= N {
                let last = res
                    .pop()
                    .expect("there should always be a value as res cap is > 0");
                let smallest = std::cmp::min_by(item, last, comparer);

                res.push(smallest);
            } else {
                res.push(item);
            }

            res.sort_unstable_by(comparer);
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

pub fn get_input(day: &str, is_test: bool) -> std::io::Result<String> {
    let filepath = if is_test {
        format!("input/{day}_test.txt")
    } else {
        format!("input/{day}.txt")
    };
    std::fs::read_to_string(filepath)
}

#[macro_export]
macro_rules! aoc_args_input_only {
    ($solver:expr, $a:tt, $b:tt) => {
        $solver($a)
    };
}

#[macro_export]
macro_rules! aoc_args_both {
    ($solver:expr, $a:tt, $b:tt) => {
        $solver($a, $b)
    };
}

#[macro_export]
macro_rules! aoc_impl {
    ($day:expr, $daymod:ident, $solver:tt) => {
        use $crate::days::$daymod::solve;
        use $crate::get_input;

        fn main() -> anyhow::Result<()> {
            let day = stringify!($day);
            let is_test = std::env::var_os("TEST").is_some();
            let input = get_input(day, is_test)?;
            let solution = $solver!(solve, (&input), is_test)?;

            println!("{day}: {solution}");

            Ok(())
        }
    };
}

#[macro_export]
macro_rules! aoc {
    ($day:tt) => {
        use $crate::aoc_args_input_only;
        $crate::aoc_impl!($day, $day, aoc_args_input_only);
    };
    ($day:tt, is_test) => {
        use $crate::aoc_args_both;
        $crate::aoc_impl!($day, $day, aoc_args_both);
    };
}
