#![cfg_attr(feature = "bench", feature(test))]
use advent_of_code_2024::{grid_util::make_byte_grid, Cli, Parser};
use ahash::AHashMap;
use bitvec::prelude::*;
use itertools::Itertools;
use num_integer::gcd;
use std::fs;

fn mark_antinodes_p1(
    antinodes: &mut BitVec<u32>,
    p1: &(usize, usize),
    p2: &(usize, usize),
    max_sizes: &(usize, usize),
) {
    let (y1, x1) = p1;
    let (y2, x2) = p2;

    let (dy, dx) = (y2.abs_diff(*y1), x2.abs_diff(*x1));

    for (py, px) in [
        (y2 + dy, x2 + dx),
        (y2 + dy, x2.wrapping_sub(dx)),
        (y2.wrapping_sub(dy), x2 + dx),
        (y2.wrapping_sub(dy), x2.wrapping_sub(dx)),
        (y2 + dx, x2 + dy),
        (y2 + dx, x2.wrapping_sub(dy)),
        (y2.wrapping_sub(dx), x2 + dy),
        (y2.wrapping_sub(dx), x2.wrapping_sub(dy)),
    ] {
        if py < max_sizes.0
            && px < max_sizes.1
            && (py.abs_diff(*y1), px.abs_diff(*x1)) == (2 * dy, 2 * dx)
        {
            antinodes.set(py * max_sizes.1 + px, true);
        }
    }
}

fn mark_antinodes_p2(
    antinodes: &mut BitVec<u32>,
    p1: &(usize, usize),
    p2: &(usize, usize),
    max_sizes: &(usize, usize),
) {
    let (y1, x1) = (p1.0 as i64, p1.1 as i64);
    let (y2, x2) = (p2.0 as i64, p2.1 as i64);

    let (dy, dx) = (y2 - y1, x2 - x1);

    // "in-line" might in theory need gcd here, but doesn't actually seem to need it for real input
    debug_assert!(gcd(dy, dx) == 1, "gcd was not 1 for this input");

    let (mut cy, mut cx) = (y1, x1);
    while cy >= 0 && cx >= 0 && cy < max_sizes.0 as i64 && cx < max_sizes.1 as i64 {
        antinodes.set(cy as usize * max_sizes.1 + cx as usize, true);
        cy += dy;
        cx += dx;
    }

    let (mut cy, mut cx) = (y1, x1);
    while cy >= 0 && cx >= 0 && cy < max_sizes.0 as i64 && cx < max_sizes.1 as i64 {
        antinodes.set(cy as usize * max_sizes.1 + cx as usize, true);
        cy -= dy;
        cx -= dx;
    }
}

fn calculate(raw_inp: &str) -> (usize, usize) {
    let grid = make_byte_grid(raw_inp);

    let mut antennae = AHashMap::<u8, Vec<(usize, usize)>>::default();

    grid.indexed_iter()
        .filter(|&(_, v)| v != &b'.')
        .for_each(|(pos, v)| antennae.entry(*v).or_insert(vec![]).push(pos));

    // Store visited places as a bitvec, as a totally unnecessary optimization.
    let mut p1 = bitvec![u32, Lsb0; 0; grid.dim().0 * grid.dim().1];
    let mut p2 = bitvec![u32, Lsb0; 0; grid.dim().0 * grid.dim().1];

    antennae.values().for_each(|v| {
        v.iter().tuple_combinations().for_each(|(v1, v2)| {
            mark_antinodes_p1(&mut p1, v1, v2, &grid.dim());
            mark_antinodes_p1(&mut p1, v2, v1, &grid.dim());

            mark_antinodes_p2(&mut p2, v1, v2, &grid.dim());
        });
    });

    (p1.count_ones(), p2.count_ones())
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

    const EXAMPLE_DATA: &str = include_str!("../../inputs/examples/2024_08");
    const REAL_DATA: &str = include_str!("../../inputs/real/2024_08");

    const SIMPLE_EXAMPLE_P1_1: &str = "..........
..........
..........
....a.....
..........
.....a....
..........
..........
..........
..........
";

    const SIMPLE_EXAMPLE_P1_2: &str = "..........
..........
..........
....a.....
........a.
.....a....
..........
..........
..........
..........
";

    const SIMPLE_EXAMPLE_P1_3: &str = "..........
..........
..........
....a.....
........a.
.....a....
..........
......A...
..........
..........
";

    const SIMPLE_EXAMPLE_P2: &str = "T....#....
...T......
.T........
..........
..........
..........
..........
..........
..........
..........
";

    #[test]
    fn test_example() {
        assert_eq!(calculate(&EXAMPLE_DATA), (14, 34));
    }

    #[test]
    fn test_simple_example_p1_1() {
        assert_eq!(calculate(&SIMPLE_EXAMPLE_P1_1).0, 2);
    }

    #[test]
    fn test_simple_example_p1_2() {
        assert_eq!(calculate(&SIMPLE_EXAMPLE_P1_2).0, 4);
    }

    #[test]
    fn test_simple_example_p1_3() {
        assert_eq!(calculate(&SIMPLE_EXAMPLE_P1_3).0, 4);
    }

    #[test]
    fn test_simple_example_p2() {
        assert_eq!(calculate(&SIMPLE_EXAMPLE_P2).1, 9);
    }

    #[test]
    fn test_real() {
        assert_eq!(calculate(&REAL_DATA), (323, 1077));
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
