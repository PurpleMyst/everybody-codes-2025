use std::{cmp::Reverse, fmt::Display};

use rayon::prelude::*;

#[inline]
pub fn solve() -> (impl Display, impl Display, impl Display) {
    (solve_part1(), solve_part2(), solve_part3())
}

#[inline]
pub fn solve_part1() -> impl Display {
    let input = include_str!("part1.txt");
    let width = input.lines().next().unwrap().len();
    let cells = input
        .lines()
        .flat_map(|line| line.trim().bytes())
        .map(|b| b - b'0')
        .collect::<Vec<u8>>();
    count(width, &cells, vec![(0, 0)])
}

#[inline]
pub fn solve_part2() -> impl Display {
    let input = include_str!("part2.txt");
    let width = input.lines().next().unwrap().len();
    let cells = input
        .lines()
        .flat_map(|line| line.trim().bytes())
        .map(|b| b - b'0')
        .collect::<Vec<u8>>();
    count(width, &cells, vec![(0, 0), (width - 1, cells.len() / width - 1)])
}

#[inline]
pub fn solve_part3() -> impl Display {
    let input = include_str!("part3.txt");
    let width = input.lines().next().unwrap().len();
    let cells = input
        .lines()
        .flat_map(|line| line.trim().bytes())
        .map(|b| b - b'0')
        .collect::<Vec<u8>>();
    let height = cells.len() / width;

    let mut q = Vec::new();

    let mut boom = vec![false; cells.len()];

    for _ in 0..3 {
        let (p, new_boom) = (0..width)
            .flat_map(|x| (0..height).map(move |y| (x, y)))
            .collect::<Vec<_>>()
            .into_par_iter()
            .map(|p| (p, ignite(width, &cells, vec![p], boom.clone())))
            .max_by_key(|(_, b)| set(&b))
            .unwrap();
        boom = new_boom;
        q.push(p);
    }

    count(width, &cells, q)
}

fn set(x: &[bool]) -> usize {
    x.into_iter().filter(|&&x| x).count()
}

fn count(width: usize, cells: &[u8], q: Vec<(usize, usize)>) -> usize {
    set(&ignite(width, cells, q, vec![false; cells.len()]))
}

fn ignite(width: usize, cells: &[u8], mut q: Vec<(usize, usize)>, mut boom: Vec<bool>) -> Vec<bool> {
    let height = cells.len() / width;

    while let Some((x, y)) = q.pop() {
        if boom[y * width + x] {
            continue;
        }
        boom[y * width + x] = true;

        let size = cells[y * width + x];
        q.extend(
            [
                (x.wrapping_add(1), y),
                (x.wrapping_sub(1), y),
                (x, y.wrapping_add(1)),
                (x, y.wrapping_sub(1)),
            ]
            .into_iter()
            .filter(|&(x, y)| x < width && y < height && cells[y * width + x] <= size),
        );
    }

    boom
}
