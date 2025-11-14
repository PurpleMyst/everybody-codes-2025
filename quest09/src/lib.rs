use std::{collections::HashSet, fmt::Display};

use itertools::{Itertools, izip};

#[inline]
pub fn solve() -> (impl Display, impl Display, impl Display) {
    (solve_part1(), solve_part2(), solve_part3())
}

#[inline]
pub fn solve_part1() -> impl Display {
    let mut seqs = include_str!("part1.txt")
        .lines()
        .map(|line| line.split_once(':').unwrap().1.as_bytes());
    let seqs: [&[u8]; 3] = std::array::from_fn(|_| seqs.next().unwrap());

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
    debug_assert_eq!(can_be_child.iter().filter(|&&x| x).count(), 1);

    let child_idx = can_be_child.iter().position(|&x| x).unwrap();
    let parents = if child_idx == 0 {
        [1, 2]
    } else if child_idx == 1 {
        [0, 2]
    } else {
        [0, 1]
    };
    let mut result = 1;
    for p in parents {
        let mut score = 0;
        for (i, &g) in seqs[child_idx].iter().enumerate() {
            if g == seqs[p][i] {
                score += 1;
            }
        }
        result *= score;
    }

    result
}

#[inline]
pub fn solve_part2() -> impl Display {
    let list = include_str!("part2.txt")
        .lines()
        .map(|line| line.split_once(':').unwrap().1.as_bytes());

    list.permutations(3)
        .map(|candidate_family| {
            let mother = candidate_family[0];
            let father = candidate_family[1];
            let child = candidate_family[2];

            let mut mother_sim = 0;
            let mut father_sim = 0;
            for (m, f, c) in izip!(mother, father, child) {
                if m == c {
                    mother_sim += 1;
                }
                if f == c {
                    father_sim += 1;
                }
                if m != c && f != c {
                    return 0;
                }
            }

            father_sim * mother_sim
        })
        .sum::<usize>()
        / 2
}

#[inline]
pub fn solve_part3() -> impl Display {
    let list = include_str!("part3.txt").lines().map(|line| {
        let (id, dna) = line.split_once(':').unwrap();
        (id.parse::<usize>().unwrap(), dna.as_bytes())
    });

    let mut families: Vec<std::collections::HashSet<(usize, &[u8])>> = Vec::new();

    list.permutations(3).for_each(|candidate_family| {
        let mother = candidate_family[0];
        let father = candidate_family[1];
        let child = candidate_family[2];

        for (m, f, c) in izip!(mother.1, father.1, child.1) {
            if m != c && f != c {
                return;
            }
        }

        let mut new_family = HashSet::new();
        families.retain_mut(|family| {
            if family.contains(&mother) || family.contains(&father) || family.contains(&child) {
                new_family.extend(family.iter().copied());
                return false;
            } else {
                return true;
            }
        });
        new_family.insert(mother);
        new_family.insert(father);
        new_family.insert(child);
        families.push(new_family);
    });

    families
        .iter()
        .max_by_key(|f| f.len())
        .map(|f| f.into_iter().map(|p| p.0).sum::<usize>())
        .unwrap()
}
