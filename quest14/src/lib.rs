use std::{collections::hash_map::Entry, fmt::Display};

use rustc_hash::FxHashMap as HashMap;

mod floor;

#[inline]
pub fn solve() -> (impl Display, impl Display, impl Display) {
    (solve_part1(), solve_part2(), solve_part3())
}

#[inline]
pub fn solve_part1() -> impl Display {
    let mut floor = floor::Floor::load(include_str!("part1.txt"));
    (0..10)
        .map(|_| {
            floor.step();
            floor.total_active()
        })
        .sum::<u32>()
}

#[inline]
pub fn solve_part2() -> impl Display {
    let mut floor = floor::Floor::load(include_str!("part2.txt"));
    (0..2025)
        .map(|_| {
            floor.step();
            floor.total_active()
        })
        .sum::<u32>()
}

#[inline]
pub fn solve_part3() -> impl Display {
    const ROUNDS: usize = 1_000_000_000;

    let pattern = floor::Floor::load(include_str!("part3.txt"));

    let mut floor = floor::Floor::empty(34);
    let mut total = 0;

    let mut seen = HashMap::default();
    seen.reserve(8192);

    let mut round = 0usize;
    let mut sofar = Vec::with_capacity(8192);
    'mainloop: loop {
        match seen.entry(floor.clone()) {
            Entry::Occupied(occupied_entry) => {
                let remaining = ROUNDS - round;
                let cycle_start = *occupied_entry.get();
                let cycle_len = round - cycle_start;
                total *= (1 + remaining / cycle_len) as u32;
                total += (sofar[cycle_start + (remaining % cycle_len)] - sofar[cycle_start]) as u32;
                return total;
            }
            Entry::Vacant(vacant_entry) => {
                vacant_entry.insert(round);
            }
        }

        floor.step();
        round += 1;

        let offset = floor.side / 2 - pattern.side / 2;
        for (&pattern_row, &floor_row) in pattern.active.iter().zip(floor.active.iter().skip(offset)) {
            if pattern_row.reverse_bits() != ((floor_row.reverse_bits() >> offset) & ((1 << pattern.side) - 1)) {
                sofar.push(total);
                continue 'mainloop;
            }
        }

        total += floor.total_active();
        sofar.push(total);
    }
}
