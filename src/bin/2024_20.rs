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

fn get_cheats<const N: usize, const SAVINGS: u32>(pos: (usize, usize), costs: &Array2<u32>) -> u32 {
    let (yp, xp) = pos;
    let ys = yp.saturating_sub(N);
    let ye = (costs.dim().0 - 1).min(yp + N);

    let cp = costs[(yp, xp)];
    debug_assert!(cp != u32::MAX);

    let mut ans = 0;
    for y in ys..ye + 1 {
        let yd = y.abs_diff(yp);
        let xs = xp.saturating_sub(N - yd);
        let xe = (costs.dim().1 - 1).min(xp + N - yd);

        for x in xs..xe + 1 {
            let cc = costs[(y, x)];
            let diff = (x.abs_diff(xp) + yd) as u32;
            ans += (cc != u32::MAX && cc >= cp + diff + SAVINGS) as u32;
        }
    }

    ans
}

fn enumerate_cheats<const SAVINGS: u32>(
    path: Vec<(usize, usize)>,
    costs: &Array2<u32>,
) -> (u32, u32) {
    let bound = path.len() - SAVINGS as usize;
    path[..bound]
        .into_par_iter()
        .map(|start| {
            (
                get_cheats::<2, SAVINGS>(*start, costs),
                get_cheats::<20, SAVINGS>(*start, costs),
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

fn calculate<const SAVINGS: u32>(raw_inp: &str) -> (u32, u32) {
    let mut grid = make_byte_grid(raw_inp);

    let start = get_pos_of(&grid, b'S');
    let end = get_pos_of(&grid, b'E');

    grid[start] = b'.';
    grid[end] = b'.';

    let (path, costs) = get_path_and_costs(&grid, start);

    enumerate_cheats::<SAVINGS>(path, &costs)
}

fn main() {
    let cpus: usize = std::thread::available_parallelism().unwrap().into();
    let threads = (cpus / 2).max(1);
    rayon::ThreadPoolBuilder::new()
        .num_threads(threads)
        .build_global()
        .unwrap();

    let args = Cli::parse();

    let inp = fs::read_to_string(args.input).expect("can't open input file");

    let (p1, p2) = calculate::<100>(&inp);
    println!("{}\n{}", p1, p2);
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_DATA: &str = include_str!("../../inputs/examples/2024_20");
    const REAL_DATA: &str = include_str!("../../inputs/real/2024_20");

    #[test]
    fn test_example_p1() {
        assert_eq!(calculate::<2>(EXAMPLE_DATA).0, 44);
        assert_eq!(calculate::<4>(EXAMPLE_DATA).0, 30);
        assert_eq!(calculate::<6>(EXAMPLE_DATA).0, 16);
        assert_eq!(calculate::<8>(EXAMPLE_DATA).0, 14);
        assert_eq!(calculate::<10>(EXAMPLE_DATA).0, 10);
        assert_eq!(calculate::<12>(EXAMPLE_DATA).0, 8);
        assert_eq!(calculate::<20>(EXAMPLE_DATA).0, 5);
        assert_eq!(calculate::<36>(EXAMPLE_DATA).0, 4);
        assert_eq!(calculate::<38>(EXAMPLE_DATA).0, 3);
        assert_eq!(calculate::<40>(EXAMPLE_DATA).0, 2);
        assert_eq!(calculate::<64>(EXAMPLE_DATA).0, 1);
    }

    #[test]
    fn test_example_p2() {
        assert_eq!(calculate::<50>(EXAMPLE_DATA).1, 285);
        assert_eq!(calculate::<52>(EXAMPLE_DATA).1, 253);
        assert_eq!(calculate::<54>(EXAMPLE_DATA).1, 222);
        assert_eq!(calculate::<56>(EXAMPLE_DATA).1, 193);
        assert_eq!(calculate::<58>(EXAMPLE_DATA).1, 154);
        assert_eq!(calculate::<60>(EXAMPLE_DATA).1, 129);
        assert_eq!(calculate::<62>(EXAMPLE_DATA).1, 106);
        assert_eq!(calculate::<64>(EXAMPLE_DATA).1, 86);
        assert_eq!(calculate::<66>(EXAMPLE_DATA).1, 67);
        assert_eq!(calculate::<68>(EXAMPLE_DATA).1, 55);
        assert_eq!(calculate::<70>(EXAMPLE_DATA).1, 41);
        assert_eq!(calculate::<72>(EXAMPLE_DATA).1, 29);
        assert_eq!(calculate::<74>(EXAMPLE_DATA).1, 7);
        assert_eq!(calculate::<76>(EXAMPLE_DATA).1, 3);
    }

    #[test]
    fn test_real() {
        assert_eq!(calculate::<100>(REAL_DATA), (1463, 985332));
    }

    #[cfg(feature = "bench")]
    mod benches {
        extern crate test;
        use test::{black_box, Bencher};

        use super::*;

        #[bench]
        fn bench(b: &mut Bencher) {
            b.iter(|| calculate::<100>(black_box(REAL_DATA)));
        }
    }
}
