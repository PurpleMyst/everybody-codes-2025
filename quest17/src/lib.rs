use std::{collections::HashSet, fmt::Display};

use pathfinding::prelude::dijkstra;
use rayon::prelude::*;

#[inline]
pub fn solve() -> (impl Display, impl Display, impl Display) {
    (solve_part1(), solve_part2(), solve_part3())
}

#[inline]
pub fn solve_part1() -> impl Display {
    let input = include_str!("part1.txt");
    let width = input.lines().next().unwrap().len();
    let map = grid::Grid::from_vec(input.lines().flat_map(|line| line.bytes()).collect(), width);

    let volcano_pos = map.indexed_iter().find(|&(_, &cell)| cell == b'@').unwrap().0;

    map.indexed_iter()
        .filter_map(|((y, x), &cell)| {
            (cell.is_ascii_digit() && x.abs_diff(volcano_pos.0).pow(2) + y.abs_diff(volcano_pos.1).pow(2) <= 100)
                .then_some((cell - b'0') as u64)
        })
        .sum::<u64>()
}

#[inline]
pub fn solve_part2() -> impl Display {
    let input = include_str!("part2.txt");
    let width = input.lines().next().unwrap().len();
    let map = grid::Grid::from_vec(input.lines().flat_map(|line| line.bytes()).collect(), width);
    let volcano_pos = map.indexed_iter().find(|&(_, &cell)| cell == b'@').unwrap().0;
    let mut visited = HashSet::new();

    (1..=width / 2)
        .map(|radius| {
            let destruction = within(&map, volcano_pos, radius)
                .filter_map(|(x, y, v)| visited.insert((x, y)).then_some(v))
                .sum::<u64>();
            (destruction, destruction * radius as u64)
        })
        .max_by_key(|it| it.0)
        .unwrap()
        .1
}

fn within(
    map: &grid::Grid<u8>,
    volcano_pos: (usize, usize),
    radius: usize,
) -> impl Iterator<Item = (usize, usize, u64)> {
    map.indexed_iter().filter_map(move |((y, x), &cell)| {
        (cell.is_ascii_digit()
            && x.abs_diff(volcano_pos.0).pow(2) + y.abs_diff(volcano_pos.1).pow(2) <= radius * radius)
            .then_some((x, y, (cell - b'0') as u64))
    })
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct LoopState {
    left: ((usize, usize), bool),
    right: ((usize, usize), bool),
    bottom: ((usize, usize), bool),
}

impl LoopState {
    fn new(left: (usize, usize), right: (usize, usize), bottom: (usize, usize)) -> Self {
        Self {
            left: (left, false),
            right: (right, false),
            bottom: (bottom, false),
        }
    }

    fn add(self, (x, y): (usize, usize)) -> Self {
        Self {
            left: (self.left.0, self.left.1 || x <= self.left.0.0),
            right: (self.right.0, self.right.1 || x >= self.right.0.0),
            bottom: (self.bottom.0, self.bottom.1 || y >= self.bottom.0.1),
        }
    }

    fn done(&self) -> bool {
        self.left.1 && self.right.1 && self.bottom.1
    }
}

#[inline]
pub fn solve_part3() -> impl Display {
    let input = include_str!("part3.txt");
    let width = input.lines().next().unwrap().len();
    let map = grid::Grid::from_vec(input.lines().flat_map(|line| line.bytes()).collect(), width);
    let volcano_pos = map.indexed_iter().find(|&(_, &cell)| cell == b'@').unwrap().0;
    let volcano_pos = (volcano_pos.1, volcano_pos.0);
    let start = map.indexed_iter().find(|&(_, &cell)| cell == b'S').unwrap().0;
    let start = (start.1, start.0);

    (1..width / 2)
        .into_par_iter()
        .find_map_first(|radius| {
            let destroyed: HashSet<(usize, usize)> =
                within(&map, volcano_pos, radius).map(|(x, y, _)| (x, y)).collect();
            let destroyed = &destroyed;
            let map = &map;

            let next_ring = within(&map, volcano_pos, radius + 1)
                .map(|(x, y, _)| (x, y))
                .filter(|p| !destroyed.contains(&p))
                .collect::<Vec<_>>();

            let left = next_ring
                .iter()
                .copied()
                .min_by_key(|&(x, y)| (x as isize - volcano_pos.0 as isize, y as isize - volcano_pos.1 as isize).0)
                .unwrap();
            let right = next_ring
                .iter()
                .copied()
                .max_by_key(|&(x, y)| (x as isize - volcano_pos.0 as isize, y as isize - volcano_pos.1 as isize).0)
                .unwrap();
            let bottom = next_ring
                .iter()
                .copied()
                .max_by_key(|&(x, y)| (x as isize - volcano_pos.0 as isize, y as isize - volcano_pos.1 as isize).1)
                .unwrap();

            let cost = dijkstra(
                &(start.0, start.1, LoopState::new(left, right, bottom)),
                |&(x, y, ls)| {
                    [
                        (x.wrapping_sub(1), y),
                        (x.wrapping_add(1), y),
                        (x, y.wrapping_sub(1)),
                        (x, y.wrapping_add(1)),
                    ]
                    .into_iter()
                    .filter(move |&(x, y)| !destroyed.contains(&(x, y)))
                    .filter_map(move |(x, y)| {
                        Some(match map.get(y, x)? {
                            b @ b'0'..=b'9' => ((x, y, ls.add((x, y))), (b - b'0') as u64),
                            b'S' => ((x, y, ls.add((x, y))), 0),
                            _ => return None,
                        })
                    })
                },
                |&(x, y, ls)| (x, y) == start && ls.done(),
            )
            .map_or(u64::MAX, |(_, cost)| cost);

            (cost < (30 * (radius + 1) as u64)).then_some(cost * radius as u64)
        })
        .unwrap()
}

