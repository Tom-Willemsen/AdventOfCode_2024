#![cfg_attr(feature = "bench", feature(test))]
use advent_of_code_2024::{Cli, Parser};
use bitvec::prelude::*;
use std::fs;
use std::str::FromStr;

#[derive(Debug)]
struct Robot {
    px: i32,
    py: i32,
    vx: i32,
    vy: i32,
}

impl Robot {
    fn position_after<const DIM_X: i32, const DIM_Y: i32>(&self, n: i32) -> (i32, i32) {
        (
            (self.px + n * self.vx).rem_euclid(DIM_X),
            (self.py + n * self.vy).rem_euclid(DIM_Y),
        )
    }

    fn forward_n_inplace<const DIM_X: i32, const DIM_Y: i32>(&mut self, n: i32) {
        let new_pos = self.position_after::<DIM_X, DIM_Y>(n);
        self.px = new_pos.0;
        self.py = new_pos.1;
    }

    fn reverse_one_inplace<const DIM_X: i32, const DIM_Y: i32>(&mut self) {
        // Try to encourage compiler to generate branchless code, and avoid
        // slow rem_euclid
        self.px -= self.vx;
        if self.px < 0 {
            self.px += DIM_X;
        }
        if self.px >= DIM_X {
            self.px -= DIM_X;
        }
        self.py -= self.vy;
        if self.py < 0 {
            self.py += DIM_Y;
        }
        if self.py >= DIM_Y {
            self.py -= DIM_Y;
        }
    }
}

impl FromStr for Robot {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (p, v) = s.split_once(" ").ok_or(())?;
        let (_, p) = p.split_once("=").ok_or(())?;
        let (_, v) = v.split_once("=").ok_or(())?;

        let (px, py) = p.split_once(",").ok_or(())?;
        let (vx, vy) = v.split_once(",").ok_or(())?;

        Ok(Robot {
            px: px.parse().map_err(|_| ())?,
            py: py.parse().map_err(|_| ())?,
            vx: vx.parse().map_err(|_| ())?,
            vy: vy.parse().map_err(|_| ())?,
        })
    }
}

fn calculate_p1<const DIM_X: i32, const DIM_Y: i32>(robots: &[Robot]) -> i32 {
    let (mut ul, mut ur, mut dl, mut dr) = (0, 0, 0, 0);

    robots
        .iter()
        .map(|r| r.position_after::<DIM_X, DIM_Y>(100))
        .for_each(|(x, y)| {
            let left = x < DIM_X / 2;
            let right = x > DIM_X / 2;
            let up = y < DIM_Y / 2;
            let down = y > DIM_Y / 2;

            match (left, up, right, down) {
                (true, true, false, false) => ul += 1,
                (true, false, false, true) => dl += 1,
                (false, true, true, false) => ur += 1,
                (false, false, true, true) => dr += 1,
                _ => {}
            }
        });

    ul * ur * dl * dr
}

fn calculate_p2<const DIM_X: i32, const DIM_Y: i32>(robots: &mut [Robot]) -> i32 {
    robots
        .iter_mut()
        .for_each(|r| r.forward_n_inplace::<DIM_X, DIM_Y>(DIM_X * DIM_Y));

    let mut n = DIM_X * DIM_Y;
    let mut bv = bitvec![u8, Lsb0; 0; (DIM_X * DIM_Y) as usize];

    while n > 0 {
        robots.iter_mut().for_each(|r| {
            r.reverse_one_inplace::<DIM_X, DIM_Y>();
            bv.set((r.px * DIM_Y + r.py) as usize, true);
        });

        // u wot m8
        if bv.as_raw_slice().contains(&0xFF) {
            return n - 1;
        }

        n -= 1;
        bv.fill(false);
    }

    panic!("no solution");
}

fn parse(raw_inp: &str) -> Vec<Robot> {
    raw_inp
        .lines()
        .filter_map(|line| line.parse().ok())
        .collect()
}

fn calculate<const DIM_X: i32, const DIM_Y: i32>(raw_inp: &str) -> (i32, i32) {
    let mut robots = parse(raw_inp);

    let p1 = calculate_p1::<DIM_X, DIM_Y>(&robots);
    let p2 = calculate_p2::<DIM_X, DIM_Y>(&mut robots);

    (p1, p2)
}

fn main() {
    let args = Cli::parse();

    let inp = fs::read_to_string(args.input).expect("can't open input file");

    let (p1, p2) = calculate::<101, 103>(&inp);
    println!("{}\n{}", p1, p2);
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_DATA: &str = include_str!("../../inputs/examples/2024_14");
    const REAL_DATA: &str = include_str!("../../inputs/real/2024_14");

    #[test]
    fn test_example() {
        assert_eq!(calculate_p1::<11, 7>(&parse(&EXAMPLE_DATA)), 12);
    }

    #[test]
    fn test_real() {
        assert_eq!(calculate::<101, 103>(&REAL_DATA), (228421332, 7790));
    }

    #[cfg(feature = "bench")]
    mod benches {
        extern crate test;
        use test::{black_box, Bencher};

        use super::*;

        #[bench]
        fn bench(b: &mut Bencher) {
            b.iter(|| calculate::<101, 103>(black_box(REAL_DATA)));
        }
    }
}
