#![cfg_attr(feature = "bench", feature(test))]
use advent_of_code_2024::grid_util::make_byte_grid;
use advent_of_code_2024::{Cli, Parser};
use bitvec::prelude::*;
use ndarray::Array2;
use std::collections::VecDeque;
use std::fs;

const DIRS: [(isize, isize); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

fn reachable(data: &Array2<u8>, start_at: (usize, usize)) -> (usize, usize) {
    let mut q = VecDeque::new();
    q.push_back(start_at);

    let mut ways = Array2::from_elem(data.dim(), 0);
    let mut seen = bitvec![u32, Lsb0; 0; data.dim().0 * data.dim().1];

    ways[start_at] = 1;

    let mut p1 = 0;
    let mut p2 = 0;

    while let Some(pos) = q.pop_front() {
        let this_tile = data[pos];
        if seen
            .get_mut(pos.0 * data.dim().1 + pos.1)
            .unwrap()
            .replace(true)
        {
            continue;
        }

        if this_tile == 9 {
            p1 += 1;
            p2 += ways[pos];
            continue;
        }

        for dir in DIRS {
            let next_pos = (
                pos.0.wrapping_add_signed(dir.0),
                pos.1.wrapping_add_signed(dir.1),
            );

            if let Some(&next_tile) = data.get(next_pos) {
                if next_tile == this_tile + 1 {
                    ways[next_pos] += ways[pos];
                    q.push_back(next_pos);
                }
            }
        }
    }

    (p1, p2)
}

fn calculate(raw_inp: &str) -> (usize, usize) {
    let mut grid = make_byte_grid(raw_inp);

    grid.mapv_inplace(|v| v - b'0');

    grid.indexed_iter()
        .filter(|(_, &v)| v == 0)
        .map(|(idx, _)| reachable(&grid, idx))
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

    const EXAMPLE_DATA: &str = include_str!("../../inputs/examples/2024_10");
    const REAL_DATA: &str = include_str!("../../inputs/real/2024_10");

    const P1_SIMPLE_EX_1: &str = "@@@0@@@
@@@1@@@
@@@2@@@
6543456
7@@@@@7
8@@@@@8
9@@@@@9";

    const P1_SIMPLE_EX_2: &str = "@@90@@9
@@@1@98
@@@2@@7
6543456
765@987
876@@@@
987@@@@";

    const P1_SIMPLE_EX_3: &str = "10@@9@@
2@@@8@@
3@@@7@@
4567654
@@@8@@3
@@@9@@2
@@@@@01";

    const P2_SIMPLE_EX_1: &str = "@@@@@0@
@@4321@
@@5@@2@
@@6543@
@@7@@4@
@@8765@
@@9@@@@";

    const P2_SIMPLE_EX_2: &str = "@@90@@9
@@@1@98
@@@2@@7
6543456
765@987
876@@@@
987@@@@";

    const P2_SIMPLE_EX_3: &str = "012345
123456
234567
345678
4@6789
56789@";

    #[test]
    fn test_example() {
        assert_eq!(calculate(&EXAMPLE_DATA), (36, 81));
    }

    #[test]
    fn test_p1_simple_example_1() {
        assert_eq!(calculate(&P1_SIMPLE_EX_1).0, 2);
    }

    #[test]
    fn test_p1_simple_example_2() {
        assert_eq!(calculate(&P1_SIMPLE_EX_2).0, 4);
    }

    #[test]
    fn test_p1_simple_example_3() {
        assert_eq!(calculate(&P1_SIMPLE_EX_3).0, 3);
    }

    #[test]
    fn test_p2_simple_example_1() {
        assert_eq!(calculate(&P2_SIMPLE_EX_1).1, 3);
    }

    #[test]
    fn test_p2_simple_example_2() {
        assert_eq!(calculate(&P2_SIMPLE_EX_2).1, 13);
    }

    #[test]
    fn test_p2_simple_example_3() {
        assert_eq!(calculate(&P2_SIMPLE_EX_3).1, 227);
    }

    #[test]
    fn test_real() {
        assert_eq!(calculate(&REAL_DATA), (430, 928));
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
