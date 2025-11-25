use std::{cmp::Ordering, fmt::Display};

#[inline]
pub fn solve() -> (impl Display, impl Display, impl Display) {
    (solve_part1(), solve_part2(), solve_part3())
}

#[inline]
pub fn solve_part1() -> impl Display {
    let input = include_str!("part1.txt");
    let numbers = input.trim().split(',').map(|n| n.parse::<u64>().unwrap());
    numbers.map(|n| 90 / n).sum::<u64>()
}

#[inline]
pub fn solve_part2() -> impl Display {
    let input = include_str!("part2.txt");
    let mut numbers = input
        .trim()
        .split(',')
        .map(|n| n.parse::<u64>().unwrap())
        .collect::<Vec<_>>();
    let mut result = 1;

    let mut d = 1;
    while numbers.iter().any(|&n| n != 0) {
        if numbers.iter().skip(d - 1).step_by(d).all(|&n| n != 0) {
            numbers.iter_mut().skip(d - 1).step_by(d).for_each(|n| {
                *n -= 1;
            });
            result *= d;
        }
        d += 1;
    }
    result
}

#[inline]
pub fn solve_part3() -> impl Display {
    let input = include_str!("part3.txt");
    let mut numbers = input
        .trim()
        .split(',')
        .map(|n| n.parse::<u64>().unwrap())
        .collect::<Vec<_>>();
    let mut spell = Vec::new();
    let mut d = 1;

    while numbers.iter().any(|&n| n != 0) {
        if numbers.iter().skip(d - 1).step_by(d).all(|&n| n != 0) {
            numbers.iter_mut().skip(d - 1).step_by(d).for_each(|n| {
                *n -= 1;
            });
            spell.push(d);
        }
        d += 1;
    }

    let blocks_needed = |len: usize| spell.iter().map(|n| len / n).sum::<usize>();

    let mut low: usize = 0;
    let mut high: usize = 100_000_000_000_000;
    while low <= high {
        let mid = low + (high - low) / 2;
        match blocks_needed(mid).cmp(&202520252025000) {
            Ordering::Less => low = mid + 1,
            Ordering::Equal => return mid,
            Ordering::Greater => high = mid - 1,
        }
    }

    high
}
