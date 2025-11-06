use std::fmt::Display;

#[inline]
pub fn solve() -> (impl Display, impl Display, impl Display) {
    (solve_part1(), solve_part2(), solve_part3())
}

#[inline]
pub fn solve_part1() -> impl Display {
    let input = include_str!("part1.txt");
    let mut numbers = parse(input).into_iter();
    let first = numbers.next().unwrap();
    let last = numbers.last().unwrap();
    2025 * first / last
}

#[inline]
pub fn solve_part2() -> impl Display {
    let input = include_str!("part2.txt");
    let mut numbers = parse(input).into_iter();
    let first = numbers.next().unwrap();
    let last = numbers.last().unwrap();
    let ratio = first as f64 / last as f64;
    (10000000000000.0 / ratio).ceil() as u64
}

#[inline]
pub fn solve_part3() -> impl Display {
    let input = include_str!("part3.txt");
    let mut it = input.lines();
    let mut prev = it.next().unwrap().trim().parse::<f64>().unwrap();
    let mut ratio = 1.0;
    for line in it {
        if let Some((a, b)) = line.split_once('|') {
            let a = a.trim().parse::<f64>().unwrap();
            let b = b.trim().parse::<f64>().unwrap();
            ratio *= prev / a;
            prev = b;
        } else {
            ratio *= prev / line.trim().parse::<f64>().unwrap();
        }
    }
    (100. * ratio).floor() as u64
}

fn parse(input: &'static str) -> impl IntoIterator<Item = u64> {
    input.lines().map(|line| line.trim().parse::<u64>().unwrap())
}
