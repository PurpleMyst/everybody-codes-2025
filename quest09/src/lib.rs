use std::{collections::HashSet, fmt::Display};

use itertools::Itertools;

#[inline]
pub fn solve() -> (impl Display, impl Display, impl Display) {
    (solve_part1(), solve_part2(), solve_part3())
}

#[inline]
pub fn solve_part1() -> impl Display {
    let scales = include_str!("part1.txt")
        .lines()
        .map(|line| line.split_once(':').unwrap().1.as_bytes())
        .collect_array().unwrap();
    similarity(scales).unwrap()
}

fn similarity(seqs: [&[u8]; 3]) -> Option<u32> {
    let mut can_be_child = [true, true, true];
    for i in 0..seqs[0].len() {
        let a = seqs[0][i];
        let b = seqs[1][i];
        let c = seqs[2][i];
        if a != b && a != c {
            can_be_child[0] = false;
        }
        if b != a && b != c {
            can_be_child[1] = false;
        }
        if c != a && c != b {
            can_be_child[2] = false;
        }
    }
    debug_assert!(can_be_child.iter().filter(|&&x| x).count() <= 1);

    let child_idx = can_be_child.iter().position(|&x| x)?;
    let parents = if child_idx == 0 {
        [1, 2]
    } else if child_idx == 1 {
        [0, 2]
    } else {
        [0, 1]
    };
    let mut result = 1;
    for p in parents {
        let mut matches = 0;
        for (i, &g) in seqs[child_idx].iter().enumerate() {
            if g == seqs[p][i] {
                matches += 1;
            }
        }
        result *= matches;
    }
    Some(result)
}

#[inline]
pub fn solve_part2() -> impl Display {
    let list = include_str!("part2.txt")
        .lines()
        .map(|line| line.split_once(':').unwrap().1.as_bytes());

    list.array_combinations()
        .filter_map(|candidate_family| {
            similarity(candidate_family)
        })
        .sum::<u32>()
}

#[inline]
pub fn solve_part3() -> impl Display {
    let list = include_str!("part3.txt").lines().map(|line| {
        let (id, dna) = line.split_once(':').unwrap();
        (id.parse::<usize>().unwrap(), dna.as_bytes())
    });

    let mut families: Vec<std::collections::HashSet<(usize, &[u8])>> = Vec::new();

    list.array_combinations().for_each(|candidate_family| {
        let candidate_family_scales = candidate_family.map(|(_, dna)| dna);
        if similarity(candidate_family_scales).is_none() {
            return;
        }

        let mut new_family = HashSet::new();
        families.retain_mut(|family| {
            if candidate_family.iter().any(|member| family.contains(member)) {
                new_family.extend(family.iter().copied());
                return false;
            } else {
                return true;
            }
        });
        new_family.extend(candidate_family);
        families.push(new_family);
    });

    families
        .iter()
        .max_by_key(|f| f.len())
        .map(|f| f.into_iter().map(|p| p.0).sum::<usize>())
        .unwrap()
}
