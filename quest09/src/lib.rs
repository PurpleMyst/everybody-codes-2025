use std::{cmp::Reverse, fmt::Display};

use itertools::Itertools;
use rayon::prelude::*;

#[derive(Clone, Copy, PartialEq)]
struct Scale {
    a: u128,
    c: u128,
    t: u128,
    g: u128,
}

impl Scale {
    fn new(dna: &[u8]) -> Self {
        let mut this = Scale { a: 0, c: 0, t: 0, g: 0 };
        for (i, &nucleotide) in dna.iter().enumerate() {
            match nucleotide {
                b'A' => this.a |= 1 << i,
                b'C' => this.c |= 1 << i,
                b'T' => this.t |= 1 << i,
                b'G' => this.g |= 1 << i,
                _ => unreachable!(),
            }
        }
        this
    }

    fn same_mask(&self, other: &Self) -> u128 {
        (self.a & other.a) | (self.c & other.c) | (self.t & other.t) | (self.g & other.g)
    }
}

#[inline]
pub fn solve() -> (impl Display, impl Display, impl Display) {
    (solve_part1(), solve_part2(), solve_part3())
}

#[inline]
pub fn solve_part1() -> impl Display {
    let scales = include_str!("part1.txt")
        .lines()
        .map(|line| Scale::new(line[2..].as_bytes()))
        .collect_array()
        .unwrap();
    similarity(scales).unwrap()
}

fn similarity(seqs: [Scale; 3]) -> Option<u32> {
    let m01 = seqs[0].same_mask(&seqs[1]);
    let m02 = seqs[0].same_mask(&seqs[2]);
    let m12 = seqs[1].same_mask(&seqs[2]);

    if (m01 | m02) == u128::MAX {
        // 0 is the child
        Some(m01.count_ones() * m02.count_ones())
    } else if (m01 | m12) == u128::MAX {
        // 1 is the child
        Some(m01.count_ones() * m12.count_ones())
    } else if (m02 | m12) == u128::MAX {
        // 2 is the child
        Some(m02.count_ones() * m12.count_ones())
    } else {
        // no child
        None
    }
}

#[inline]
pub fn solve_part2() -> impl Display {
    let list = include_str!("part2.txt")
        .lines()
        .map(|line| Scale::new(line.split_once(':').unwrap().1.as_bytes()))
        .collect_vec();

    list.par_iter()
        .filter_map(|child| {
            let candidates = list
                .iter()
                .filter(|&duck| duck != child)
                .sorted_unstable_by_key(|duck| Reverse(duck.same_mask(&child).count_ones()))
                .collect_vec();

            for (i, mother) in candidates.iter().enumerate() {
                let mother_mask = mother.same_mask(&child);
                let mother_matches = mother_mask.count_ones();

                for father in candidates.iter().skip(i + 1) {
                    let father_mask = father.same_mask(&child);
                    let father_matches = father_mask.count_ones();
                    if mother_matches + father_matches < 128 {
                        break;
                    }
                    if mother_mask | father_mask == u128::MAX {
                        return Some(father_matches * mother_matches);
                    }
                }
            }
            return None;
        })
        .sum::<u32>()
}

#[inline]
pub fn solve_part3() -> impl Display {
    let list = include_str!("part3.txt")
        .lines()
        .map(|line| {
            let (id, dna) = line.split_once(':').unwrap();
            (id.parse::<usize>().unwrap() - 1, Scale::new(dna.as_bytes()))
        })
        .collect_vec();

    let triplets = list
        .par_iter()
        .filter_map(|child| {
            let candidates = list
                .iter()
                .filter(|&duck| duck != child)
                .sorted_unstable_by_key(|duck| Reverse(duck.1.same_mask(&child.1).count_ones()))
                .collect_vec();

            for (i, mother) in candidates.iter().enumerate() {
                let mother_mask = mother.1.same_mask(&child.1);
                let mother_matches = mother_mask.count_ones();

                for father in candidates.iter().skip(i + 1) {
                    let father_mask = father.1.same_mask(&child.1);
                    let father_matches = father_mask.count_ones();
                    if mother_matches + father_matches < 128 {
                        break;
                    }
                    if mother_mask | father_mask == u128::MAX {
                        return Some((mother.0, father.0, child.0));
                    }
                }
            }
            return None;
        })
        .collect_vec_list();

    let mut ds = disjoint::DisjointSet::with_len(list.len());
    for (m, f, c) in triplets.into_iter().flatten() {
        ds.join(m, f);
        ds.join(m, c);
    }
    ds.sets()
        .into_iter()
        .max_by_key(|f| f.len())
        .map(|f| f.into_iter().map(|p| p + 1).sum::<usize>())
        .unwrap()
}
