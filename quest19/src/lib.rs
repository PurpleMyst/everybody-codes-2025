use std::fmt::Display;

use itertools::Itertools;
use atoi::FromRadix10;

#[inline]
pub fn solve() -> (impl Display, impl Display, impl Display) {
    (solve_part1(), solve_part2(), solve_part3())
}

#[inline]
pub fn solve_part1() -> impl Display {
    let input = include_str!("part1.txt");
    do_solve(input)
}

fn do_solve(input: &'static str) -> usize {
    let walls = input
        .lines()
        .map(|line| -> (usize, usize, usize) { line.split(',').map(|n| usize::from_radix_10(n.as_bytes()).0).collect_tuple().unwrap() });
    let mut flaps = 0;
    let mut cur_x = 0;
    let mut cur_y = 0;
    for (tx, gaps) in &walls.chunk_by(|(x, _, _)| *x) {
        let ty = gaps.map(|(_, y, _)| y).min().unwrap();
        let dx = tx - cur_x;
        let needed = (ty + dx).saturating_sub(cur_y).div_ceil(2);
        flaps += needed;
        cur_x = tx;
        cur_y += 2 * needed;
        cur_y -= dx;
    }
    flaps
}

#[inline]
pub fn solve_part2() -> impl Display {
    let input = include_str!("part2.txt");
    do_solve(input)
}

#[inline]
pub fn solve_part3() -> impl Display {
    let input = include_str!("part3.txt");
    do_solve(input)
}
