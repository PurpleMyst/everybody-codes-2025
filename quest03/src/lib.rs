use std::{collections::HashMap, fmt::Display};

#[inline]
pub fn solve() -> (impl Display, impl Display, impl Display) {
    (solve_part1(), solve_part2(), solve_part3())
}

#[inline]
pub fn solve_part1() -> impl Display {
    let mut crates = include_str!("part1.txt")
        .trim()
        .split(',')
        .map(|s| s.parse().unwrap())
        .collect::<Vec<u64>>();
    crates.sort_unstable();
    crates.dedup();

    crates.iter().sum::<u64>()
}

#[inline]
pub fn solve_part2() -> impl Display {
    let mut crates = include_str!("part2.txt")
        .trim()
        .split(',')
        .map(|s| s.parse().unwrap())
        .collect::<Vec<u64>>();
    crates.sort_unstable();
    crates.dedup();
    crates[..20].iter().sum::<u64>()
}

#[inline]
pub fn solve_part3() -> impl Display {
    let crates = include_str!("part3.txt")
        .trim()
        .split(',')
        .map(|s| s.parse().unwrap())
        .collect::<Vec<u64>>();

    let mut seen = HashMap::<_, usize>::new();
    for c in crates {
        *seen.entry(c).or_default() += 1;
    }
    seen.into_values().max().unwrap()
}
