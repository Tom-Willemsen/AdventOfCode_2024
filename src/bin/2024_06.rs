#![cfg_attr(feature = "bench", feature(test))]
use advent_of_code_2024::{grid_util::make_byte_grid, Cli, Parser};
use ahash::{AHashMap, AHashSet};
use bitvec::prelude::*;
use ndarray::Array2;
use rayon::prelude::*;
use std::fs;

type JumpMap = AHashMap<(usize, usize, Direction), (usize, usize, Direction)>;

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
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
    jump_map: &JumpMap,
) -> ((usize, usize), Direction) {
    if extra_obstacle.map(|f| f.0) != Some(pos.0) && extra_obstacle.map(|f| f.1) != Some(pos.1) {
        if let Some((y, x, d)) = jump_map.get(&(pos.0, pos.1, dir)) {
            return ((*y, *x), *d);
        }
    }

    let want = dir.forward(pos);
    if Some(want) != extra_obstacle && grid.get(want).unwrap_or(&b'.') != &b'#' {
        (want, dir)
    } else {
        (pos, dir.turn_right())
    }
}

fn does_loop(
    grid: &Array2<u8>,
    start_pos: (usize, usize),
    extra_obstacle: (usize, usize),
    jump_map: &JumpMap,
) -> bool {
    // Store visited places as a bitvec, this is ~5x faster than a HashSet.
    let mut visited = bitvec![u32, Lsb0; 0; grid.dim().0 * grid.dim().1 * 4];

    let mut pos = start_pos;
    let mut dir: Direction = Direction::Up;

    let dim_y = grid.dim().0;
    let dim_x = grid.dim().1;

    while pos.0 < dim_y && pos.1 < dim_x {
        if visited
            .get_mut(pos.0 * 4 * dim_x + pos.1 * 4 + dir as usize)
            .expect("invalid bitvec index")
            .replace(true)
        {
            return true;
        }
        (pos, dir) = next_pos(grid, pos, dir, Some(extra_obstacle), jump_map);
    }
    false
}

fn make_jump_map(grid: &Array2<u8>) -> JumpMap {
    grid.indexed_iter()
        .filter(|(_, &v)| v == b'#')
        .flat_map(|(pos, _)| {
            [
                (pos.0.wrapping_sub(1), pos.1, Direction::Down),
                (pos.0 + 1, pos.1, Direction::Up),
                (pos.0, pos.1.wrapping_sub(1), Direction::Right),
                (pos.0, pos.1 + 1, Direction::Left),
            ]
        })
        .filter(|(y, x, _)| y < &grid.dim().0 && x < &grid.dim().1)
        .map(|(y, x, d)| {
            let nd = d.turn_right();
            let mut np = (y, x);

            while grid.get(nd.forward(np)) == Some(&b'.') {
                np = nd.forward(np);
            }
            ((y, x, d), (np.0, np.1, nd))
        })
        .collect()
}

fn calculate(raw_inp: &str) -> (usize, usize) {
    let mut grid = make_byte_grid(raw_inp);

    let start_pos = grid
        .indexed_iter()
        .find(|(_, &val)| val == b'^')
        .map(|(pos, _)| pos)
        .expect("no start position?");

    grid[start_pos] = b'.';

    let mut visited = AHashSet::<(usize, usize)>::default();

    let mut pos = start_pos;
    let mut dir = Direction::Up;

    let dim_y = grid.dim().0;
    let dim_x = grid.dim().1;

    while pos.0 < dim_y && pos.1 < dim_x {
        visited.insert(pos);
        (pos, dir) = next_pos(&grid, pos, dir, None, &AHashMap::default());
    }

    let p1 = visited.len();

    let jump_map = make_jump_map(&grid);

    let p2 = visited
        .par_iter()
        .filter(|&pos| pos != &start_pos)
        .filter(|&obstacle| does_loop(&grid, start_pos, *obstacle, &jump_map))
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
