#![cfg_attr(feature = "bench", feature(test))]
use advent_of_code_2024::bitvec_set::BitVecSet2D;
use advent_of_code_2024::{grid_util::make_byte_grid, Cli, Parser};
use ndarray::{indices_of, Array2};
use std::collections::VecDeque;
use std::fs;

const DIRS: [(isize, isize); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

fn perimeter_contribution(grid: &Array2<u8>, pos: (usize, usize)) -> (usize, usize) {
    let mut p1 = 0;
    let mut p2 = 0;

    let this_tile = grid.get(pos);

    let l = grid.get((pos.0, pos.1.wrapping_add_signed(-1))) == this_tile;
    let r = grid.get((pos.0, pos.1.wrapping_add_signed(1))) == this_tile;
    let u = grid.get((pos.0.wrapping_add_signed(-1), pos.1)) == this_tile;
    let d = grid.get((pos.0.wrapping_add_signed(1), pos.1)) == this_tile;

    if !u {
        p1 += 1;
        if !l
            || grid.get((pos.0.wrapping_add_signed(-1), pos.1.wrapping_add_signed(-1))) == this_tile
        {
            p2 += 1;
        }
    }
    if !l {
        p1 += 1;
        if !d
            || grid.get((pos.0.wrapping_add_signed(1), pos.1.wrapping_add_signed(-1))) == this_tile
        {
            p2 += 1;
        }
    }
    if !d {
        p1 += 1;
        if !r || grid.get((pos.0.wrapping_add_signed(1), pos.1.wrapping_add_signed(1))) == this_tile
        {
            p2 += 1;
        }
    }
    if !r {
        p1 += 1;
        if !u
            || grid.get((pos.0.wrapping_add_signed(-1), pos.1.wrapping_add_signed(1))) == this_tile
        {
            p2 += 1;
        }
    }

    (p1, p2)
}

fn get_region_score(
    grid: &Array2<u8>,
    idx: (usize, usize),
    ever_visited: &mut BitVecSet2D,
) -> (usize, usize) {
    let mut q = VecDeque::with_capacity(64);
    q.push_back(idx);

    let mut area = 0;
    let mut p1_perimeter = 0;
    let mut p2_perimeter = 0;

    while let Some(pos) = q.pop_front() {
        if !ever_visited.insert(pos) {
            continue;
        }
        let this_tile = grid[pos];

        let perimeter_contribs = perimeter_contribution(grid, pos);
        p1_perimeter += perimeter_contribs.0;
        p2_perimeter += perimeter_contribs.1;
        area += 1;

        for dir in DIRS {
            let next_pos = (
                pos.0.wrapping_add_signed(dir.0),
                pos.1.wrapping_add_signed(dir.1),
            );

            if let Some(&next_tile) = grid.get(next_pos) {
                if next_tile == this_tile {
                    q.push_back(next_pos);
                }
            }
        }
    }

    (p1_perimeter * area, p2_perimeter * area)
}

fn calculate(raw_inp: &str) -> (usize, usize) {
    let grid = make_byte_grid(raw_inp);
    let mut ever_visited = BitVecSet2D::new(grid.dim());

    indices_of(&grid)
        .into_iter()
        .map(|idx| {
            if ever_visited.contains(&idx) {
                (0, 0)
            } else {
                get_region_score(&grid, idx, &mut ever_visited)
            }
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

    const EXAMPLE_DATA: &str = include_str!("../../inputs/examples/2024_12");
    const REAL_DATA: &str = include_str!("../../inputs/real/2024_12");

    const EXAMPLE_SMALL_1: &str = "AAAA
BBCD
BBCC
EEEC
";

    const EXAMPLE_SMALL_2: &str = "OOOOO
OXOXO
OOOOO
OXOXO
OOOOO
";

    const EXAMPLE_P2_SMALL_3: &str = "EEEEE
EXXXX
EEEEE
EXXXX
EEEEE";

    const EXAMPLE_P2_SMALL_4: &str = "AAAAAA
AAABBA
AAABBA
ABBAAA
ABBAAA
AAAAAA
";

    #[test]
    fn test_example() {
        assert_eq!(calculate(&EXAMPLE_DATA), (1930, 1206));
    }

    #[test]
    fn test_example_small_1() {
        assert_eq!(calculate(&EXAMPLE_SMALL_1), (140, 80));
    }

    #[test]
    fn test_example_small_2() {
        assert_eq!(calculate(&EXAMPLE_SMALL_2), (772, 436));
    }

    #[test]
    fn test_example_p2_small_3() {
        assert_eq!(calculate(&EXAMPLE_P2_SMALL_3).1, 236);
    }

    #[test]
    fn test_example_p2_small_4() {
        assert_eq!(calculate(&EXAMPLE_P2_SMALL_4).1, 368);
    }

    #[test]
    fn test_real() {
        assert_eq!(calculate(&REAL_DATA), (1319878, 784982));
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
