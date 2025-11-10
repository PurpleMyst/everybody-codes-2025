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
    const REPEATS: usize = 1000;

    let input = include_str!("part3.txt").trim().as_bytes();
    let len = input.len();

    let compute = |to_left| {
        let mut total = 0u32;
        for (i, &l) in input.iter().enumerate() {
            if !l.is_ascii_lowercase() {
                continue;
            }
            let mentor = l.to_ascii_uppercase();
            for j in i as isize - DISTANCE_LIMIT as isize..=(i + DISTANCE_LIMIT) as isize {
                if input[j.rem_euclid(len as isize) as usize] == mentor
                    && j >= -(to_left as isize) * len as isize
                    && j < (REPEATS - to_left) as isize * len as isize
                {
                    total += 1;
                }
            }
        }
        total
    };
    let (left, (mid, right)) = rayon::join(|| compute(0), || rayon::join(|| compute(1), || compute(REPEATS - 1)));
    left + mid + right
}
