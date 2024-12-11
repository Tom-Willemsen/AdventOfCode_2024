#![cfg_attr(feature = "bench", feature(test))]
use advent_of_code_2024::{Cli, Parser};
use num_integer::div_rem;
use rustc_hash::{FxBuildHasher, FxHashMap};
use std::fs;

fn split_num(n: u64) -> Option<(u64, u64)> {
    let a = n.ilog10() + 1;

    if a % 2 == 0 {
        Some(div_rem(n, 10_u64.pow(a / 2)))
    } else {
        None
    }
}

fn blink<const P1_BLINKS: usize, const P2_BLINKS: usize>(raw_inp: &str) -> (usize, usize) {
    let mut stones = raw_inp
        .trim()
        .split(" ")
        .filter_map(|n| n.parse().ok())
        .fold(FxHashMap::default(), |mut m, n| {
            *m.entry(n).or_insert(0) += 1;
            m
        });

    let mut p1 = 0;

    for i in 0..P2_BLINKS {
        if i == P1_BLINKS {
            p1 = stones.values().sum();
        }

        let cap = stones.capacity();
        stones = stones.into_iter().fold(
            FxHashMap::with_capacity_and_hasher(cap, FxBuildHasher::default()),
            |mut m, (n, v)| {
                if n == 0 {
                    *m.entry(1).or_insert(0) += v;
                } else if let Some((a, b)) = split_num(n) {
                    *m.entry(a).or_insert(0) += v;
                    *m.entry(b).or_insert(0) += v;
                } else {
                    *m.entry(n * 2024).or_insert(0) += v;
                }
                m
            },
        );
    }

    (p1, stones.values().sum())
}

fn calculate(raw_inp: &str) -> (usize, usize) {
    blink::<25, 75>(raw_inp)
}

fn main() {
    let args = Cli::parse();

    let inp = fs::read_to_string(args.input).expect("can't open input file");

    let (p1, p2) = calculate(&inp);
    println!("{}\n{}", p1, p2);
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_DATA: &str = "125 17";
    const REAL_DATA: &str = include_str!("../../inputs/real/2024_11");

    #[test]
    fn test_split_num() {
        assert_eq!(split_num(12), Some((1, 2)));
        assert_eq!(split_num(123), None);
        assert_eq!(split_num(1234), Some((12, 34)));
        assert_eq!(split_num(12345), None);
        assert_eq!(split_num(123456), Some((123, 456)));

        assert_eq!(split_num(1), None);
        assert_eq!(split_num(10), Some((1, 0)));
        assert_eq!(split_num(100), None);
        assert_eq!(split_num(1000), Some((10, 0)));
        assert_eq!(split_num(10000), None);
        assert_eq!(split_num(100000), Some((100, 0)));
    }

    #[test]
    fn test_example() {
        assert_eq!(blink::<1, 2>(&EXAMPLE_DATA), (3, 4));
        assert_eq!(blink::<3, 4>(&EXAMPLE_DATA), (5, 9));
        assert_eq!(blink::<5, 6>(&EXAMPLE_DATA), (13, 22));
        assert_eq!(blink::<6, 25>(&EXAMPLE_DATA), (22, 55312));
    }

    #[test]
    fn test_real() {
        assert_eq!(calculate(&REAL_DATA), (186424, 219838428124832));
    }

    #[cfg(feature = "bench")]
    mod benches {
        extern crate test;
        use test::{black_box, Bencher};

        use super::*;

        #[bench]
        fn bench(b: &mut Bencher) {
            b.iter(|| calculate(black_box(REAL_DATA)));
        }
    }
}
