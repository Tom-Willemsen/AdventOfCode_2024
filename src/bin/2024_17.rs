#![cfg_attr(feature = "bench", feature(test))]
use advent_of_code_2024::{Cli, Parser};
use itertools::Itertools;
use std::fs;

fn run_program(mut a: u64, mut b: u64, mut c: u64, program: &[u64]) -> Vec<u64> {
    let mut ip = 0;
    let mut out = Vec::with_capacity(16);

    while ip + 1 < program.len() {
        let inst = program[ip];
        let literal = program[ip + 1];
        let combo = match literal {
            0..=3 => literal,
            4 => a,
            5 => b,
            6 => c,
            _ => u64::MAX,
        };

        match inst {
            0 => {
                debug_assert!(combo != u64::MAX);
                a /= 2_u64.pow(combo as u32);
            }
            1 => {
                b ^= literal;
            }
            2 => {
                debug_assert!(combo != u64::MAX);
                b = combo.rem_euclid(8);
            }
            3 => {
                if a != 0 {
                    ip = literal as usize;
                    continue;
                }
            }
            4 => {
                b ^= c;
            }
            5 => {
                debug_assert!(combo != u64::MAX);
                out.push(combo.rem_euclid(8));
            }
            6 => {
                debug_assert!(combo != u64::MAX);
                b = a / 2_u64.pow(combo as u32);
            }
            7 => {
                debug_assert!(combo != u64::MAX);
                c = a / 2_u64.pow(combo as u32);
            }
            _ => panic!("invalid instruction"),
        }

        ip += 2;
    }
    out
}

fn get_a(coeffs: &[u64]) -> u64 {
    coeffs
        .iter()
        .enumerate()
        .map(|(p, n)| 8u64.pow((coeffs.len() - p - 1) as u32) * n)
        .sum()
}

fn part2(nums: &mut Vec<u64>, program: &[u64], b: u64, c: u64) -> Option<u64> {
    if nums.len() == program.len() {
        return Some(get_a(nums));
    }

    (0..8)
        .filter_map(|i| {
            nums.push(i);

            let length = nums.len();
            let solution = run_program(get_a(nums), b, c, program)
                .ends_with(&program[program.len() - length..])
                .then(|| part2(nums, program, b, c))
                .flatten();

            nums.pop();
            solution
        })
        .min()
}

fn calculate(raw_inp: &str) -> (String, u64) {
    let (head, tail) = raw_inp.split_once("\n\n").expect("bad format");

    let (a, b, c) = head
        .lines()
        .filter_map(|line| line.split_once(": ").and_then(|x| x.1.parse::<u64>().ok()))
        .collect_tuple()
        .expect("too many registers");

    let program = tail
        .strip_prefix("Program: ")
        .expect("bad format")
        .split(",")
        .filter_map(|x| x.trim().parse().ok())
        .collect::<Vec<u64>>();

    let p1 = run_program(a, b, c, &program).into_iter().join(",");
    let p2 = part2(&mut Vec::with_capacity(16), &program, b, c).expect("no p2 solution?");

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

    const EXAMPLE_DATA: &str = include_str!("../../inputs/examples/2024_17");
    const REAL_DATA: &str = include_str!("../../inputs/real/2024_17");

    #[test]
    fn test_simple_example_1() {
        assert_eq!(run_program(10, 0, 0, &[5, 0, 5, 1, 5, 4]), &[0, 1, 2]);
    }

    #[test]
    fn test_simple_example_2() {
        assert_eq!(
            run_program(2024, 0, 0, &[0, 1, 5, 4, 3, 0]),
            &[4, 2, 5, 6, 7, 7, 7, 7, 3, 1, 0]
        );
    }

    #[test]
    fn test_example_p1() {
        assert_eq!(
            run_program(729, 0, 0, &[0, 1, 5, 4, 3, 0]),
            &[4, 6, 3, 5, 6, 3, 5, 2, 1, 0]
        );
    }

    #[test]
    fn test_example_p2() {
        assert_eq!(calculate(EXAMPLE_DATA).1, 117440)
    }

    #[test]
    fn test_real() {
        assert_eq!(
            calculate(&REAL_DATA),
            ("1,0,2,0,5,7,2,1,3".to_string(), 265652340990875)
        );
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
