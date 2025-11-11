use std::fmt::Display;

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

    let mut left_a_knights = 0;
    let mut left_b_knights = 0;
    let mut left_c_knights = 0;
    for &tent in input.iter().take(DISTANCE_LIMIT + 1) {
        match tent {
            b'A' => left_a_knights += 1,
            b'B' => left_b_knights += 1,
            b'C' => left_c_knights += 1,
            _ => {}
        }
    }

    let mut mid_a_knights = left_a_knights;
    let mut mid_b_knights = left_b_knights;
    let mut mid_c_knights = left_c_knights;
    for &tent in input.iter().rev().take(DISTANCE_LIMIT + 1) {
        match tent {
            b'A' => mid_a_knights += 1,
            b'B' => mid_b_knights += 1,
            b'C' => mid_c_knights += 1,
            _ => {}
        }
    }

    for j in 0..DISTANCE_LIMIT {
        let k = j + DISTANCE_LIMIT + 1;
        match input[j] {
            b'a' => left += left_a_knights,
            b'b' => left += left_b_knights,
            b'c' => left += left_c_knights,
            _ => {}
        }
        match input[k] {
            b'A' => left_a_knights += 1,
            b'B' => left_b_knights += 1,
            b'C' => left_c_knights += 1,
            _ => {}
        }
    }
    for i in 0..len - DISTANCE_LIMIT {
        let j = i + DISTANCE_LIMIT;
        let k = i + 2 * DISTANCE_LIMIT + 1;

        match input[j] {
            b'a' => left += left_a_knights,
            b'b' => left += left_b_knights,
            b'c' => left += left_c_knights,
            _ => {}
        }
        match input[i] {
            b'A' => left_a_knights -= 1,
            b'B' => left_b_knights -= 1,
            b'C' => left_c_knights -= 1,
            _ => {}
        }
        match input[k % len] {
            b'A' => left_a_knights += 1,
            b'B' => left_b_knights += 1,
            b'C' => left_c_knights += 1,
            _ => {}
        }
    }

    let mut mid = 0;
    for j in 0..len - DISTANCE_LIMIT - 1 {
        let i = j as isize - DISTANCE_LIMIT as isize;
        let k = j + DISTANCE_LIMIT + 1;

        match input[j] {
            b'a' => mid += mid_a_knights,
            b'b' => mid += mid_b_knights,
            b'c' => mid += mid_c_knights,
            _ => {}
        }
        match input[i.rem_euclid(len as isize) as usize] {
            b'A' => mid_a_knights -= 1,
            b'B' => mid_b_knights -= 1,
            b'C' => mid_c_knights -= 1,
            _ => {}
        }
        match input[k] {
            b'A' => mid_a_knights += 1,
            b'B' => mid_b_knights += 1,
            b'C' => mid_c_knights += 1,
            _ => {}
        }
    }

    let mut right_a_knights = mid_a_knights;
    let mut right_b_knights = mid_b_knights;
    let mut right_c_knights = mid_c_knights;
    let mut right = mid;

    for j in len - DISTANCE_LIMIT - 1..len {
        let i = j - DISTANCE_LIMIT;
        let k = j - (len - DISTANCE_LIMIT - 1);
        match input[j] {
            b'a' => mid += mid_a_knights,
            b'b' => mid += mid_b_knights,
            b'c' => mid += mid_c_knights,
            _ => {}
        }
        match input[i] {
            b'A' => mid_a_knights -= 1,
            b'B' => mid_b_knights -= 1,
            b'C' => mid_c_knights -= 1,
            _ => {}
        }
        match input[k] {
            b'A' => mid_a_knights += 1,
            b'B' => mid_b_knights += 1,
            b'C' => mid_c_knights += 1,
            _ => {}
        }
    }

    for j in len - DISTANCE_LIMIT - 1..len {
        let i = j - DISTANCE_LIMIT;
        match input[j] {
            b'a' => right += right_a_knights,
            b'b' => right += right_b_knights,
            b'c' => right += right_c_knights,
            _ => {}
        }
        match input[i] {
            b'A' => right_a_knights -= 1,
            b'B' => right_b_knights -= 1,
            b'C' => right_c_knights -= 1,
            _ => {}
        }
    }

    left + (REPEATS - 2) * mid + right
}
