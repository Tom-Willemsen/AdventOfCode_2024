#![cfg_attr(feature = "bench", feature(test))]
use advent_of_code_2024::{Cli, Parser};
use ahash::AHashMap;
use std::fs;

fn score_middle(pages: &[i32]) -> i32 {
    pages[pages.len() / 2]
}

fn scores_of_line(line: &str, rules: &AHashMap<i32, Vec<i32>>) -> (i32, i32) {
    let mut items = line
        .split(",")
        .filter_map(|val| val.parse().ok())
        .collect::<Vec<i32>>();

    let mut pages = Vec::with_capacity(items.len());
    let mut was_valid_p1 = true;

    while !items.is_empty() {
        let item = *items
            .iter()
            .find(|&&i| {
                rules
                    .get(&i)
                    .unwrap_or(&Vec::<i32>::default())
                    .iter()
                    .filter(|dep| items.contains(dep))
                    .all(|dep| pages.contains(dep))
            })
            .expect("impossible dependencies");

        if item != items[0] {
            was_valid_p1 = false;
        }

        pages.push(item);
        items.retain(|i| i != &item);
    }

    if was_valid_p1 {
        (score_middle(&pages), 0)
    } else {
        (0, score_middle(&pages))
    }
}

fn calculate(raw_inp: &str) -> (i32, i32) {
    let (rules, pages) = raw_inp.split_once("\n\n").expect("invalid file format");

    let mut rulesmap = AHashMap::<i32, Vec<i32>>::default();

    rules
        .lines()
        .filter_map(|line| line.split_once("|"))
        .map(|(x, y)| (x.parse().expect("NaN"), y.parse().expect("NaN")))
        .for_each(|(x, y)| {
            rulesmap.entry(y).or_insert(vec![]).push(x);
        });

    pages
        .lines()
        .map(|line| scores_of_line(line, &rulesmap))
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

    const EXAMPLE_DATA: &str = include_str!("../../inputs/examples/2024_05");
    const REAL_DATA: &str = include_str!("../../inputs/real/2024_05");

    #[test]
    fn test_example() {
        assert_eq!(calculate(&EXAMPLE_DATA), (143, 123));
    }

    #[test]
    fn test_real() {
        assert_eq!(calculate(&REAL_DATA), (6612, 4944));
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
