#![cfg_attr(feature = "bench", feature(test))]
use advent_of_code_2024::{Cli, Parser};
use ndarray::Array2;
use std::{collections::VecDeque, fs};

const DIRS: [(isize, isize); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

fn pathfind<const DIM: usize>(grid: &Array2<usize>, n: usize) -> Option<u16> {
    let mut costs = Array2::from_elem((DIM + 1, DIM + 1), u16::MAX);

    let mut q = VecDeque::<(usize, usize)>::with_capacity(64);
    q.push_back((0, 0));
    costs[(0, 0)] = 0;

    while let Some(pos) = q.pop_front() {
        for dir in DIRS {
            let new_pos = (
                pos.0.wrapping_add_signed(dir.0),
                pos.1.wrapping_add_signed(dir.1),
            );
            let new_cost = costs[pos] + 1;

            if let Some(&tile) = grid.get(new_pos) {
                if new_pos == (DIM, DIM) {
                    return Some(new_cost);
                } else if tile >= n && new_cost < costs[new_pos] {
                    q.push_back(new_pos);
                    costs[new_pos] = new_cost;
                }
            }
        }
    }

    None
}

fn calculate<const DIM: usize, const P1_ITER: usize>(raw_inp: &str) -> (u16, &str) {
    let mut grid = Array2::from_elem((DIM + 1, DIM + 1), usize::MAX);

    raw_inp
        .lines()
        .filter_map(|line| {
            let (a, b) = line.split_once(",")?;
            let a = a.parse().ok()?;
            let b = b.parse().ok()?;
            Some((a, b))
        })
        .enumerate()
        .for_each(|(n, (x, y))| {
            grid[(y, x)] = n;
        });

    let n_lines = raw_inp.trim().lines().count();

    let p1 = pathfind::<DIM>(&grid, P1_ITER).expect("no p1 solution?");

    let mut p2_upper = n_lines;
    let mut p2_lower = P1_ITER;

    while p2_lower + 1 != p2_upper {
        let n = (p2_lower + p2_upper) / 2;
        if pathfind::<DIM>(&grid, n).is_some() {
            p2_lower = n;
        } else {
            p2_upper = n;
        }
    }

    let p2 = raw_inp.lines().nth(p2_lower).expect("bad");

    (p1, p2)
}

fn main() {
    let args = Cli::parse();

    let inp = fs::read_to_string(args.input).expect("can't open input file");

    let (p1, p2) = calculate::<70, 1024>(&inp);
    println!("{}\n{}", p1, p2);
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_DATA: &str = include_str!("../../inputs/examples/2024_18");
    const REAL_DATA: &str = include_str!("../../inputs/real/2024_18");

    #[test]
    fn test_example() {
        assert_eq!(calculate::<6, 12>(EXAMPLE_DATA), (22, "6,1"))
    }

    #[test]
    fn test_real() {
        assert_eq!(calculate::<70, 1024>(REAL_DATA), (276, "60,37"));
    }

    #[cfg(feature = "bench")]
    mod benches {
        extern crate test;
        use test::{black_box, Bencher};

        use super::*;

        #[bench]
        fn bench(b: &mut Bencher) {
            b.iter(|| calculate::<70, 1024>(black_box(REAL_DATA)));
        }
    }
}
