use std::fmt::Display;

use itertools::Itertools;

#[derive(Clone, Copy)]
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
        .map(|line| Scale::new(line.split_once(':').unwrap().1.as_bytes()))
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
        .map(|line| Scale::new(line.split_once(':').unwrap().1.as_bytes()));

    list.array_combinations()
        .filter_map(|candidate_family| similarity(candidate_family))
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

    let mut ds = disjoint::DisjointSet::with_len(list.len());

    list.into_iter().array_combinations().for_each(|candidate_family| {
        let candidate_family_scales = candidate_family.map(|(_, dna)| dna);
        if similarity(candidate_family_scales).is_none() {
            return;
        }

        ds.join(candidate_family[0].0, candidate_family[1].0);
        ds.join(candidate_family[0].0, candidate_family[2].0);
    });

    ds.sets()
        .into_iter()
        .max_by_key(|f| f.len())
        .map(|f| f.into_iter().map(|p| p + 1).sum::<usize>())
        .unwrap()
}
