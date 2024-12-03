#![cfg_attr(feature = "bench", feature(test))]
use advent_of_code_2024::{Cli, Parser};
use std::fs;

fn try_parse_mul(raw_inp: &str) -> Option<(i32, i32)> {
    if !raw_inp.starts_with("mul(") {
        return None;
    }
    if let Some((a, tail)) = raw_inp[4..].split_once(",") {
        if let Ok(a) = a.parse() {
            if let Some((b, _)) = tail.split_once(")") {
                if let Ok(b) = b.parse() {
                    return Some((a, b));
                }
            }
        }
    }
    None
}

fn calculate(raw_inp: &str) -> (i32, i32) {
    let mut enabled: bool = true;
    let mut p1 = 0;
    let mut p2 = 0;

    for start in 0..raw_inp.len() {
        if raw_inp[start..].starts_with("do()") {
            enabled = true;
        } else if raw_inp[start..].starts_with("don't()") {
            enabled = false;
        } else if let Some((a, b)) = try_parse_mul(&raw_inp[start..]) {
            p1 += a * b;
            if enabled {
                p2 += a * b;
            }
        }
    }
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

    const EXAMPLE_DATA_P1: &str = include_str!("../../inputs/examples/2024_03_p1");
    const EXAMPLE_DATA_P2: &str = include_str!("../../inputs/examples/2024_03_p2");
    const REAL_DATA: &str = include_str!("../../inputs/real/2024_03");

    #[test]
    fn test_simple() {
        assert_eq!(calculate(""), (0, 0));
        assert_eq!(calculate("mul(2,3"), (0, 0));
        assert_eq!(calculate("mul(2,3)"), (6, 6));
        assert_eq!(calculate("don't()mul(2,3)"), (6, 0));
        assert_eq!(calculate("mul(2,3)don't()mul(2,3)"), (12, 6));
    }

    #[test]
    fn test_example_p1() {
        assert_eq!(calculate(&EXAMPLE_DATA_P1).0, 161);
    }

    #[test]
    fn test_example_p2() {
        assert_eq!(calculate(&EXAMPLE_DATA_P2).1, 48);
    }

    #[test]
    fn test_real() {
        assert_eq!(calculate(&REAL_DATA), (183380722, 82733683));
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
