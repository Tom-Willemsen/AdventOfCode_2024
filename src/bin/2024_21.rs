#![cfg_attr(feature = "bench", feature(test))]
use advent_of_code_2024::{Cli, Parser};
use ahash::AHashMap;
use ahash::AHashSet;
use itertools::Itertools;
use ndarray::Array2;
use std::fs;

type Cache = AHashMap<(usize, Vec<u8>), u64>;

struct RequiredMoveCounts {
    up: usize,
    down: usize,
    left: usize,
    right: usize,
}

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
struct Robot {
    y: usize,
    x: usize,
    keypad: Array2<u8>,
}

impl Robot {
    fn move_bot(&mut self, dir: u8) {
        match dir {
            b'v' => self.y += 1,
            b'^' => self.y = self.y.wrapping_sub(1),
            b'>' => self.x += 1,
            b'<' => self.x = self.x.wrapping_sub(1),
            b'A' => {}
            _ => panic!("invalid move"),
        }
    }

    fn required_moves(&self, target: (usize, usize)) -> RequiredMoveCounts {
        let y = target.0.abs_diff(self.y);
        let x = target.1.abs_diff(self.x);

        let left = if target.1 < self.x { x } else { 0 };
        let right = if target.1 > self.x { x } else { 0 };
        let down = if target.0 > self.y { y } else { 0 };
        let up = if target.0 < self.y { y } else { 0 };

        RequiredMoveCounts {
            left,
            right,
            up,
            down,
        }
    }

    fn type_code(&mut self, code: &[u8]) -> Vec<Vec<u8>> {
        let mut ans: Vec<Vec<u8>> = vec![];
        for c in code {
            let (target_y, target_x) = self.find(*c);
            let possible_moves = self.possible_code_sequences((target_y, target_x));

            if !ans.is_empty() {
                let mut new_answers = AHashSet::default();
                for a in &ans {
                    for pos in &possible_moves {
                        new_answers.insert(
                            a.iter()
                                .copied()
                                .chain(pos.iter().copied())
                                .collect::<Vec<_>>(),
                        );
                    }
                }

                ans = new_answers.into_iter().collect_vec();
            } else {
                ans = possible_moves;
            }

            self.y = target_y;
            self.x = target_x;
        }
        ans
    }

    fn possible_code_sequences(&self, target: (usize, usize)) -> Vec<Vec<u8>> {
        let mut ans = vec![];
        let required_moves = self.required_moves(target);

        ans.extend(vec![b'^'; required_moves.up]);
        ans.extend(vec![b'v'; required_moves.down]);
        ans.extend(vec![b'<'; required_moves.left]);
        ans.extend(vec![b'>'; required_moves.right]);

        let length = ans.len();
        ans.into_iter()
            .permutations(length)
            .unique()
            .filter(|seq| self.would_be_valid_move_sequence(seq))
            .map(|seq| seq.into_iter().chain([b'A']).collect::<Vec<_>>())
            .collect::<Vec<_>>()
    }

    fn is_in_valid_position(&self) -> bool {
        self.keypad[(self.y, self.x)] != b'#'
    }

    fn would_be_valid_move_sequence(&self, moves: &[u8]) -> bool {
        let mut simbot = self.clone();

        for c in moves {
            simbot.move_bot(*c);
            if !simbot.is_in_valid_position() {
                return false;
            }
        }
        true
    }

    fn find(&self, needle: u8) -> (usize, usize) {
        self.keypad
            .indexed_iter()
            .find(|(_, &v)| v == needle)
            .map(|(pos, _)| pos)
            .expect("can't find target")
    }
}

fn minimum_cost_dir_bot(code: &[u8], depth: usize, cache: &mut Cache) -> u64 {
    if depth == 0 {
        return code.len() as u64;
    } else if let Some(&cached_result) = cache.get(&(depth, code.to_vec())) {
        return cached_result;
    }

    let dir_keypad: Array2<u8> =
        Array2::from_shape_vec((2, 3), vec![b'#', b'^', b'A', b'<', b'v', b'>'])
            .expect("static data");

    let mut bot = Robot {
        y: 0,
        x: 2,
        keypad: dir_keypad,
    };

    let result = bot
        .type_code(code)
        .into_iter()
        .map(|way| {
            let mut fragments = vec![];
            let mut fragment = vec![];
            for item in way {
                fragment.push(item);
                if item == b'A' {
                    fragments.push(fragment);
                    fragment = vec![];
                }
            }

            fragments
                .into_iter()
                .map(|f| minimum_cost_dir_bot(&f, depth - 1, cache))
                .sum()
        })
        .min()
        .expect("no solution?");

    cache.insert((depth, code.to_vec()), result);
    result
}

fn minimum_cost<const DEPTH: usize>(code: &[u8], cache: &mut Cache) -> u64 {
    let num_keypad: Array2<u8> = Array2::from_shape_vec(
        (4, 3),
        vec![
            b'7', b'8', b'9', b'4', b'5', b'6', b'1', b'2', b'3', b'#', b'0', b'A',
        ],
    )
    .expect("static data");

    let mut numeric_robot = Robot {
        y: 3,
        x: 2,
        keypad: num_keypad,
    };

    let numeric_bot_ways = numeric_robot.type_code(code);

    numeric_bot_ways
        .into_iter()
        .map(|way| minimum_cost_dir_bot(&way, DEPTH, cache))
        .min()
        .expect("no solution?")
}

fn complexity_scores(code: &[u8], cache: &mut Cache) -> (u64, u64) {
    let numeric_part = code
        .iter()
        .filter(|&c| c != &b'A')
        .map(|c| c - b'0')
        .fold(0, |acc, elem| acc * 10 + elem as u64);

    let p1 = minimum_cost::<2>(code, cache) * numeric_part;
    let p2 = minimum_cost::<25>(code, cache) * numeric_part;

    (p1, p2)
}

fn calculate(raw_inp: &str) -> (u64, u64) {
    let mut cache = AHashMap::default();
    raw_inp
        .lines()
        .map(|line| complexity_scores(line.as_bytes(), &mut cache))
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

    const EXAMPLE_DATA: &str = include_str!("../../inputs/examples/2024_21");
    const REAL_DATA: &str = include_str!("../../inputs/real/2024_21");

    #[test]
    fn test_example() {
        assert_eq!(calculate(EXAMPLE_DATA).0, 126384);
    }

    #[test]
    fn test_real() {
        assert_eq!(calculate(REAL_DATA), (215374, 260586897262600));
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
