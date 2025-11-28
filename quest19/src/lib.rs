use std::fmt::Display;

use itertools::Itertools;
use pathfinding::prelude::astar;

#[derive(Hash, Copy, Clone, PartialEq, Eq)]
struct State {
    x: usize,
    y: usize,
}

impl std::fmt::Debug for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        (self.x, self.y).fmt(f)
    }
}

impl State {
    fn done(&self, walls: &[(usize, usize, usize)]) -> bool {
        self.x == walls.last().unwrap().0
    }

    fn advance(self, walls: &[(usize, usize, usize)]) -> impl IntoIterator<Item = (Self, usize)> {
        let next_wall = walls.iter().find(|w| w.0 > self.x).unwrap();

        let dx = next_wall.0 - self.x;

        walls.iter().filter(|w| w.0 == next_wall.0).flat_map(move |w| {
            let target_ys = w.1..w.1 + w.2;
            let ok_flaps = (dx.saturating_sub(self.y) + 1) / 2..=dx;

            // y = self.y + 2 * flaps - dx
            // 2*flaps = y - self.y + dx
            target_ys.filter_map(move |y| {
                let a = (y + dx).checked_sub(self.y)?;
                if a % 2 != 0 {
                    return None;
                }
                let flaps = a / 2;
                if !ok_flaps.contains(&flaps) {
                    return None;
                }
                Some((State { x: next_wall.0, y }, flaps))
            })
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
    do_solve(input)
}

fn do_solve(input: &'static str) -> usize {
    let walls = input
        .lines()
        .map(|line| -> (usize, usize, usize) { line.split(',').map(|n| n.parse().unwrap()).collect_tuple().unwrap() })
        .collect::<Vec<_>>();

    let target_x = walls.last().unwrap().0;
    // let target_y = walls.iter().filter(|w| w.0 == target_x).map(|w| w.1).min().unwrap();

    let (_path, cost) = astar(
        &State { x: 0, y: 0 },
        |s| s.advance(&walls),
        |s| target_x - s.x,
        |s| s.done(&walls),
    )
    .unwrap();

    cost
}

#[inline]
pub fn solve_part2() -> impl Display {
    let input = include_str!("part2.txt");
    do_solve(input)
}

#[inline]
pub fn solve_part3() -> impl Display {
    let input = include_str!("part3.txt");
    do_solve(input)
}
