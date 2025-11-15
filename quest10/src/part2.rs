use std::{fmt::Display, mem::swap};

use rustc_hash::FxHashSet as HashSet;

const SIDE: usize = 101;

struct Board {
    /// per-row bitmask of sheep
    sheep: [u128; SIDE],
    hideouts: [u128; SIDE],

    sheep_offset: usize,
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
        .filter(|&(x, y)| x < SIDE && y < SIDE)
    }

    fn eat_sheep(&self, (x, y): (usize, usize)) -> Option<(usize, usize)> {
        if self.hideouts[y] & (1 << x) != 0 {
            return None;
        }

        let actual_y = y.wrapping_sub(self.sheep_offset);
        if actual_y > SIDE {
            return None;
        }
        (self.sheep[actual_y] & (1 << x) != 0).then_some((x, actual_y))
    }
}

#[inline]
pub fn solve() -> impl Display {
    let mut dragon = None;
    let mut sheep = [0u128; SIDE];
    let mut hideouts = [0u128; SIDE];
    for (y, row) in include_str!("part2.txt").lines().enumerate() {
        for (x, cell) in row.bytes().enumerate() {
            match cell {
                b'S' => sheep[y] |= 1 << x,
                b'#' => hideouts[y] |= 1 << x,
                b'D' => dragon = Some((x, y)),
                _ => {}
            }
        }
    }
    let mut board = Board {
        sheep,
        hideouts,
        sheep_offset: 0,
    };

    let mut states = HashSet::default();
    let mut next_states = states.clone();
    let mut eaten = [0u128; SIDE];
    states.insert(dragon.unwrap());
    for _ in 0..20 {
        next_states.extend(states.drain().flat_map(|d| board.dragon_moves(d)));
        next_states
            .iter()
            .filter_map(|&p| board.eat_sheep(p))
            .for_each(|(x, y)| {
                eaten[y] |= 1 << x;
            });
        board.sheep_offset += 1;
        next_states
            .iter()
            .filter_map(|&p| board.eat_sheep(p))
            .for_each(|(x, y)| {
                eaten[y] |= 1 << x;
            });
        swap(&mut states, &mut next_states);
    }

    eaten.iter().map(|&row| row.count_ones()).sum::<u32>()
}
