#![cfg_attr(feature = "bench", feature(test))]
use advent_of_code_2024::{Cli, Parser};
use itertools::Itertools;
use std::fs;

fn is_safe(nums: &[i32]) -> bool {
    let first_dir =
        nums.get(1).expect("not enough numbers") > nums.first().expect("not enough numbers");

    nums.iter()
        .tuple_windows()
        .all(|(a, b)| (1..=3).contains(&(a - b).abs()) && ((b > a) == first_dir))
}

fn calculate(raw_inp: &str) -> (i32, i32) {
    raw_inp
        .lines()
        .map(|line| {
            line.split(" ")
                .map(|n| n.parse().expect("NaN"))
                .collect::<Vec<_>>()
        })
        .map(|r| {
            let p1_is_safe = is_safe(&r);
            let p2_is_safe = p1_is_safe
                || r.iter()
                    .cloned()
                    .combinations(r.len() - 1)
                    .any(|sr| is_safe(&sr));

            (p1_is_safe, p2_is_safe)
        })
        .fold((0, 0), |a, b| (a.0 + b.0 as i32, a.1 + b.1 as i32))
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

    const EXAMPLE_DATA: &str = include_str!("../../inputs/examples/2024_02");
    const REAL_DATA: &str = include_str!("../../inputs/real/2024_02");

    #[test]
    fn test_example() {
        assert_eq!(calculate(&EXAMPLE_DATA), (2, 4));
    }

    #[test]
    fn test_real() {
        assert_eq!(calculate(&REAL_DATA), (660, 689));
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
