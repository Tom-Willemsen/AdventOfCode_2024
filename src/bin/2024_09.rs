#![cfg_attr(feature = "bench", feature(test))]
use advent_of_code_2024::{Cli, Parser};
use std::{collections::VecDeque, fs};

#[derive(Debug, Clone, Copy)]
struct AmphipodFile {
    id: usize,
    len: usize,
    offset: usize,
}

fn part1(raw_inp: &str) -> usize {
    let mut is_file = true;
    let mut id = 0_usize;

    let mut disk = raw_inp
        .trim()
        .bytes()
        .flat_map(|c| {
            let len = (c - b'0') as usize;
            let r = if is_file {
                [Some(id)].repeat(len)
            } else {
                [None].repeat(len)
            };
            if is_file {
                id += 1;
            }
            is_file = !is_file;
            r
        })
        .collect::<Vec<Option<usize>>>();

    let mut next_empty_slot = disk.iter().position(|&e| e.is_none()).unwrap();
    while next_empty_slot < disk.len() {
        disk.swap_remove(next_empty_slot);

        while next_empty_slot < disk.len() && disk[next_empty_slot].is_some() {
            next_empty_slot += 1;
        }
    }

    disk.into_iter()
        .zip(0_usize..)
        .map(|(x, y)| x.unwrap_or(0) * y)
        .sum()
}

fn sum_to_n(n: usize) -> usize {
    (n * (n + 1)) / 2
}

fn part2(raw_inp: &str) -> usize {
    let mut is_file = true;
    let mut id = 0_usize;

    let mut offset = 0_usize;

    let mut space_buckets: [VecDeque<usize>; 10] = Default::default();
    let mut files = vec![];

    raw_inp.trim().bytes().for_each(|c| {
        let len = (c - b'0') as usize;
        if is_file {
            files.push(AmphipodFile { id, len, offset });
            id += 1;
        } else {
            space_buckets[len].push_back(offset);
        }
        is_file = !is_file;
        offset += len;
    });

    for file in files.iter_mut().rev() {
        if let Some(bucket) = (file.len..space_buckets.len())
            .map(|i| (i, &space_buckets[i]))
            .filter(|(_, b)| b.front().map(|x| x < &file.offset).unwrap_or(false))
            .min_by(|a, b| a.1[0].cmp(&b.1[0]))
            .map(|x| x.0)
        {
            let space = space_buckets[bucket].pop_front().expect("empty bucket");

            file.offset = space;

            if file.len < bucket {
                let itm = space + file.len;
                let idx = match space_buckets[bucket - file.len].binary_search(&itm) {
                    Ok(i) => i,
                    Err(i) => i,
                };
                space_buckets[bucket - file.len].insert(idx, itm);
            }
        }
    }

    files
        .into_iter()
        .map(|f| f.id * (sum_to_n(f.len) + f.len * f.offset - f.len))
        .sum()
}

fn calculate(raw_inp: &str) -> (usize, usize) {
    (part1(raw_inp), part2(raw_inp))
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
