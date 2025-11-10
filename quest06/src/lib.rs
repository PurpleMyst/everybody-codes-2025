use std::fmt::Display;

use rayon::prelude::*;

#[inline]
pub fn solve() -> (impl Display, impl Display, impl Display) {
    (solve_part1(), solve_part2(), solve_part3())
}

#[inline]
pub fn solve_part1() -> impl Display {
    let mut a_so_far = 0;

    let mut a_pairs = 0;

    for l in include_str!("part1.txt").trim().bytes() {
        match l {
            b'A' => a_so_far += 1,
            b'a' => a_pairs += a_so_far,
            _ => {}
        }
    }

    a_pairs
}

#[inline]
pub fn solve_part2() -> impl Display {
    let mut a_so_far = 0;
    let mut b_so_far = 0;
    let mut c_so_far = 0;

    let mut a_pairs = 0;
    let mut b_pairs = 0;
    let mut c_pairs = 0;

    for l in include_str!("part2.txt").trim().bytes() {
        match l {
            b'A' => a_so_far += 1,
            b'B' => b_so_far += 1,
            b'C' => c_so_far += 1,
            b'a' => a_pairs += a_so_far,
            b'b' => b_pairs += b_so_far,
            b'c' => c_pairs += c_so_far,
            _ => {}
        }
    }

    a_pairs + b_pairs + c_pairs
}

#[inline]
pub fn solve_part3() -> impl Display {
    const DISTANCE_LIMIT: usize = 1000;
    const REPEATS: usize = 1000;

    let input = include_str!("part3.txt").trim().as_bytes().repeat(REPEATS);

    let len = input.len();

    let (a, b, c) = input
        .par_iter()
        .enumerate()
        .map(|(i, &l)| -> (usize, usize, usize) {
            if !matches!(l, b'a'..=b'c') {
                return (0, 0, 0);
            }
            let mentor = l.to_ascii_uppercase();
            let mut total = 0;

            for &m in &input[i.saturating_sub(DISTANCE_LIMIT)..=(i + DISTANCE_LIMIT).min(len - 1)] {
                if m == mentor {
                    total += 1;
                }
            }

            match l {
                b'a' => (total, 0, 0),
                b'b' => (0, total, 0),
                b'c' => (0, 0, total),
                _ => unreachable!(),
            }
        })
        .reduce(|| (0, 0, 0), |(x, y, z), (a, b, c)| (x + a, y + b, z + c));
    a + b + c
}

