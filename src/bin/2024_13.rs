#![cfg_attr(feature = "bench", feature(test))]
use advent_of_code_2024::{Cli, Parser};
use itertools::Itertools;
use std::{fs, str::FromStr};

struct ClawMachine {
    a: (i64, i64),
    b: (i64, i64),
    prize: (i64, i64),
}

impl ClawMachine {
    fn best_cost<const OFFSET: i64>(&self) -> Option<i64> {
        let (tx, ty) = (self.prize.0 + OFFSET, self.prize.1 + OFFSET);
        let (ax, ay) = self.a;
        let (bx, by) = self.b;

        // Simultaneous equations
        // NA * ax + NB * bx = tx
        // NA * ay + NB * by = ty
        let nb = (tx * ay - ax * ty) / (bx * ay - ax * by);
        let na = (ty - nb * by) / ay;

        let actual = (na * ax + nb * bx, na * ay + nb * by);

        (na >= 0 && nb >= 0 && actual == (tx, ty)).then_some(na * 3 + nb)
    }
}

impl FromStr for ClawMachine {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.lines()
            .filter_map(|line| {
                line.split(" ")
                    .skip(1)
                    .map(|c| &c[2..])
                    .map(|c| c.trim_end_matches(","))
                    .filter_map(|c| c.parse().ok())
                    .next_tuple()
            })
            .collect_tuple()
            .map(|(a, b, prize)| ClawMachine { a, b, prize })
            .ok_or(())
    }
}

const P2_OFFSET: i64 = 10000000000000;

fn calculate(raw_inp: &str) -> (i64, i64) {
    raw_inp
        .split("\n\n")
        .filter_map(|group| group.parse().ok())
        .map(|m: ClawMachine| {
            (
                m.best_cost::<0>().unwrap_or(0),
                m.best_cost::<P2_OFFSET>().unwrap_or(0),
            )
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

    const EXAMPLE_DATA: &str = include_str!("../../inputs/examples/2024_13");
    const REAL_DATA: &str = include_str!("../../inputs/real/2024_13");

    const EX1: ClawMachine = ClawMachine {
        a: (94, 34),
        b: (22, 67),
        prize: (8400, 5400),
    };

    const EX2: ClawMachine = ClawMachine {
        a: (26, 66),
        b: (67, 21),
        prize: (12748, 12176),
    };

    const EX3: ClawMachine = ClawMachine {
        a: (17, 86),
        b: (84, 37),
        prize: (7870, 6450),
    };

    const EX4: ClawMachine = ClawMachine {
        a: (69, 23),
        b: (27, 71),
        prize: (18641, 10279),
    };

    #[test]
    fn test_example_p1() {
        assert_eq!(calculate(&EXAMPLE_DATA).0, 480);

        assert_eq!(EX1.best_cost::<0>(), Some(280));
        assert_eq!(EX2.best_cost::<0>(), None);
        assert_eq!(EX3.best_cost::<0>(), Some(200));
        assert_eq!(EX4.best_cost::<0>(), None);
    }

    #[test]
    fn test_example_p2() {
        assert!(EX1.best_cost::<P2_OFFSET>().is_none());
        assert!(EX2.best_cost::<P2_OFFSET>().is_some());
        assert!(EX3.best_cost::<P2_OFFSET>().is_none());
        assert!(EX4.best_cost::<P2_OFFSET>().is_some());
    }

    #[test]
    fn test_real() {
        assert_eq!(calculate(&REAL_DATA), (35729, 88584689879723));
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
