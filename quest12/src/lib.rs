use std::fmt::Display;

use rayon::prelude::*;

struct Grid {
    width: usize,
    cells: Vec<u8>,
}

impl Grid {
    fn new(input: &str) -> Self {
        let width = input.lines().next().unwrap().len();
        let cells = input
            .lines()
            .flat_map(|line| line.trim().bytes())
            .map(|b| b - b'0')
            .collect::<Vec<u8>>();
        Self { width, cells }
    }

    fn height(&self) -> usize {
        self.cells.len() / self.width
    }

    fn idx(&self, x: usize, y: usize) -> usize {
        y * self.width + x
    }
}

#[derive(Clone)]
struct Visited(Vec<u128>);

impl Visited {
    fn empty(for_grid: &Grid) -> Self {
        debug_assert!(for_grid.height() <= 128);
        Self(vec![0; for_grid.width])
    }

    fn insert(&mut self, x: usize, y: usize) -> bool {
        let mask = 1u128 << y;
        if self.0[x] & mask != 0 {
            false
        } else {
            self.0[x] |= mask;
            true
        }
    }

    fn set(&self) -> usize {
        self.0.iter().map(|&col| col.count_ones() as usize).sum()
    }
}

#[inline]
pub fn solve() -> (impl Display, impl Display, impl Display) {
    (solve_part1(), solve_part2(), solve_part3())
}

#[inline]
pub fn solve_part1() -> impl Display {
    let grid = Grid::new(include_str!("part1.txt"));
    count(&grid, vec![(0, 0)])
}

#[inline]
pub fn solve_part2() -> impl Display {
    let grid = Grid::new(include_str!("part2.txt"));
    count(&grid, vec![(0, 0), (grid.width - 1, grid.height() - 1)])
}

#[inline]
pub fn solve_part3() -> impl Display {
    let grid = Grid::new(include_str!("part3.txt"));

    let mut q = Vec::new();

    let mut boom = Visited::empty(&grid);
    for _ in 0..3 {
        let (p, new_boom) = (0..grid.width)
            .flat_map(|x| (0..grid.height()).map(move |y| (x, y)))
            .collect::<Vec<_>>()
            .into_par_iter()
            .map(|p| (p, ignite(&grid, vec![p], boom.clone())))
            .max_by_key(|(_, b)| b.set())
            .unwrap();
        boom = new_boom;
        q.push(p);
    }

    count(&grid, q)
}

fn count(grid: &Grid, q: Vec<(usize, usize)>) -> usize {
    ignite(grid, q, Visited::empty(grid)).set()
}

fn ignite(grid: &Grid, mut q: Vec<(usize, usize)>, mut boom: Visited) -> Visited {
    while let Some((x, y)) = q.pop() {
        if !boom.insert(x, y) {
            continue;
        }
        let size = grid.cells[grid.idx(x, y)];
        q.extend(
            [
                (x.wrapping_add(1), y),
                (x.wrapping_sub(1), y),
                (x, y.wrapping_add(1)),
                (x, y.wrapping_sub(1)),
            ]
            .into_iter()
            .filter(|&(x, y)| x < grid.width && y < grid.height() && grid.cells[grid.idx(x, y)] <= size),
        );
    }

    boom
}
