use std::{fmt::Display, iter::once, mem::swap};

use rustc_hash::FxHashSet as HashSet;

struct Board {
    /// per-row bitmask of sheep
    sheep: Vec<u128>,

    width: usize,
    height: usize,
}

impl Board {
    fn dragon_moves(&self, (x, y): (usize, usize)) -> impl Iterator<Item = (usize, usize)> {
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
        .filter(|&(x, y)| x < self.width && y < self.height)
    }

    fn has_sheep(&self, (x, y): (usize, usize)) -> bool {
        self.sheep[y] & (1 << x) != 0
    }
}

#[inline]
pub fn solve() -> impl Display {
    let mut dragon = None;
    let mut sheep = Vec::new();
    let mut width = 0;
    let mut height = 0;
    for (y, row) in include_str!("part1.txt").lines().enumerate() {
        height = y + 1;
        let mut row_sheep = 0;
        for (x, cell) in row.bytes().enumerate() {
            width = x + 1;
            match cell {
                b'S' => row_sheep |= 1 << x,
                b'D' => dragon = Some((x, y)),
                _ => {}
            }
        }
        sheep.push(row_sheep);
    }
    let board = Board { sheep, width, height };

    let mut states = HashSet::default();
    let mut next_states = states.clone();
    states.insert(dragon.unwrap());
    for _ in 0..4 {
        next_states.extend(states.drain().flat_map(|d| once(d).chain(board.dragon_moves(d))));
        swap(&mut states, &mut next_states);
    }

    states.into_iter().filter(|&p| board.has_sheep(p)).count()
}
