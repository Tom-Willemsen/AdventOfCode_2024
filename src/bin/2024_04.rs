#![cfg_attr(feature = "bench", feature(test))]
use advent_of_code_2024::{grid_util::make_byte_grid, Cli, Parser};
use ndarray::{indices_of, Array2};
use std::fs;

fn calculate_p1(grid: &Array2<u8>) -> usize {
    indices_of(grid)
        .into_iter()
        .map(|(r, c)| {
            let mut count = 0;
            for s in [b"XMAS", b"SAMX"] {
                // Right
                if (0..=3).all(|i| grid.get((r, c + i)) == Some(&s[i])) {
                    count += 1;
                }
                // Down
                if (0..=3).all(|i| grid.get((r + i, c)) == Some(&s[i])) {
                    count += 1;
                }
                // Diagonal down-right
                if (0..=3).all(|i| grid.get((r + i, c + i)) == Some(&s[i])) {
                    count += 1;
                }
                // Diagonal down-left
                if (0..=3).all(|i| grid.get((r + i, c.wrapping_sub(i))) == Some(&s[i])) {
                    count += 1;
                }
            }
            count
        })
        .sum()
}

fn calculate_p2(grid: &Array2<u8>) -> usize {
    grid.indexed_iter()
        .filter(|(_, &val)| val == b'A')
        .filter(|((r, c), _)| {
            [b"MSMS", b"MSSM", b"SMMS", b"SMSM"].iter().any(|&s| {
                grid.get((r.wrapping_sub(1), c.wrapping_sub(1))) == Some(&s[0])
                    && grid.get((r + 1, c + 1)) == Some(&s[1])
                    && grid.get((r + 1, c.wrapping_sub(1))) == Some(&s[2])
                    && grid.get((r.wrapping_sub(1), c + 1)) == Some(&s[3])
            })
        })
        .count()
}

fn calculate(raw_inp: &str) -> (usize, usize) {
    let grid = make_byte_grid(raw_inp);

    (calculate_p1(&grid), calculate_p2(&grid))
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

    const EXAMPLE_DATA: &str = include_str!("../../inputs/examples/2024_04");
    const REAL_DATA: &str = include_str!("../../inputs/real/2024_04");

    #[test]
    fn test_example() {
        assert_eq!(calculate(&EXAMPLE_DATA), (18, 9));
    }

    #[test]
    fn test_real() {
        assert_eq!(calculate(&REAL_DATA), (2427, 1900));
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
