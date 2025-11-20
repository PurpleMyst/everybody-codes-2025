use std::{
    collections::{HashMap, HashSet, hash_map::Entry},
    fmt::Display,
};

#[inline]
pub fn solve() -> (impl Display, impl Display, impl Display) {
    (solve_part1(), solve_part2(), solve_part3())
}

#[inline]
pub fn solve_part1() -> impl Display {
    let width = include_str!("part1.txt").lines().next().unwrap().trim().len();
    let height = include_str!("part1.txt").lines().count();

    let mut alive = include_str!("part1.txt")
        .lines()
        .enumerate()
        .flat_map(|(y, row)| {
            row.bytes()
                .enumerate()
                .filter(|&(_, b)| b == b'#')
                .map(move |(x, _)| (x, y))
        })
        .collect::<HashSet<_>>();

    let mut new_alive = HashSet::new();

    let mut total = 0;
    for _ in 0..10 {
        new_alive.clear();
        for y in 0..height {
            for x in 0..width {
                let is_alive = alive.contains(&(x, y));
                let alive_neighbors = [
                    (x.wrapping_sub(1), y.wrapping_sub(1)),
                    (x.wrapping_sub(1), y.wrapping_add(1)),
                    (x.wrapping_add(1), y.wrapping_sub(1)),
                    (x.wrapping_add(1), y.wrapping_add(1)),
                ]
                .into_iter()
                .filter(|&(nx, ny)| alive.contains(&(nx, ny)))
                .count();

                if (is_alive && alive_neighbors % 2 != 0) || (!is_alive && alive_neighbors % 2 == 0) {
                    new_alive.insert((x, y));
                }
            }
        }
        std::mem::swap(&mut alive, &mut new_alive);
        total += alive.len();
    }

    total
}

#[inline]
pub fn solve_part2() -> impl Display {

    let width = include_str!("part2.txt").lines().next().unwrap().trim().len();
    let height = include_str!("part2.txt").lines().count();

    let mut alive = include_str!("part2.txt")
        .lines()
        .enumerate()
        .flat_map(|(y, row)| {
            row.bytes()
                .enumerate()
                .filter(|&(_, b)| b == b'#')
                .map(move |(x, _)| (x, y))
        })
        .collect::<HashSet<_>>();

    let mut new_alive = HashSet::new();

    let mut total = 0;
    for _ in 0..2025 {
        new_alive.clear();
        for y in 0..height {
            for x in 0..width {
                let is_alive = alive.contains(&(x, y));
                let alive_neighbors = [
                    (x.wrapping_sub(1), y.wrapping_sub(1)),
                    (x.wrapping_sub(1), y.wrapping_add(1)),
                    (x.wrapping_add(1), y.wrapping_sub(1)),
                    (x.wrapping_add(1), y.wrapping_add(1)),
                ]
                .into_iter()
                .filter(|&(nx, ny)| alive.contains(&(nx, ny)))
                .count();

                if (is_alive && alive_neighbors % 2 != 0) || (!is_alive && alive_neighbors % 2 == 0) {
                    new_alive.insert((x, y));
                }
            }
        }
        std::mem::swap(&mut alive, &mut new_alive);
        total += alive.len();
    }

    total
}

#[inline]
pub fn solve_part3() -> impl Display {
    let width = 34usize;
    let height = 34usize;
    let pattern_width = include_str!("part3.txt").lines().next().unwrap().trim().len();
    let pattern_height = include_str!("part3.txt").lines().count();
    let pattern = include_str!("part3.txt")
        .lines()
        .enumerate()
        .flat_map(|(y, row)| {
            row.bytes()
                .enumerate()
                .filter(|&(_, b)| b == b'#')
                .map(move |(x, _)| (x, y))
        })
        .collect::<HashSet<_>>();

    let mut alive = HashSet::new();
    let mut new_alive = HashSet::new();

    let mut total = 0;

    let mut seen = HashMap::new();

    let mut round = 0usize;
    let mut cycle_hit = false;
    'mainloop: while round < 1_000_000_000 {
        if !cycle_hit {
            match seen.entry(alive.iter().copied().collect::<Vec<_>>()) {
                Entry::Occupied(occupied_entry) => {
                    let remaining = 1_000_000_000 - round;
                    let cycle_len = round - occupied_entry.get();
                    total *= 1 + remaining / cycle_len;
                    round = 1000000000 - (remaining % cycle_len);
                    cycle_hit = true;
                }
                Entry::Vacant(vacant_entry) => {
                    vacant_entry.insert(round);
                }
            }
        }

        new_alive.clear();
        for y in 0..height {
            for x in 0..width {
                let is_alive = alive.contains(&(x, y));
                let alive_neighbors = [
                    (x.wrapping_sub(1), y.wrapping_sub(1)),
                    (x.wrapping_sub(1), y.wrapping_add(1)),
                    (x.wrapping_add(1), y.wrapping_sub(1)),
                    (x.wrapping_add(1), y.wrapping_add(1)),
                ]
                .into_iter()
                .filter(|&(nx, ny)| alive.contains(&(nx, ny)))
                .count();

                if (is_alive && alive_neighbors % 2 != 0) || (!is_alive && alive_neighbors % 2 == 0) {
                    new_alive.insert((x, y));
                }
            }
        }
        std::mem::swap(&mut alive, &mut new_alive);
        round += 1;

        let offset = width/2 - pattern_width/2;
        for y in 0..pattern_height {
            for x in 0..pattern_width {
                if pattern.contains(&(x, y)) != alive.contains(&(offset + x, offset + y)) {
                    continue 'mainloop;
                }
            }
        }

        total += alive.len();
    }

    total
}

