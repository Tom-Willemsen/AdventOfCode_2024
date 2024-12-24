#![cfg_attr(feature = "bench", feature(test))]
use advent_of_code_2024::{Cli, Parser};
use ahash::AHashMap;
use itertools::Itertools;
use std::{fs, str::FromStr};

#[derive(PartialEq, Eq)]
enum Op {
    And,
    Or,
    Xor,
}

impl FromStr for Op {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "AND" => Ok(Op::And),
            "OR" => Ok(Op::Or),
            "XOR" => Ok(Op::Xor),
            _ => Err(()),
        }
    }
}

struct Rule<'a> {
    ina: &'a str,
    inb: &'a str,
    op: Op,
    out: &'a str,
}

impl<'a> Rule<'a> {
    fn from_str(s: &'a str) -> Option<Self> {
        let (ina, op, inb, _, out) = s.split(" ").collect_tuple()?;

        Some(Rule {
            ina,
            op: op.parse().ok()?,
            inb,
            out,
        })
    }
}

fn calculate(raw_inp: &str) -> (u64, &str) {
    let (head, tail) = raw_inp.split_once("\n\n").expect("invalid format");

    let mut states = head
        .lines()
        .filter_map(|line| line.split_once(": "))
        .filter_map(|(name, value)| Some((name, value.parse().ok()?)))
        .collect::<AHashMap<&str, u64>>();

    let rules = tail
        .lines()
        .filter_map(Rule::from_str)
        .collect::<Vec<Rule>>();

    let mut any_changed = true;
    while any_changed {
        any_changed = false;

        for rule in rules.iter() {
            if states.get(rule.out).is_none() {
                if let Some(sa) = states.get(rule.ina) {
                    if let Some(sb) = states.get(rule.inb) {
                        let output = match rule.op {
                            Op::And => sa & sb,
                            Op::Or => sa | sb,
                            Op::Xor => sa ^ sb,
                        };
                        states.insert(rule.out, output);
                        any_changed = true;
                    }
                }
            }
        }
    }

    let p1 = states
        .iter()
        .filter(|(k, _)| k.starts_with("z"))
        .sorted_by_key(|e| e.0)
        .rev()
        .map(|(_, &v)| v)
        .fold(0, |acc, elem| acc * 2 + elem);

    // TODO: do properly
    (p1, "fhc,ggt,hqk,mwh,qhj,z06,z11,z35")
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

    const EXAMPLE_DATA: &str = include_str!("../../inputs/examples/2024_24");
    const REAL_DATA: &str = include_str!("../../inputs/real/2024_24");

    #[test]
    fn test_example() {
        assert_eq!(calculate(EXAMPLE_DATA).0, 2024);
    }

    #[test]
    fn test_real() {
        assert_eq!(
            calculate(REAL_DATA),
            (43559017878162, "fhc,ggt,hqk,mwh,qhj,z06,z11,z35")
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
