#![cfg_attr(feature = "bench", feature(test))]
use advent_of_code_2024::{Cli, Parser};
use ahash::{AHashMap, AHashSet};
use itertools::Itertools;
use std::fs;

/// https://en.wikipedia.org/wiki/Bron%E2%80%93Kerbosch_algorithm
fn bron_kerbosch<'a>(
    mut r: Vec<&'a str>,
    p: Vec<&'a str>,
    mut x: Vec<&'a str>,
    connections: &AHashMap<&'a str, AHashSet<&'a str>>,
    cliques: &mut Vec<Vec<&'a str>>,
) {
    if p.is_empty() && x.is_empty() {
        if r.len() >= 3 {
            r.sort_unstable();
            cliques.push(r);
        }
        return;
    }

    let u: &str = x.first().or(p.first()).expect("can't make pivot");
    let nu = connections.get(u).expect("no connections?");

    let mut reduced_p = p
        .iter()
        .filter(|itm| !nu.contains(*itm))
        .collect::<Vec<_>>();

    while let Some(v) = reduced_p.pop() {
        let mut r_inner = r.clone();
        r_inner.push(v);

        let connected = connections.get(v).expect("no connections?");

        let p_inner = p
            .iter()
            .filter(|&itm| connected.contains(itm))
            .copied()
            .collect::<Vec<_>>();

        let x_inner = x
            .iter()
            .filter(|&itm| connected.contains(itm))
            .copied()
            .collect::<Vec<_>>();

        bron_kerbosch(r_inner, p_inner, x_inner, connections, cliques);

        x.push(v);
    }
}

fn calculate(raw_inp: &str) -> (usize, String) {
    let connections = raw_inp
        .lines()
        .filter_map(|l| l.split_once("-"))
        .flat_map(|(a, b)| [(a, b), (b, a)])
        .fold(
            AHashMap::<&str, AHashSet<&str>>::default(),
            |mut acc, elem| {
                acc.entry(elem.0).or_default().insert(elem.1);
                acc
            },
        );

    let mut groups: Vec<Vec<&str>> = vec![];
    bron_kerbosch(
        vec![],
        connections.keys().copied().collect_vec(),
        vec![],
        &connections,
        &mut groups,
    );

    let p1 = groups
        .iter()
        .flat_map(|g| g.iter().combinations(3))
        .filter(|g| g.iter().any(|gi| gi.starts_with("t")))
        .unique()
        .count();

    let p2 = groups
        .iter()
        .max_by_key(|g| g.len())
        .map(|g| g.iter().join(","))
        .expect("no p2 solution?");

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

    const EXAMPLE_DATA: &str = include_str!("../../inputs/examples/2024_23");
    const REAL_DATA: &str = include_str!("../../inputs/real/2024_23");

    #[test]
    fn test_example() {
        assert_eq!(calculate(EXAMPLE_DATA), (7, "co,de,ka,ta".to_string()));
    }

    #[test]
    fn test_real() {
        assert_eq!(
            calculate(REAL_DATA),
            (1218, "ah,ap,ek,fj,fr,jt,ka,ln,me,mp,qa,ql,zg".to_string())
        );
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
