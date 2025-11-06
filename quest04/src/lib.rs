use std::fmt::Display;

use atoi::FromRadix10;

#[inline]
pub fn solve() -> (impl Display, impl Display, impl Display) {
    (solve_part1(), solve_part2(), solve_part3())
}

#[inline]
pub fn solve_part1() -> impl Display {
    let input = include_bytes!("part1.txt");
    let (first, last) = parse(input);
    2025 * first / last
}

#[inline]
pub fn solve_part2() -> impl Display {
    let input = include_bytes!("part2.txt");
    let (first, last) = parse(input);
    (10_000_000_000_000 * last).div_ceil(first)
}

#[inline]
pub fn solve_part3() -> impl Display {
    let input = include_bytes!("part3.txt");
    let mut it = input[..input.len() - 1].split(|&b| b == b'\n');
    let mut prev = u64::from_radix_10(it.next().unwrap()).0 as f64;
    let mut ratio = 100.0;
    for line in it {
        if let Some(i) = line.iter().position(|&b| b == b'|') {
            let (a, b) = line.split_at(i);
            let a = u64::from_radix_10(a).0 as f64;
            let b = u64::from_radix_10(&b[1..]).0 as f64;
            ratio *= prev / a;
            prev = b;
        } else {
            ratio *= prev / u64::from_radix_10(line).0 as f64;
        }
    }
    ratio as u64
}

fn parse(input: &[u8]) -> (u64, u64) {
    let first = u64::from_radix_10(input).0;
    let last_line_start = input[..input.len() - 1].iter().rposition(|&b| b == b'\n').unwrap() + 1;
    let last = u64::from_radix_10(&input[last_line_start..]).0;
    (first, last)
}
