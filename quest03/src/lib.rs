use std::fmt::Display;

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
    let mut crates = include_str!("part3.txt")
        .trim()
        .split(',')
        .map(|s| s.parse().unwrap())
        .collect::<Vec<u64>>();

    let mut n = 0;
    while !crates.is_empty() {
        crates.sort_unstable();
        let mut i = 0;

        while i < crates.len() {
            let element = crates.remove(i);
            while i < crates.len() && crates[i] == element {
                i += 1;
            }
        }

        n += 1;
    }
    n
}
