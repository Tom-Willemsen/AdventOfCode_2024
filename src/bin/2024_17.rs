#![cfg_attr(feature = "bench", feature(test))]
use advent_of_code_2024::{Cli, Parser};
use itertools::Itertools;
use std::{fs, ops::BitXor};

fn run_program(a: i64, b: i64, c: i64, program: &[i64]) -> Vec<i64> {
    let mut ip = 0;

    let mut out = vec![];

    let mut a = a;
    let mut b = b;
    let mut c = c;

    loop {
        let inst = program[ip];
        let literal = program[ip + 1];
        let combo = match literal {
            0..=3 => literal,
            4 => a,
            5 => b,
            6 => c,
            _ => i64::MAX,
        };

        match inst {
            0 => {
                assert!(combo != i64::MAX);
                a /= 2_i64.pow(combo as u32);
            }
            1 => {
                b = b.bitxor(literal);
            }
            2 => {
                assert!(combo != i64::MAX);
                b = combo.rem_euclid(8);
            }
            3 => {
                if a != 0 {
                    ip = literal as usize;
                    if ip + 1 >= program.len() {
                        break;
                    }
                    continue;
                }
            }
            4 => {
                b = b.bitxor(c);
            }
            5 => {
                assert!(combo != i64::MAX);
                out.push(combo.rem_euclid(8));
            }
            6 => {
                assert!(combo != i64::MAX);
                b = a / 2_i64.pow(combo as u32);
            }
            7 => {
                assert!(combo != i64::MAX);
                c = a / 2_i64.pow(combo as u32);
            }
            _ => panic!("invalid instruction"),
        }

        ip += 2;
        if ip + 1 >= program.len() {
            break;
        }
    }
    out
}

fn get_a(coeffs: &[i64]) -> i64 {
    coeffs
        .iter()
        .enumerate()
        .map(|(p, n)| 8i64.pow((coeffs.len() - p - 1) as u32) * n)
        .sum()
}

fn part2(nums: &mut Vec<i64>, program: &[i64]) -> Option<i64> {
    if nums.len() == program.len() {
        return Some(get_a(nums));
    }

    for i in 0..8 {
        nums.push(i);

        let output = run_program(get_a(nums), 0, 0, program);

        let length = nums.len();
        if output.ends_with(&program[program.len() - length..]) {
            if let Some(solution) = part2(nums, program) {
                return Some(solution);
            }
        }
        nums.pop();
    }
    None
}

fn calculate(raw_inp: &str) -> (String, i64) {
    let (head, tail) = raw_inp.split_once("\n\n").expect("bad format");

    let (a, b, c) = head
        .lines()
        .filter_map(|line| line.split_once(": ").and_then(|x| x.1.parse::<i64>().ok()))
        .collect_tuple()
        .expect("too many registers");

    let program = tail
        .split_once(": ")
        .expect("bad format")
        .1
        .split(",")
        .filter_map(|x| x.trim().parse().ok())
        .collect::<Vec<i64>>();

    let p1 = run_program(a, b, c, &program).into_iter().join(",");
    let p2 = part2(&mut vec![], &program).expect("no p2 solution?");

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
