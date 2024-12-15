#![cfg_attr(feature = "bench", feature(test))]
use advent_of_code_2024::{grid_util::make_byte_grid, Cli, Parser};
use ndarray::Array2;
use std::{collections::VecDeque, fs};

fn score_grid<const MATCH: u8>(grid: &Array2<u8>) -> usize {
    grid.indexed_iter()
        .filter_map(|(pos, &v)| (v == MATCH).then_some(100 * pos.0 + pos.1))
        .sum()
}

fn get_dir(mv: &u8) -> (isize, isize) {
    match mv {
        b'^' => (-1, 0),
        b'v' => (1, 0),
        b'<' => (0, -1),
        b'>' => (0, 1),
        _ => panic!("bad move"),
    }
}

fn new_pos(pos: (usize, usize), mv: &u8) -> (usize, usize) {
    let dir = get_dir(mv);
    (
        pos.0.wrapping_add_signed(dir.0),
        pos.1.wrapping_add_signed(dir.1),
    )
}

fn part1(mut grid: Array2<u8>, moves: &[u8], start_pos: (usize, usize)) -> usize {
    let mut pos = start_pos;
    for mv in moves {
        let mut next_pos = new_pos(pos, mv);
        let next_robot_pos = next_pos;

        while let Some(b'O') = grid.get(next_pos) {
            next_pos = new_pos(next_pos, mv);
        }

        if grid.get(next_pos) == Some(&b'.') {
            grid[pos] = b'.';
            grid[next_pos] = b'O';
            grid[next_robot_pos] = b'@';
            pos = next_robot_pos;
        }
    }

    score_grid::<b'O'>(&grid)
}

fn blow_up_grid(grid: &Array2<u8>) -> Array2<u8> {
    Array2::from_shape_fn((grid.dim().0, grid.dim().1 * 2), |idx| {
        let even = idx.1 % 2 == 0;
        match (grid[(idx.0, idx.1 / 2)], even) {
            (b'#', _) => b'#',
            (b'.', _) => b'.',
            (b'O', true) => b'[',
            (b'O', false) => b']',
            (b'@', true) => b'@',
            (b'@', false) => b'.',
            _ => panic!("invalid type"),
        }
    })
}

fn part2(original_grid: &Array2<u8>, moves: &[u8], start_pos: (usize, usize)) -> usize {
    let mut grid = blow_up_grid(original_grid);
    let mut pos = (start_pos.0, start_pos.1 * 2);

    let mut q = VecDeque::default();
    let mut moved_from = Vec::default();

    for mv in moves {
        let next_pos = new_pos(pos, mv);

        if let Some(b'.') = grid.get(next_pos) {
            pos = next_pos;
            continue;
        }

        q.clear();
        moved_from.clear();

        q.push_back(pos);

        let mut can_move = true;

        while let Some(p) = q.pop_front() {
            if moved_from.contains(&p) {
                continue;
            }

            let np = new_pos(p, mv);
            moved_from.push(p);

            match grid.get(np) {
                Some(&b'[') => {
                    q.push_back(np);
                    if mv == &b'^' || mv == &b'v' {
                        q.push_back((np.0, np.1 + 1));
                    }
                }
                Some(&b']') => {
                    q.push_back(np);
                    if mv == &b'^' || mv == &b'v' {
                        q.push_back((np.0, np.1.wrapping_add_signed(-1)));
                    }
                }
                Some(b'#') | None => {
                    can_move = false;
                    break;
                }
                _ => {}
            }
        }

        if can_move {
            // This works because we built up moved_from in order of increasing
            // distance, so can iterate back in reverse
            moved_from.iter().rev().for_each(|&pos| {
                let np = new_pos(pos, mv);
                grid[np] = grid[pos];
                grid[pos] = b'.';
            });

            pos = next_pos;
        }
    }

    score_grid::<b'['>(&grid)
}

fn calculate(raw_inp: &str) -> (usize, usize) {
    let (head, tail) = raw_inp.split_once("\n\n").expect("invalid format");

    let grid = make_byte_grid(head);

    let pos = grid
        .indexed_iter()
        .find(|(_, &v)| v == b'@')
        .map(|(pos, _)| pos)
        .expect("can't find robot start pos");

    let moves = tail.bytes().filter(|&b| b != b'\n').collect::<Vec<_>>();

    let p2 = part2(&grid, &moves, pos);
    let p1 = part1(grid, &moves, pos);

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

    const EXAMPLE_DATA: &str = include_str!("../../inputs/examples/2024_15");
    const REAL_DATA: &str = include_str!("../../inputs/real/2024_15");

    const SMALL_EX: &str = "########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

<^^>>>vv<v>>v<<
";

    #[test]
    fn test_small_example_p1() {
        assert_eq!(calculate(&SMALL_EX).0, 2028);
    }

    #[test]
    fn test_scoring_p1() {
        let grid = "#######
#...O..
#......
";
        assert_eq!(score_grid::<b'O'>(&make_byte_grid(grid)), 104);
    }

    #[test]
    fn test_scoring_p2() {
        let grid = "##########
##...[]...
##........
";
        assert_eq!(score_grid::<b'['>(&make_byte_grid(grid)), 105);
    }

    #[test]
    fn test_example() {
        assert_eq!(calculate(&EXAMPLE_DATA), (10092, 9021));
    }

    #[test]
    fn test_real() {
        assert_eq!(calculate(&REAL_DATA), (1438161, 1437981));
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
