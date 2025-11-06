use std::fmt::Display;

use atoi::atoi;

#[inline]
pub fn solve() -> (impl Display, impl Display, impl Display) {
    (solve_part1(), solve_part2(), solve_part3())
}

#[inline]
pub fn solve_part1() -> impl Display {
    let input = include_bytes!("part1.txt");
    let mut numbers = parse(input).into_iter();
    let first = numbers.next().unwrap();
    let last = numbers.last().unwrap();
    2025 * first / last
}

#[inline]
pub fn solve_part2() -> impl Display {
    let input = include_bytes!("part2.txt");
    let mut numbers = parse(input).into_iter();
    let first = numbers.next().unwrap();
    let last = numbers.last().unwrap();
    (10000000000000 * last).div_ceil(first)
}

#[inline]
pub fn solve_part3() -> impl Display {
    let input = include_bytes!("part3.txt");
    let mut it = input[..input.len() - 1].split(|&b| b == b'\n');
    let mut prev = atoi::<u64>(it.next().unwrap()).unwrap() as f64;
    let mut ratio = 1.0;
    for line in it {
        if let Some(i) = line.iter().position(|&b| b == b'|') {
            let (a, b) = line.split_at(i);
            let a = atoi::<u64>(a).unwrap() as f64;
            let b = atoi::<u64>(&b[1..]).unwrap() as f64;
            ratio *= prev / a;
            prev = b;
        } else {
            ratio *= prev / atoi::<u64>(line).unwrap() as f64;
        }
    }
    (100. * ratio).floor() as u64
}

fn parse(input: &[u8]) -> impl IntoIterator<Item = u64> {
    input[..input.len() - 1].split(|&b| b == b'\n').map(|line| atoi::<u64>(line).unwrap())
}
