use std::{fmt::Display, iter::once, mem::swap};

use rustc_hash::FxHashSet as HashSet;

const SIDE: u8 = 21;

struct Board {
    /// per-row bitmask of sheep
    sheep: [u32; SIDE as usize],
}

impl Board {
    fn dragon_moves(&self, (x, y): (u8, u8)) -> impl Iterator<Item = (u8, u8)> {
        [
            (x.wrapping_sub(2), y.wrapping_sub(1)),
            (x.wrapping_sub(1), y.wrapping_sub(2)),
            (x.wrapping_add(2), y.wrapping_add(1)),
            (x.wrapping_add(1), y.wrapping_add(2)),
            (x.wrapping_sub(2), y.wrapping_add(1)),
            (x.wrapping_sub(1), y.wrapping_add(2)),
            (x.wrapping_add(2), y.wrapping_sub(1)),
            (x.wrapping_add(1), y.wrapping_sub(2)),
        ]
        .into_iter()
        .filter(|&(x, y)| x < SIDE && y < SIDE)
    }

    fn has_sheep(&self, (x, y): (u8, u8)) -> bool {
        self.sheep[y as usize] & (1 << x) != 0
    }
}

#[inline]
pub fn solve() -> impl Display {
    let mut dragon = (0, 0);
    let mut sheep = [0u32; SIDE as usize];
    for (y, row) in include_str!("part1.txt").lines().enumerate() {
        for (x, cell) in row.bytes().enumerate() {
            match cell {
                b'S' => sheep[y] |= 1 << x,
                b'D' => dragon = (x as u8, y as u8),
                _ => {}
            }
        }
    }
    let board = Board { sheep };

    let mut states = HashSet::default();
    let mut next_states = states.clone();
    states.insert(dragon);
    for _ in 0..4 {
        next_states.extend(states.drain().flat_map(|d| once(d).chain(board.dragon_moves(d))));
        swap(&mut states, &mut next_states);
    }

    states.into_iter().filter(|&p| board.has_sheep(p)).count()
}
