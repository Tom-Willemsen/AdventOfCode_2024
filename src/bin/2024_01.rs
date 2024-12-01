#![cfg_attr(feature = "bench", feature(test))]
use advent_of_code_2024::{Cli, Parser};
use ahash::AHashMap;
use itertools::Itertools;
use std::{fs, iter::zip};

fn calculate(raw_inp: &str) -> (i32, i32) {
    let (mut left, mut right): (Vec<i32>, Vec<i32>) = raw_inp
        .lines()
        .map(|line| line.split_once("   ").expect("invalid format"))
        .map(|(l, r)| {
            (
                l.parse::<i32>().expect("NaN"),
                r.parse::<i32>().expect("NaN"),
            )
        })
        .unzip();

    left.sort_unstable();
    right.sort_unstable();

    let right_count: AHashMap<i32, i32> = right
        .iter()
        .dedup_with_count()
        .map(|(count, v)| (*v, count as i32))
        .collect();

    let p1 = zip(&left, &right).map(|(l, r)| (l - r).abs()).sum();

    let p2 = left
        .into_iter()
        .map(|l| l * right_count.get(&l).unwrap_or(&0))
        .sum();

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

    const EXAMPLE_DATA: &str = include_str!("../../inputs/examples/2024_01");
    const REAL_DATA: &str = include_str!("../../inputs/real/2024_01");

    #[test]
    fn test_example() {
        assert_eq!(calculate(&EXAMPLE_DATA), (11, 31));
    }

    #[test]
    fn test_real() {
        assert_eq!(calculate(&REAL_DATA), (1646452, 23609874));
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
