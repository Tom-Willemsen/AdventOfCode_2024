#![cfg_attr(feature = "bench", feature(test))]
use advent_of_code_2024::bitvec_set::BitVecSet2D;
use advent_of_code_2024::{grid_util::make_byte_grid, Cli, Parser};
use ndarray::{s, Array2, Array3};
use std::collections::VecDeque;
use std::{fs, i32};

#[derive(Copy, Clone)]
enum Dir {
    RIGHT,
    UP,
    LEFT,
    DOWN,
}

impl Dir {
    fn raw(&self) -> (isize, isize) {
        match self {
            Dir::RIGHT => (0, 1),
            Dir::LEFT => (0, -1),
            Dir::UP => (-1, 0),
            Dir::DOWN => (1, 0),
        }
    }

    fn turn_right(&self) -> Dir {
        match self {
            Dir::RIGHT => Dir::DOWN,
            Dir::DOWN => Dir::LEFT,
            Dir::LEFT => Dir::UP,
            Dir::UP => Dir::RIGHT,
        }
    }

    fn turn_left(&self) -> Dir {
        match self {
            Dir::RIGHT => Dir::UP,
            Dir::UP => Dir::LEFT,
            Dir::LEFT => Dir::DOWN,
            Dir::DOWN => Dir::RIGHT,
        }
    }
}

struct PosAndDir {
    py: usize,
    px: usize,
    dir: Dir,
}

impl PosAndDir {
    fn step(&self, forward: bool) -> PosAndDir {
        let raw_dir = self.dir.raw();
        let m = if forward { 1 } else { -1 };
        PosAndDir {
            py: self.py.wrapping_add_signed(raw_dir.0 * m),
            px: self.px.wrapping_add_signed(raw_dir.1 * m),
            dir: self.dir,
        }
    }

    fn left(&self) -> PosAndDir {
        PosAndDir {
            py: self.py,
            px: self.px,
            dir: self.dir.turn_left(),
        }
    }

    fn right(&self) -> PosAndDir {
        PosAndDir {
            py: self.py,
            px: self.px,
            dir: self.dir.turn_right(),
        }
    }

    fn next_moves_and_costs(&self, forward: bool) -> [(PosAndDir, i32); 3] {
        [
            (self.step(forward), 1),
            (self.left(), 1000),
            (self.right(), 1000),
        ]
    }

    fn pos(&self) -> (usize, usize) {
        (self.py, self.px)
    }

    fn raw(&self) -> (usize, usize, usize) {
        (self.py, self.px, self.dir as usize)
    }
}

fn find_index(grid: &Array2<u8>, ch: u8) -> (usize, usize) {
    grid.indexed_iter()
        .find(|(_, &v)| v == ch)
        .map(|(pos, _)| pos)
        .expect("can't find ch")
}

fn part1(grid: &Array2<u8>, costs: &mut Array3<i32>, end: &(usize, usize)) -> i32 {
    let start = find_index(&grid, b'S');
    let start = PosAndDir {
        py: start.0,
        px: start.1,
        dir: Dir::RIGHT,
    };

    // Dumb VecDeque faster than BinaryHeap today
    let mut q = VecDeque::<(i32, PosAndDir)>::default();

    costs[start.raw()] = 0;
    q.push_back((0, start));

    while let Some((cost, pos)) = q.pop_front() {
        for (next_pos, c) in pos.next_moves_and_costs(true).into_iter() {
            let next_cost = cost - c;

            if let Some(&tile) = grid.get(next_pos.pos()) {
                if tile != b'#' && next_cost > costs[next_pos.raw()] {
                    costs[next_pos.raw()] = next_cost;
                    q.push_back((next_cost, next_pos));
                }
            }
        }
    }

    -costs
        .slice(s![end.0, end.1, ..])
        .iter()
        .max()
        .expect("no solution")
}

fn part2(grid: &Array2<u8>, costs: &Array3<i32>, end: &(usize, usize), p1_score: i32) -> usize {
    let mut q = VecDeque::default();

    let mut best_paths = BitVecSet2D::new(grid.dim());
    best_paths.insert(*end);

    for d in [Dir::UP, Dir::DOWN, Dir::LEFT, Dir::RIGHT] {
        let pos = PosAndDir {
            py: end.0,
            px: end.1,
            dir: d,
        };
        if Some(&-p1_score) == costs.get(pos.raw()) {
            q.push_back(pos);
        }
    }

    while let Some(pos) = q.pop_front() {
        for (next_pos, c) in pos.next_moves_and_costs(false).into_iter() {
            let next_cost = costs[pos.raw()] + c;

            if Some(&next_cost) == costs.get(next_pos.raw()) {
                best_paths.insert(next_pos.pos());
                q.push_back(next_pos);
            }
        }
    }

    best_paths.len()
}

fn calculate(raw_inp: &str) -> (i32, usize) {
    let grid = make_byte_grid(raw_inp);
    let end = find_index(&grid, b'E');
    let mut costs: Array3<i32> = Array3::from_elem((grid.dim().0, grid.dim().1, 4), i32::MIN);

    let p1 = part1(&grid, &mut costs, &end);
    let p2 = part2(&grid, &costs, &end, p1);

    return (p1, p2);
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

    const EXAMPLE_DATA_1: &str = include_str!("../../inputs/examples/2024_16_1");
    const EXAMPLE_DATA_2: &str = include_str!("../../inputs/examples/2024_16_2");
    const REAL_DATA: &str = include_str!("../../inputs/real/2024_16");

    #[test]
    fn test_example_1() {
        assert_eq!(calculate(&EXAMPLE_DATA_1), (7036, 45));
    }

    #[test]
    fn test_example_2() {
        assert_eq!(calculate(&EXAMPLE_DATA_2), (11048, 64));
    }

    #[test]
    fn test_real() {
        assert_eq!(calculate(&REAL_DATA), (98484, 531));
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
