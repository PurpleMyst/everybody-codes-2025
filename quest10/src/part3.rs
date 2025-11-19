use std::fmt::Display;

use arrayvec::ArrayVec;
use memoize::memoize;
use rustc_hash::FxHashMap;

const WIDTH: u8 = 7;
const HEIGHT: u8 = 6;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct GameState {
    sheep: u64,
    dragon: (u8, u8),
}

fn pos2idx((x, y): (u8, u8)) -> u8 {
    y * WIDTH + x
}

impl GameState {
    fn dragon_moves(&self, hideouts: u64) -> impl Iterator<Item = Self> {
        let (x, y) = self.dragon;
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
        .filter(|&(x, y)| x < WIDTH && y < HEIGHT)
        .map(move |p| {
            let mut new_self = Self { dragon: p, ..*self };
            if self.has_sheep(p) && (hideouts & (1 << pos2idx(p)) == 0) {
                new_self.sheep &= !(1 << pos2idx(p));
            }

            new_self
        })
    }

    fn sheep_moves(&self, hideouts: u64) -> impl Iterator<Item = (Self, Option<(u8, u8)>)> {
        let mut v = ArrayVec::<(Self, Option<(u8, u8)>), 5>::new();

        let mut sheep = self.sheep;
        while sheep != 0 {
            let idx = sheep.trailing_zeros() as u8;
            sheep &= sheep - 1;
            let x = idx % WIDTH;
            let y = idx / WIDTH;
            let mut new_self = *self;
            if y == HEIGHT - 1 || ((hideouts & (1 << pos2idx((x, y + 1))) != 0) || self.dragon != (x, y + 1)) {
                new_self.sheep &= !(1 << pos2idx((x, y)));
                if y != HEIGHT - 1 {
                    new_self.sheep |= 1 << pos2idx((x, y + 1));
                }
                v.push((new_self, Some((x, y + 1))));
            }
        }

        if v.is_empty() {
            v.push((*self, None));
        }

        v.into_iter()
    }

    fn has_sheep(&self, p: (u8, u8)) -> bool {
        self.sheep & (1 << pos2idx(p)) != 0
    }
}

#[inline]
pub fn solve() -> impl Display {
    memoized_flush_move_sequences(); // for benchmarking

    let mut initial_state = GameState {
        sheep: 0,
        dragon: (0, 0),
    };
    let mut hideouts = 0;

    for (y, row) in (0..).zip(include_str!("part3.txt").lines()) {
        for (x, cell) in (0..).zip(row.bytes()) {
            match cell {
                b'S' => initial_state.sheep |= 1 << pos2idx((x, y)),
                b'#' => hideouts |= 1 << pos2idx((x, y)),
                b'D' => initial_state.dragon = (x, y),
                _ => {}
            }
        }
    }

    move_sequences(initial_state, hideouts, true)
}

#[memoize(Ignore: hideouts, CustomHasher: FxHashMap, HasherInit: FxHashMap::default())]
fn move_sequences(state: GameState, hideouts: u64, sheeps_turn: bool) -> u64 {
    if state.sheep == 0 {
        return 1;
    }

    if sheeps_turn {
        state
            .sheep_moves(hideouts)
            .map(|(new_state, maybe_move)| {
                if let Some(p) = maybe_move
                    && ((p.1 >= HEIGHT) || (hideouts & (1 << pos2idx(p)) != 0 && p.0 != 3))
                {
                    // The second condition of the OR represents an assumption: in every column but
                    // the middle one, as soon as hideouts start it's hideouts all the way down,
                    // which means that if a sheep gets into an hideout it can escape unscathed.
                    return 0;
                }

                move_sequences(new_state, hideouts, false)
            })
            .sum()
    } else {
        state
            .dragon_moves(hideouts)
            .map(|new_state| move_sequences(new_state, hideouts, true))
            .sum()
    }
}
