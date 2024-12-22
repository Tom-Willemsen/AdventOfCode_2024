#![cfg_attr(feature = "bench", feature(test))]
use advent_of_code_2024::{Cli, Parser};
use itertools::{repeat_n, Itertools};
use rayon::prelude::*;
use std::{collections::VecDeque, fs};

fn next_secret(mut n: i64) -> i64 {
    n = (n ^ (n << 6)) & 0xFFFFFF;
    n = (n ^ (n >> 5)) & 0xFFFFFF;
    n = (n ^ (n << 11)) & 0xFFFFFF;
    n
}

fn ones_digit(n: i64) -> i64 {
    n % 10
}

fn sell(n: i64, c1: i64, c2: i64, c3: i64, c4: i64) -> Option<i64> {
    let mut diffs = VecDeque::default();
    let mut n = n;
    for _ in 0..2000 {
        let next_n = next_secret(n);

        diffs.push_back(ones_digit(next_n) - ones_digit(n));
        n = next_n;

        if diffs.len() > 4 {
            diffs.pop_front();
        }

        if diffs.get(0) == Some(&c1)
            && diffs.get(1) == Some(&c2)
            && diffs.get(2) == Some(&c3)
            && diffs.get(3) == Some(&c4)
        {
            return Some(ones_digit(n));
        }
    }
    None
}

fn brute_force(raw_inp: &str) -> i64 {
    repeat_n((-9..=9).into_iter(), 4)
        .multi_cartesian_product()
        .par_bridge()
        .filter_map(|v| {
            let c1 = v[0];
            let c2 = v[1];
            let c3 = v[2];
            let c4 = v[3];

            if c1 + c2 < -9
                || c1 + c2 > 9
                || c2 + c3 < -9
                || c2 + c3 > 9
                || c3 + c4 < -9
                || c3 + c4 > 9
                || c1 + c2 + c3 < -9
                || c1 + c2 + c3 > 9
                || c2 + c3 + c4 < -9
                || c2 + c3 + c4 > 9
                || c1 + c2 + c3 + c4 < -9
                || c1 + c2 + c3 + c4 > 9
            {
                return None;
            }

            Some(
                raw_inp
                    .lines()
                    .filter_map(|line| line.parse::<i64>().ok())
                    .filter_map(|n| sell(n, c1, c2, c3, c4))
                    .sum::<i64>(),
            )
        })
        .max()
        .unwrap()
}

fn calculate(raw_inp: &str) -> (i64, i64) {
    let p1 = raw_inp
        .lines()
        .filter_map(|line| line.parse::<i64>().ok())
        .map(|n| {
            let mut n = n;
            for _ in 0..2000 {
                n = next_secret(n);
            }
            n
        })
        .sum::<i64>();

    let p2 = brute_force(raw_inp);

    (p1, p2)
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
