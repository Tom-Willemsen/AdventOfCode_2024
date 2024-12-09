#![cfg_attr(feature = "bench", feature(test))]
use advent_of_code_2024::{Cli, Parser};
use std::fs;

#[derive(Debug, Clone, Copy)]
struct Disk {
    id: Option<usize>,
    len: usize,
}

fn part1(raw_inp: &str) -> usize {
    let mut is_file = true;
    let mut id = 0_usize;

    let mut disk = raw_inp
        .trim()
        .bytes()
        .flat_map(|c| {
            let len = (c - &b'0') as usize;
            let mut r = vec![];
            for _ in 0..len {
                if is_file {
                    r.push(Some(id))
                } else {
                    r.push(None)
                }
            }
            if is_file {
                id += 1;
            }
            is_file = !is_file;
            return r;
        })
        .collect::<Vec<Option<usize>>>();

    let mut next_empty_slot = disk.iter().position(|&e| e == None).unwrap();
    while next_empty_slot < disk.len() {
        disk.swap_remove(next_empty_slot);
        while next_empty_slot < disk.len() && disk[next_empty_slot] != None {
            next_empty_slot += 1;
        }
    }

    disk.iter()
        .zip(0_usize..)
        .map(|(x, y)| x.unwrap() * y)
        .sum()
}

fn merge_free_space(disk: Vec<Disk>) -> Vec<Disk> {
    let mut nd: Vec<Disk> = vec![];

    for item in disk {
        if item.id == None {
            let l = nd.len();
            if nd[l - 1].id == None {
                nd.get_mut(l - 1).unwrap().len += item.len;
            } else if item.len > 0 {
                nd.push(item);
            }
        } else {
            nd.push(item);
        }
    }
    nd
}

fn part2(raw_inp: &str) -> usize {
    let mut is_file = true;
    let mut id = 0_usize;

    let mut disk = raw_inp
        .trim()
        .bytes()
        .map(|c| {
            let len = (c - &b'0') as usize;
            let r = Disk {
                id: if is_file { Some(id) } else { None },
                len: len,
            };

            if is_file {
                id += 1;
            }
            is_file = !is_file;
            return r;
        })
        .collect::<Vec<Disk>>();

    id -= 1;

    while id != 0 {
        let opos = disk.iter().position(|e| e.id == Some(id)).unwrap();

        let f = disk[opos];

        let npos = disk.iter().position(|e| e.id == None && e.len >= f.len);

        if let Some(npos) = npos {
            if npos >= opos {
                id -= 1;
                continue;
            }

            let f = disk[opos];
            let s = disk[npos];

            disk[opos].id = None;
            disk[npos] = f;

            disk.insert(
                npos + 1,
                Disk {
                    id: None,
                    len: (s.len - f.len),
                },
            );
        }

        // Merging free space doesn't actually seem to be required?
        disk = merge_free_space(disk);

        id -= 1;
    }

    disk.iter()
        .flat_map(|c| {
            let mut r = vec![];
            for _ in 0..c.len {
                r.push(c.id)
            }
            return r;
        })
        .zip(0_usize..)
        .map(|(x, y)| x.unwrap_or(0) * y)
        .sum()
}

fn calculate(raw_inp: &str) -> (usize, usize) {
    (part1(&raw_inp), part2(&raw_inp))
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

    const EXAMPLE_DATA: &str = include_str!("../../inputs/examples/2024_09");
    const REAL_DATA: &str = include_str!("../../inputs/real/2024_09");

    #[test]
    fn test_example() {
        assert_eq!(calculate(&EXAMPLE_DATA), (1928, 2858));
    }

    #[test]
    fn test_real() {
        assert_eq!(calculate(&REAL_DATA), (6385338159127, 6415163624282));
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
