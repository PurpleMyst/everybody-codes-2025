use std::{cmp::Reverse, fmt::Display, iter::once};

use atoi::FromRadix10;
use itertools::Itertools;

struct SpineSegment {
    value: u8,
    left: Option<u8>,
    right: Option<u8>,
}

impl SpineSegment {
    fn new(value: u8) -> Self {
        Self {
            value,
            left: None,
            right: None,
        }
    }
}

struct Sword {
    id: usize,
    spine: Vec<SpineSegment>,
}

impl Display for Sword {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for segment in &self.spine {
            if let Some(left) = segment.left {
                write!(f, "{:2}-", left)?;
            } else {
                write!(f, "   ")?;
            }
            write!(f, "{:2}", segment.value)?;
            if let Some(right) = segment.right {
                write!(f, "-{:<2}", right)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl Sword {
    fn empty(id: usize) -> Self {
        Self { id, spine: vec![] }
    }

    fn new(id: usize, numbers: impl Iterator<Item = u8>) -> Self {
        let mut s = Self::empty(id);
        for n in numbers {
            s.push(n);
        }
        s
    }

    fn push(&mut self, value: u8) {
        for segment in &mut self.spine {
            if value < segment.value && segment.left.is_none() {
                segment.left = Some(value);
                return;
            }
            if value > segment.value && segment.right.is_none() {
                segment.right = Some(value);
                return;
            }
        }
        self.spine.push(SpineSegment::new(value))
    }

    fn quality(&self) -> u64 {
        self.spine
            .iter()
            .map(|segment| segment.value)
            .fold(0, |acc, n| 10 * acc + n as u64)
    }

    fn levels(&self) -> impl Iterator<Item = u16> {
        self.spine.iter().map(|segment| {
            segment
                .left
                .into_iter()
                .chain(once(segment.value))
                .chain(segment.right.into_iter())
                .fold(0, |acc, n| 10 * acc + n as u16)
        })
    }
}

#[inline]
pub fn solve() -> (impl Display, impl Display, impl Display) {
    (solve_part1(), solve_part2(), solve_part3())
}

#[inline]
pub fn solve_part1() -> impl Display {
    let input = include_str!("part1.txt");
    let (id, numbers) = parse(input).next().unwrap();
    Sword::new(id, numbers).quality()
}

fn parse(input: &str) -> impl Iterator<Item = (usize, impl Iterator<Item = u8>)> {
    input.lines().map(|line| {
        let (id, nums) = line.split_once(":").unwrap();
        (
            usize::from_radix_10(id.as_bytes()).0,
            nums.split(",").map(|n| u8::from_radix_10(n.as_bytes()).0),
        )
    })
}

#[inline]
pub fn solve_part2() -> impl Display {
    let input = include_str!("part2.txt").trim();
    let swords = parse(input);
    let (min, max) = swords
        .map(|(id, ns)| Sword::new(id, ns).quality())
        .minmax()
        .into_option()
        .unwrap();
    max - min
}

#[inline]
pub fn solve_part3() -> impl Display {
    let input = include_str!("part3.txt").trim();
    let swords = parse(input);
    let ordered_swords = swords
        .map(|(id, ns)| Sword::new(id, ns))
        .sorted_by_cached_key(|sword| Reverse((sword.quality(), sword.levels().collect_vec(), sword.id)));
    ordered_swords
        .enumerate()
        .map(|(idx, s)| (1 + idx) * s.id)
        .sum::<usize>()
}
