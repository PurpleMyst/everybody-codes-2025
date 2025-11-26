use std::fmt::Display;

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
    let dist_map = grid::Grid::from_vec(
        map.indexed_iter()
            .map(|((y, x), _)| x.abs_diff(volcano_pos.0).pow(2) + y.abs_diff(volcano_pos.1).pow(2))
            .collect::<Vec<usize>>(),
        width,
    );

    (1..=width / 2)
        .map(|radius| {
            let destruction = dist_map
                .indexed_iter()
                .filter_map(|((x, y), &d)| {
                    (d != 0 && d > (radius - 1).pow(2) && d <= radius.pow(2)).then(|| (map[(y, x)] - b'0') as u64)
                })
                .sum::<u64>();
            (destruction, destruction * radius as u64)
        })
        .max_by_key(|it| it.0)
        .unwrap()
        .1
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct LoopState {
    left: usize,
    right: usize,
    bottom: usize,
    flags: u8,
}

impl LoopState {
    fn new(left: usize, right: usize, bottom: usize) -> Self {
        Self {
            left,
            right,
            bottom,
            flags: 0,
        }
    }

    fn add(self, (x, y): (usize, usize)) -> Self {
        Self {
            flags: if x <= self.left {
                self.flags | 1
            } else if x >= self.right {
                self.flags | 2
            } else if y >= self.bottom {
                self.flags | 4
            } else {
                self.flags
            },
            ..self
        }
    }

    fn done(&self) -> bool {
        self.flags.count_ones() == 3
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

    let dist_map = grid::Grid::from_vec(
        map.indexed_iter()
            .map(|((y, x), _)| x.abs_diff(volcano_pos.0).pow(2) + y.abs_diff(volcano_pos.1).pow(2))
            .collect::<Vec<usize>>(),
        width,
    );

    (1..width / 2)
        .into_par_iter()
        .find_map_first(|radius| {
            let map = &map;
            let dist_map = &dist_map;

            let mut left = usize::MAX;
            let mut right = usize::MIN;
            let mut bottom = usize::MIN;
            dist_map
                .indexed_iter()
                .filter_map(|((x, y), &d)| (d <= (radius + 1) * (radius + 1)).then_some((x, y)))
                .for_each(|(x, y)| {
                    left = left.min(x);
                    right = right.max(x);
                    bottom = bottom.max(y);
                });

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
                    .filter(move |&(x, y)| dist_map.get(y, x).is_some_and(|&d| d > radius.pow(2)))
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

            (cost < (30 * (radius + 1) as u64)).then_some(cost.wrapping_mul(radius as u64))
        })
        .unwrap()
}

