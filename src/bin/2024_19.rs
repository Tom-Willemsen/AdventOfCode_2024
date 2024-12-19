#![cfg_attr(feature = "bench", feature(test))]
use advent_of_code_2024::{Cli, Parser};
use ahash::AHashMap;
use rayon::prelude::*;
use std::fs;

fn valid_paths<'a>(line: &'a str, towels: &[&str], cache: &mut AHashMap<&'a str, u64>) -> u64 {
    if line.is_empty() {
        1
    } else if let Some(&cached_result) = cache.get(line) {
        cached_result
    } else {
        let r = towels
            .iter()
            .filter(|&&t| line.starts_with(t))
            .map(|&t| valid_paths(&line[t.len()..], towels, cache))
            .sum();

        cache.insert(line, r);
        r
    }
}

fn calculate(raw_inp: &str) -> (u64, u64) {
    let (towels, arrangements) = raw_inp.split_once("\n\n").expect("bad format");
    let towels = towels.trim().split(", ").collect::<Vec<_>>();

    arrangements
        .par_lines()
        .map(|line| valid_paths(line, &towels, &mut AHashMap::default()))
        .map(|n| ((n >= 1) as u64, n))
        .reduce(|| (0, 0), |a, b| (a.0 + b.0, a.1 + b.1))
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

    const EXAMPLE_DATA: &str = include_str!("../../inputs/examples/2024_19");
    const REAL_DATA: &str = include_str!("../../inputs/real/2024_19");

    #[test]
    fn test_example() {
        assert_eq!(calculate(EXAMPLE_DATA), (6, 16));
    }

    #[test]
    fn test_real() {
        assert_eq!(calculate(REAL_DATA), (226, 601201576113503));
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
