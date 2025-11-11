use std::fmt::Display;

use itertools::izip;

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
    const REPEATS: u32 = 1000;

    let input = include_str!("part3.txt").trim().as_bytes();
    let len = input.len();
    debug_assert!(input.len() > DISTANCE_LIMIT);

    let mut left = 0u32;
    let mut mid = 0u32;
    let mut right = 0u32;

    let mut left_a_knights = 0;
    let mut left_b_knights = 0;
    let mut left_c_knights = 0;
    let mut midright_a_knights = 0;
    let mut midright_b_knights = 0;
    let mut midright_c_knights = 0;

    for &tent in input.iter().take(DISTANCE_LIMIT + 1) {
        match tent {
            b'A' => left_a_knights += 1,
            b'B' => left_b_knights += 1,
            b'C' => left_c_knights += 1,
            _ => {}
        }
    }

    for &tent in input.iter().rev().take(DISTANCE_LIMIT + 1) {
        match tent {
            b'A' => midright_a_knights += 1,
            b'B' => midright_b_knights += 1,
            b'C' => midright_c_knights += 1,
            _ => {}
        }
    }

    {
        let mut a_knights = left_a_knights;
        let mut b_knights = left_b_knights;
        let mut c_knights = left_c_knights;

        for (i, j, k) in izip!(-(DISTANCE_LIMIT as isize).., 0..len, DISTANCE_LIMIT + 1..) {
            match input[j] {
                b'a' => left += a_knights,
                b'b' => left += b_knights,
                b'c' => left += c_knights,
                _ => {}
            }
            if i >= 0 {
                match input[i as usize] {
                    b'A' => a_knights -= 1,
                    b'B' => b_knights -= 1,
                    b'C' => c_knights -= 1,
                    _ => {}
                }
            }
            match input[k % len] {
                b'A' => a_knights += 1,
                b'B' => b_knights += 1,
                b'C' => c_knights += 1,
                _ => {}
            }
        }
    }

    {
        let mut a_knights = left_a_knights + midright_a_knights;
        let mut b_knights = left_b_knights + midright_b_knights;
        let mut c_knights = left_c_knights + midright_c_knights;

        for (i, j, k) in izip!(-(DISTANCE_LIMIT as isize).., 0..len, DISTANCE_LIMIT + 1..) {
            match input[j] {
                b'a' => mid += a_knights,
                b'b' => mid += b_knights,
                b'c' => mid += c_knights,
                _ => {}
            }
            match input[i.rem_euclid(len as isize) as usize] {
                b'A' => a_knights -= 1,
                b'B' => b_knights -= 1,
                b'C' => c_knights -= 1,
                _ => {}
            }
            match input[k % len] {
                b'A' => a_knights += 1,
                b'B' => b_knights += 1,
                b'C' => c_knights += 1,
                _ => {}
            }
        }
    }

    {
        let mut a_knights = left_a_knights + midright_a_knights;
        let mut b_knights = left_b_knights + midright_b_knights;
        let mut c_knights = left_c_knights + midright_c_knights;

        for (i, j, k) in izip!(-(DISTANCE_LIMIT as isize).., 0..len, DISTANCE_LIMIT + 1..) {
            match input[j] {
                b'a' => right += a_knights,
                b'b' => right += b_knights,
                b'c' => right += c_knights,
                _ => {}
            }
            match input[i.rem_euclid(len as isize) as usize] {
                b'A' => a_knights -= 1,
                b'B' => b_knights -= 1,
                b'C' => c_knights -= 1,
                _ => {}
            }
            if k < len {
                match input[k] {
                    b'A' => a_knights += 1,
                    b'B' => b_knights += 1,
                    b'C' => c_knights += 1,
                    _ => {}
                }
            }
        }
    }

    left + (REPEATS - 2) * mid + right
}
