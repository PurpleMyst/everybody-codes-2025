use std::fmt::Display;

use atoi::atoi;

#[inline]
pub fn solve() -> (impl Display, impl Display, impl Display) {
    (solve_part1(), solve_part2(), solve_part3())
}

#[inline]
pub fn solve_part1() -> impl Display {
    let mut crates = include_str!("part1.txt")
        .trim()
        .as_bytes()
        .split(|&c| c == b',')
        .map(|s| atoi(s).unwrap())
        .collect::<Vec<u8>>();
    crates.sort_unstable();
    crates.dedup();
    crates.into_iter().map(|n| n as u16).sum::<u16>()
}

#[inline]
pub fn solve_part2() -> impl Display {
    let mut crates = include_str!("part2.txt")
        .trim()
        .as_bytes()
        .split(|&c| c == b',')
        .map(|s| atoi(s).unwrap())
        .collect::<Vec<u8>>();
    crates.sort_unstable();
    crates.dedup();
    crates.into_iter().take(20).map(|n| n as u16).sum::<u16>()
}

#[inline]
pub fn solve_part3() -> impl Display {
    let crates = include_str!("part3.txt")
        .trim()
        .as_bytes()
        .split(|&c| c == b',')
        .map(|s| atoi(s).unwrap())
        .collect::<Vec<u8>>();
    let mut frequency = [0u16; 100];
    for c in crates {
        frequency[c as usize] += 1;
    }
    frequency.into_iter().max().unwrap()
}
