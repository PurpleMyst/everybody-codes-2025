use std::fmt::Display;

use memoize::memoize;
use rustc_hash::FxHashMap;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct GameState {
    hideouts: u64,
    sheep: u64,

    width: u8,
    height: u8,

    dragon: (u8, u8),
}

#[derive(Clone, Copy, Debug)]
enum Actor {
    Sheep,
    Dragon,
}

#[derive(Clone, Copy, Debug)]
struct Move {
    row: u8,
    col: u8,
    actor: Actor,
}

impl Display for Move {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}>{}{}",
            match self.actor {
                Actor::Sheep => 'S',
                Actor::Dragon => 'D',
            },
            "ABCDEF".chars().nth(self.col as usize).unwrap(),
            self.row + 1
        )?;
        Ok(())
    }
}

impl Display for GameState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.height {
            for x in 0..self.width {
                if self.is_hideout((x, y)) {
                    write!(f, "\x1b[32m")?;
                }

                if self.dragon == (x, y) {
                    write!(f, "D")?;
                } else if self.has_sheep((x, y)) {
                    write!(f, "S")?;
                } else if self.is_hideout((x, y)) {
                    write!(f, "#")?;
                } else {
                    write!(f, ".")?;
                }

                write!(f, "\x1b[0m")?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl GameState {
    fn dragon_moves(&self) -> impl Iterator<Item = (Self, Move)> {
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
        .filter(|&(x, y)| x < self.width && y < self.height)
        .map(|p| {
            let mut new_self = Self { dragon: p, ..*self };
            if self.has_sheep(p) && !self.is_hideout(p) {
                new_self.sheep &= !(1 << self.pos2idx(p));
            }

            (
                new_self,
                Move {
                    row: p.1,
                    col: p.0,
                    actor: Actor::Dragon,
                },
            )
        })
    }

    fn sheep_moves(&self) -> impl Iterator<Item = (Self, Option<Move>)> {
        let mut v = Vec::new();

        for y in 0..self.height {
            for x in 0..self.width {
                if !self.has_sheep((x, y)) {
                    continue;
                }
                let mut new_self = *self;
                if y == self.height - 1 || (self.is_hideout((x, y + 1)) || self.dragon != (x, y + 1)) {
                    new_self.sheep &= !(1 << self.pos2idx((x, y)));

                    if y != self.height - 1 {
                        new_self.sheep |= 1 << self.pos2idx((x, y + 1));
                    }
                    v.push((
                        new_self,
                        Some(Move {
                            row: y + 1,
                            col: x,
                            actor: Actor::Sheep,
                        }),
                    ));
                }
            }
        }
        if v.is_empty() {
            v.push((*self, None));
        }

        v.into_iter()
    }

    fn pos2idx(&self, (x, y): (u8, u8)) -> u8 {
        y * self.width + x
    }

    fn has_sheep(&self, p: (u8, u8)) -> bool {
        self.sheep & (1 << self.pos2idx(p)) != 0
    }

    fn is_hideout(&self, p: (u8, u8)) -> bool {
        self.hideouts & (1 << self.pos2idx(p)) != 0
    }
}

#[inline]
pub fn solve() -> impl Display {
    memoized_flush_move_sequences(); // for benchmarking

    let width = include_str!("part3.txt").lines().next().unwrap().trim().len() as _;
    let height = include_str!("part3.txt").lines().count() as _;

    let mut board = GameState {
        width,
        height,
        sheep: 0,
        hideouts: 0,
        dragon: (0, 0),
    };

    for (y, row) in (0..).zip(include_str!("part3.txt").lines()) {
        for (x, cell) in (0..).zip(row.bytes()) {
            match cell {
                b'S' => board.sheep |= 1 << board.pos2idx((x, y)),
                b'#' => board.hideouts |= 1 << board.pos2idx((x, y)),
                b'D' => board.dragon = (x, y),
                _ => {}
            }
        }
    }

    move_sequences(board, true)
}

#[memoize(CustomHasher: FxHashMap, HasherInit: FxHashMap::default())]
fn move_sequences(board: GameState, sheeps_turn: bool) -> u64 {
    if board.sheep == 0 {
        return 1;
    }

    if sheeps_turn {
        board
            .sheep_moves()
            .map(|(new_board, maybe_move)| {
                if let Some(m) = maybe_move
                    && ((m.row >= board.height) || (new_board.is_hideout((m.col, m.row)) && m.col != 3))
                {
                    // The second condition of the OR represents an assumption: in every column but
                    // the middle one, as soon as hideouts start it's hideouts all the way down,
                    // which means that if a sheep gets into an hideout it can escape unscathed.
                    return 0;
                }

                move_sequences(new_board, false)
            })
            .sum()
    } else {
        board
            .dragon_moves()
            .map(|(new_board, _)| move_sequences(new_board, true))
            .sum()
    }
}
