#![cfg_attr(feature = "bench", feature(test))]
use advent_of_code_2024::{Cli, Parser};
use num_integer::div_rem;
use std::fs;

fn can_match<const ALLOW_COMBINATION: bool>(result: i64, nums: &[i64]) -> bool {
    if let Some(&lastnum) = nums.last() {
        debug_assert!(lastnum > 0);
        let a = &nums[0..nums.len() - 1];

        if ALLOW_COMBINATION {
            let (d, m) = div_rem(result - lastnum, 10i64.pow(lastnum.ilog10() + 1));
            if m == 0 && can_match::<ALLOW_COMBINATION>(d, a) {
                return true;
            }
        }

        let (d, m) = div_rem(result, lastnum);
        if m == 0 && can_match::<ALLOW_COMBINATION>(d, a) {
            return true;
        }

        if can_match::<ALLOW_COMBINATION>(result - lastnum, a) {
            return true;
        }
        false
    } else {
        result == 0
    }
}

fn calculate(raw_inp: &str) -> (i64, i64) {
    raw_inp
        .lines()
        .map(|line| {
            let (head, tail) = line.split_once(": ").expect("invalid format");

            let result = head.parse::<i64>().expect("NaN");
            let nums = tail
                .split(" ")
                .map(|n| n.parse().expect("NaN"))
                .collect::<Vec<i64>>();

            let one = can_match::<false>(result, &nums);
            let two = one || can_match::<true>(result, &nums);

            (if one { result } else { 0 }, if two { result } else { 0 })
        })
        .fold((0, 0), |a, b| (a.0 + b.0, a.1 + b.1))
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

    const EXAMPLE_DATA: &str = include_str!("../../inputs/examples/2024_07");
    const REAL_DATA: &str = include_str!("../../inputs/real/2024_07");

    #[test]
    fn test_example() {
        assert_eq!(calculate(&EXAMPLE_DATA), (3749, 11387));
    }

    #[test]
    fn test_real() {
        assert_eq!(calculate(&REAL_DATA), (2437272016585, 162987117690649));
    }

    #[test]
    fn test_combination() {
        assert!(can_match::<true>(15, &[1, 5]));
        assert!(can_match::<true>(105, &[10, 5]));
        assert!(can_match::<true>(1005, &[100, 5]));
        assert!(can_match::<true>(5100, &[5, 100]));
        assert!(can_match::<true>(9999, &[99, 99]));

        assert!(!can_match::<false>(15, &[1, 5]));
        assert!(!can_match::<false>(105, &[10, 5]));
        assert!(!can_match::<false>(1005, &[100, 5]));
        assert!(!can_match::<false>(5100, &[5, 100]));
        assert!(!can_match::<false>(9999, &[99, 99]));
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
