#![cfg_attr(feature = "bench", feature(test))]
use advent_of_code_2024::{Cli, Parser};
use ahash::HashMapExt;
use mimalloc::MiMalloc;
use rayon::prelude::*;
use rustc_hash::FxHashMap;
use std::fs;

#[global_allocator]
static GLOBAL_ALLOC: MiMalloc = MiMalloc;

fn next_secret(mut n: i64) -> i64 {
    n = (n ^ (n << 6)) & 0xFFFFFF;
    n = (n ^ (n >> 5)) & 0xFFFFFF;
    n = (n ^ (n << 11)) & 0xFFFFFF;
    n
}

fn sell(n: i64) -> (i64, FxHashMap<u32, i64>) {
    let mut n = n;
    let mut map = FxHashMap::with_capacity(2000);

    let mut diffs: u32 = 0;

    for i in 0..2000 {
        let next_n = next_secret(n);

        let prev_ones = n % 10;
        let next_ones = next_n % 10;

        // who dis?
        diffs <<= 8;
        diffs |= (next_ones - prev_ones + 10) as u32;

        n = next_n;

        if i >= 3 {
            map.entry(diffs).or_insert(next_ones);
        }
    }
    (n, map)
}

fn calculate(raw_inp: &str) -> (i64, i64) {
    let (p1, p2) = raw_inp
        .par_lines()
        .filter_map(|line| line.parse::<i64>().ok())
        .map(sell)
        .reduce(
            || (0, FxHashMap::with_capacity(5000)),
            |mut acc, elem| {
                elem.1.into_iter().for_each(|(k, v)| {
                    *acc.1.entry(k).or_insert(0) += v;
                });
                acc.0 += elem.0;
                acc
            },
        );

    (p1, p2.into_values().max().expect("no solution?"))
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

    const EXAMPLE_DATA_P1: &str = include_str!("../../inputs/examples/2024_22_p1");
    const EXAMPLE_DATA_P2: &str = include_str!("../../inputs/examples/2024_22_p2");
    const REAL_DATA: &str = include_str!("../../inputs/real/2024_22");

    #[test]
    fn test_example() {
        assert_eq!(calculate(EXAMPLE_DATA_P1).0, 37327623);
        assert_eq!(calculate(EXAMPLE_DATA_P2).1, 23);
    }

    #[test]
    fn test_real() {
        assert_eq!(calculate(REAL_DATA), (16039090236, 1808));
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
