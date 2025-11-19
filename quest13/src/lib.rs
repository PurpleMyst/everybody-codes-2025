use std::{collections::VecDeque, fmt::Display};

#[inline]
pub fn solve() -> (impl Display, impl Display, impl Display) {
    (solve_part1(), solve_part2(), solve_part3())
}

#[inline]
pub fn solve_part1() -> impl Display {
    let input = include_str!("part1.txt");
    let xs = input.lines().map(|line| line.parse::<u16>().unwrap());
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
        (a.parse::<u32>().unwrap(), b.parse::<u32>().unwrap())
    });
    let mut wheel = VecDeque::new();
    wheel.push_back(Item { bounds: (1, 1), forward: true });

    let mut len = 1usize;

    for (i, (a, b)) in xs.enumerate() {
        if i % 2 == 0 {
            wheel.push_back(Item { bounds: (a, b), forward: true });
        } else {
            wheel.push_front(Item { bounds: (a, b), forward: false });
        }
        len += (b - a + 1) as usize;
    }

    let mut target = turns;
    for &item in &wheel {
        if item.bounds == (1, 1) {
            debug_assert!(item.forward);
            break;
        }
        target += item.len();
    }
    target %= len;

    let mut i = 0;
    for item in wheel {
        if i + item.len() < target {
            i += item.len();
            continue;
        }
        let remaining = (target - i) as u32;
        if item.forward {
            return item.bounds.0 + remaining
        } else {
            return item.bounds.1 - remaining
        }
    }
    unreachable!()
}
