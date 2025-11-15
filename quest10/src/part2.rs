use std::{collections::HashSet, fmt::Display, mem::swap};
struct Board {
    /// per-row bitmask of sheep
    sheep: Vec<u128>,
    hideouts: Vec<u128>,

    sheep_offset: usize,

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

    fn eat_sheep(&self, (x, y): (usize, usize)) -> Option<(usize, usize)> {
        if self.hideouts[y] & (1 << x) != 0 {
            return None;
        }

        let actual_y = y.wrapping_sub(self.sheep_offset);
        if actual_y > self.height {
            return None;
        }
        (self.sheep[actual_y] & (1 << x) != 0).then_some((x, actual_y))
    }
}

#[inline]
pub fn solve() -> impl Display {
    let mut dragon = None;
    let mut sheep = Vec::new();
    let mut hideouts = Vec::new();
    let mut width = 0;
    let mut height = 0;
    for (y, row) in include_str!("part2.txt").lines().enumerate() {
        height = y + 1;
        let mut row_sheep = 0;
        let mut row_hideouts = 0;
        for (x, cell) in row.bytes().enumerate() {
            width = x + 1;
            match cell {
                b'S' => row_sheep |= 1 << x,
                b'#' => row_hideouts |= 1 << x,
                b'D' => dragon = Some((x, y)),
                _ => {}
            }
        }
        sheep.push(row_sheep);
        hideouts.push(row_hideouts);
    }
    let mut board = Board {
        sheep,
        width,
        height,
        hideouts,
        sheep_offset: 0,
    };

    let mut states = HashSet::new();
    let mut next_states = states.clone();
    let mut meals = HashSet::new();
    states.insert(dragon.unwrap());
    for _ in 0..20 {
        next_states.extend(states.drain().flat_map(|d| board.dragon_moves(d)));

        meals.extend(next_states.iter().filter_map(|&p| board.eat_sheep(p)));
        board.sheep_offset += 1;
        meals.extend(next_states.iter().filter_map(|&p| board.eat_sheep(p)));

        swap(&mut states, &mut next_states);
    }

    meals.len()
}
