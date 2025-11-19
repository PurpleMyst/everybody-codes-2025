use std::{collections::VecDeque, fmt::Display};

use atoi::FromRadix10;

#[inline]
pub fn solve() -> (impl Display, impl Display, impl Display) {
    (solve_part1(), solve_part2(), solve_part3())
}

#[inline]
pub fn solve_part1() -> impl Display {
    let input = include_str!("part1.txt");
    let xs = input.lines().map(|line| u16::from_radix_10(line.as_bytes()).0);
    let mut wheel = VecDeque::new();
    wheel.push_back(1);
    for (i, x) in xs.enumerate() {
        if i % 2 == 0 {
            wheel.push_back(x);
        } else {
            wheel.push_front(x);
        }
    }
    let l = wheel.len();
    let i = ((l - 1) / 2 + 2025) % l;
    wheel[i]
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct Item {
    bounds: (u32, u32),
    forward: bool
}

impl Item {
    fn len(&self) -> usize {
        (self.bounds.1 - self.bounds.0 + 1) as _
    }
}

#[inline]
pub fn solve_part2() -> impl Display {
    solve_part23(include_str!("part2.txt"), 20252025)
}

#[inline]
pub fn solve_part3() -> impl Display {
    solve_part23(include_str!("part3.txt"), 202520252025)
}

fn solve_part23(input: &str, turns: usize) -> u32 {
    let xs = input.lines().map(|line| {
        let (a, b) = line.split_once('-').unwrap();
        (u32::from_radix_10(a.as_bytes()).0, u32::from_radix_10(b.as_bytes()).0)
    });
    let mut wheel = VecDeque::new();
    wheel.push_back(Item { bounds: (1, 1), forward: true });

    // Construct the wheel and compute length and target position while doing so;
    // the target position is `turns` indices after the initial `1`, so if we're moving that we
    // should offset the target position as well.
    let mut len = 1usize;
    let mut target = turns;
    for (i, (a, b)) in xs.enumerate() {
        if i % 2 == 0 {
            wheel.push_back(Item { bounds: (a, b), forward: true });
        } else {
            wheel.push_front(Item { bounds: (a, b), forward: false });
            target += (b - a + 1) as usize;
        }
        len += (b - a + 1) as usize;
    }
    target %= len;

    // Iterate over the list, which is in clockwise order, skipping over ranges if our target is
    // not within them.
    let mut i = 0;
    for item in wheel {
        if i + item.len() < target {
            i += item.len();
            continue;
        }
        let remaining = (target - i) as u32;
        if item.forward {
            return item.bounds.0 + remaining;
        } else {
            return item.bounds.1 - remaining;
        }
    }
    unreachable!()
}
