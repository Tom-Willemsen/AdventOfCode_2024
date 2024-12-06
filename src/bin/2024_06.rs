#![cfg_attr(feature = "bench", feature(test))]
use advent_of_code_2024::{grid_util::make_byte_grid, Cli, Parser};
use ahash::AHashSet;
use bitvec::prelude::*;
use ndarray::Array2;
use rayon::prelude::*;
use std::fs;

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn forward(self, pos: (usize, usize)) -> (usize, usize) {
        match self {
            Direction::Up => (pos.0.wrapping_sub(1), pos.1),
            Direction::Right => (pos.0, pos.1 + 1),
            Direction::Down => (pos.0 + 1, pos.1),
            Direction::Left => (pos.0, pos.1.wrapping_sub(1)),
        }
    }

    fn turn_right(self) -> Direction {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }
}

fn next_pos(
    grid: &Array2<u8>,
    pos: (usize, usize),
    dir: Direction,
    extra_obstacle: Option<(usize, usize)>,
) -> ((usize, usize), Direction) {
    let want = dir.forward(pos);
    if Some(want) != extra_obstacle && grid.get(want).unwrap_or(&b'.') != &b'#' {
        (want, dir)
    } else {
        (pos, dir.turn_right())
    }
}

fn does_loop(grid: &Array2<u8>, start_pos: (usize, usize), extra_obstacle: (usize, usize)) -> bool {
    // Store visited places as a bitvec, this is ~5x faster than a HashSet.
    let mut visited = bitvec![u32, Lsb0; 0; grid.dim().0 * grid.dim().1 * 4];

    let mut pos = start_pos;
    let mut dir: Direction = Direction::Up;

    while (0..grid.dim().0).contains(&pos.0) && (0..grid.dim().1).contains(&pos.1) {
        let mut v = visited
            .get_mut(pos.0 * 4 * grid.dim().1 + pos.1 * 4 + dir as usize)
            .expect("invalid bitvec index");

        if *v {
            return true;
        }
        *v = true;
        (pos, dir) = next_pos(grid, pos, dir, Some(extra_obstacle));
    }
    false
}

fn calculate(raw_inp: &str) -> (usize, usize) {
    let grid = make_byte_grid(raw_inp);

    let start_pos = grid
        .indexed_iter()
        .find(|(_, &val)| val == b'^')
        .map(|(pos, _)| pos)
        .expect("no start position?");

    let mut visited = AHashSet::<(usize, usize)>::default();

    let mut pos = start_pos;
    let mut dir = Direction::Up;
    while (0..grid.dim().0).contains(&pos.0) && (0..grid.dim().1).contains(&pos.1) {
        visited.insert(pos);
        (pos, dir) = next_pos(&grid, pos, dir, None);
    }

    let p1 = visited.len();

    let p2 = visited
        .par_iter()
        .filter(|&pos| pos != &start_pos)
        .filter(|&obstacle| does_loop(&grid, start_pos, *obstacle))
        .count();

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

    const EXAMPLE_DATA: &str = include_str!("../../inputs/examples/2024_06");
    const REAL_DATA: &str = include_str!("../../inputs/real/2024_06");

    #[test]
    fn test_example() {
        assert_eq!(calculate(&EXAMPLE_DATA), (41, 6));
    }

    #[test]
    fn test_real() {
        assert_eq!(calculate(&REAL_DATA), (5067, 1793));
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
