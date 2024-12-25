#![cfg_attr(feature = "bench", feature(test))]
use advent_of_code_2024::{Cli, Parser};
use std::fs;

fn calculate(raw_inp: &str) -> usize {
    let mut locks = Vec::with_capacity(250);
    let mut keys = Vec::with_capacity(250);

    raw_inp
        .split("\n\n")
        .map(|s| {
            let c = s
                .bytes()
                .filter(|b| b == &b'#' || b == &b'.')
                .zip(0u32..)
                .filter(|(b, _)| b == &b'#')
                .map(|(_, idx)| (1 << (4 * (idx % 5))))
                .sum::<u32>();
            (s.starts_with('#'), c)
        })
        .for_each(|(is_lock, c)| {
            if is_lock {
                locks.push(c);
            } else {
                keys.push(c);
            }
        });

    locks
        .into_iter()
        .map(|lock| {
            keys.iter()
                .filter(|&&key| ((lock + key) & 0x88888) == 0)
                .count()
        })
        .sum()
}

fn main() {
    let args = Cli::parse();

    let inp = fs::read_to_string(args.input).expect("can't open input file");

    let p1 = calculate(&inp);
    println!("{}", p1);
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_DATA: &str = include_str!("../../inputs/examples/2024_25");
    const REAL_DATA: &str = include_str!("../../inputs/real/2024_25");

    #[test]
    fn test_example() {
        assert_eq!(calculate(EXAMPLE_DATA), 3);
    }

    #[test]
    fn test_real() {
        assert_eq!(calculate(REAL_DATA), 3196);
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
