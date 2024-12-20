#![cfg_attr(feature = "bench", feature(test))]
use advent_of_code_2024::{grid_util::make_byte_grid, Cli, Parser};
use ndarray::Array2;
use rayon::prelude::*;
use std::{collections::VecDeque, fs};

const DIRS: [(isize, isize); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

// Assumes the path never branches
fn get_path_and_costs(
    grid: &Array2<u8>,
    start: (usize, usize),
) -> (Vec<(usize, usize)>, Array2<u32>) {
    let mut costs = Array2::from_elem(grid.dim(), u32::MAX);
    let mut path = vec![];
    let mut q = VecDeque::default();

    q.push_back((start.0, start.1));
    costs[(start.0, start.1)] = 0;
    path.push(start);

    while let Some(pos) = q.pop_front() {
        for dir in DIRS {
            let new_pos = (
                pos.0.wrapping_add_signed(dir.0),
                pos.1.wrapping_add_signed(dir.1),
            );
            let new_cost = costs[pos] + 1;

            if let Some(&tile) = grid.get((new_pos.0, new_pos.1)) {
                if (tile == b'.') && new_cost < costs[new_pos] {
                    q.push_back(new_pos);
                    costs[new_pos] = new_cost;
                    path.push(new_pos);
                    break;
                }
            }
        }
    }

    (path, costs)
}

fn get_cheats(pos: (usize, usize), costs: &Array2<u32>, n: u32, good_cheat_savings: u32) -> u32 {
    let (yp, xp) = pos;
    let ys = yp.saturating_sub(n as usize);
    let ye = (costs.dim().0 - 1).min(yp + n as usize);

    let cp = costs[(yp, xp)];
    debug_assert!(cp != u32::MAX);

    let mut ans = 0;
    for y in ys..=ye {
        let yd = y.abs_diff(yp);
        let xs = xp.saturating_sub(n as usize - yd);
        let xe = (costs.dim().1 - 1).min(xp + n as usize - yd);

        for x in xs..=xe {
            let cc = costs[(y, x)];
            if cc == u32::MAX {
                continue;
            }
            let diff = (x.abs_diff(xp) + y.abs_diff(yp)) as u32;
            if cc >= cp + diff + good_cheat_savings {
                ans += 1;
            }
        }
    }

    ans
}

fn enumerate_cheats(
    path: Vec<(usize, usize)>,
    costs: &Array2<u32>,
    good_cheat_savings: u32,
) -> (u32, u32) {
    path.into_par_iter()
        .map(|start| {
            (
                get_cheats(start, costs, 2, good_cheat_savings),
                get_cheats(start, costs, 20, good_cheat_savings),
            )
        })
        .reduce(|| (0, 0), |a, b| (a.0 + b.0, a.1 + b.1))
}

fn get_pos_of(grid: &Array2<u8>, needle: u8) -> (usize, usize) {
    grid.indexed_iter()
        .find(|(_, &v)| v == needle)
        .map(|(pos, _)| pos)
        .expect("can't find needle")
}

fn calculate(raw_inp: &str, good_cheat_savings: u32) -> (u32, u32) {
    let mut grid = make_byte_grid(raw_inp);

    let start = get_pos_of(&grid, b'S');
    let end = get_pos_of(&grid, b'E');

    grid[start] = b'.';
    grid[end] = b'.';

    let (path, costs) = get_path_and_costs(&grid, start);

    enumerate_cheats(path, &costs, good_cheat_savings)
}

fn main() {
    let args = Cli::parse();

    let inp = fs::read_to_string(args.input).expect("can't open input file");

    let (p1, p2) = calculate(&inp, 100);
    println!("{}\n{}", p1, p2);
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_DATA: &str = include_str!("../../inputs/examples/2024_20");
    const REAL_DATA: &str = include_str!("../../inputs/real/2024_20");

    #[test]
    fn test_example_p1() {
        assert_eq!(calculate(EXAMPLE_DATA, 2).0, 44);
        assert_eq!(calculate(EXAMPLE_DATA, 4).0, 30);
        assert_eq!(calculate(EXAMPLE_DATA, 6).0, 16);
        assert_eq!(calculate(EXAMPLE_DATA, 8).0, 14);
        assert_eq!(calculate(EXAMPLE_DATA, 10).0, 10);
        assert_eq!(calculate(EXAMPLE_DATA, 12).0, 8);
        assert_eq!(calculate(EXAMPLE_DATA, 20).0, 5);
        assert_eq!(calculate(EXAMPLE_DATA, 36).0, 4);
        assert_eq!(calculate(EXAMPLE_DATA, 38).0, 3);
        assert_eq!(calculate(EXAMPLE_DATA, 40).0, 2);
        assert_eq!(calculate(EXAMPLE_DATA, 64).0, 1);
    }

    #[test]
    fn test_example_p2() {
        assert_eq!(calculate(EXAMPLE_DATA, 50).1, 285);
        assert_eq!(calculate(EXAMPLE_DATA, 52).1, 253);
        assert_eq!(calculate(EXAMPLE_DATA, 54).1, 222);
        assert_eq!(calculate(EXAMPLE_DATA, 56).1, 193);
        assert_eq!(calculate(EXAMPLE_DATA, 58).1, 154);
        assert_eq!(calculate(EXAMPLE_DATA, 60).1, 129);
        assert_eq!(calculate(EXAMPLE_DATA, 62).1, 106);
        assert_eq!(calculate(EXAMPLE_DATA, 64).1, 86);
        assert_eq!(calculate(EXAMPLE_DATA, 66).1, 67);
        assert_eq!(calculate(EXAMPLE_DATA, 68).1, 55);
        assert_eq!(calculate(EXAMPLE_DATA, 70).1, 41);
        assert_eq!(calculate(EXAMPLE_DATA, 72).1, 29);
        assert_eq!(calculate(EXAMPLE_DATA, 74).1, 7);
        assert_eq!(calculate(EXAMPLE_DATA, 76).1, 3);
    }

    #[test]
    fn test_real() {
        assert_eq!(calculate(REAL_DATA, 100), (1463, 985332));
    }

    #[cfg(feature = "bench")]
    mod benches {
        extern crate test;
        use test::{black_box, Bencher};

        use super::*;

        #[bench]
        fn bench(b: &mut Bencher) {
            b.iter(|| calculate(black_box(REAL_DATA, 100)));
        }
    }
}
