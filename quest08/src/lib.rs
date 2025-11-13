use std::fmt::Display;

use intmap::IntMap;
use rayon::prelude::*;

#[inline]
pub fn solve() -> (impl Display, impl Display, impl Display) {
    (solve_part1(), solve_part2(), solve_part3())
}

#[inline]
pub fn solve_part1() -> impl Display {
    const NAILS: usize = 32;
    let input = include_str!("part1.txt").trim();
    let mut it = input.split(',').map(|n| n.parse::<usize>().unwrap());

    let mut prev_idx = it.next().unwrap();
    let mut count = 0;
    for cur_idx in it {
        if cur_idx.abs_diff(prev_idx) == NAILS / 2 {
            count += 1;
        }

        prev_idx = cur_idx;
    }

    count
}

struct Line {
    x1: f64,
    y1: f64,
    x2: f64,
    y2: f64,
}

impl Line {
    fn new((x1, y1): (f64, f64), (x2, y2): (f64, f64)) -> Self {
        Self { x1, y1, x2, y2 }
    }

    fn intersects(&self, other: &Self) -> bool {
        let x1 = self.x1;
        let x2 = self.x2;
        let x3 = other.x1;
        let x4 = other.x2;
        let y1 = self.y1;
        let y2 = self.y2;
        let y3 = other.y1;
        let y4 = other.y2;

        let denum = (x1 - x2) * (y3 - y4) - (y1 - y2) * (x3 - x4);
        if denum == 0.0 {
            return false;
        }

        let x_num = (x1 * y2 - y1 * x2) * (x3 - x4) - (x1 - x2) * (x3 * y4 - y3 * x4);
        let y_num = (x1 * y2 - y1 * x2) * (y3 - y4) - (y1 - y2) * (x3 * y4 - y3 * x4);

        x_num * x_num + y_num * y_num < (1.0 - 1e-12) * denum.abs() * denum.abs()
    }
}

#[inline]
pub fn solve_part2() -> impl Display {
    const NAILS: usize = 256;
    let input = include_str!("part2.txt").trim();
    let mut it = input.split(',').map(|n| n.parse::<usize>().unwrap());

    let idx_to_point = |idx| {
        let theta = idx as f64 * std::f64::consts::TAU / NAILS as f64;
        (theta.cos(), theta.sin())
    };

    let mut prev_idx = it.next().unwrap();
    let lines = it
        .map(|cur_idx| {
            let prev_point = idx_to_point(prev_idx);
            let cur_point = idx_to_point(cur_idx);
            let line = Line::new(prev_point, cur_point);
            prev_idx = cur_idx;
            line
        })
        .collect::<Vec<_>>();

    lines
        .par_iter()
        .enumerate()
        .map(|(i, line_i)| lines[..i].iter().filter(|line_j| line_i.intersects(line_j)).count())
        .sum::<usize>()
}

#[inline]
pub fn solve_part3() -> impl Display {
    const NAILS: usize = 256;
    let input = include_str!("part3.txt").trim();
    let mut it = input.split(',').map(|n| n.parse::<usize>().unwrap());
    let mut adjacency = IntMap::<usize, Vec<usize>>::default();
    let mut a = it.next().unwrap();
    for b in it {
        adjacency.entry(a).or_default().push(b);
        adjacency.entry(b).or_default().push(a);
        a = b;
    }

    // approach birthed from bob.oblong on the discord
    (1..NAILS)
        .into_par_iter()
        .map(|a| {
            let mut cuts = 0u16;
            (a + 2..NAILS)
                .map(|b| {
                    cuts -= adjacency
                        .get(b)
                        .unwrap()
                        .iter()
                        .filter(|&&c| a < c && c < b - 1)
                        .count() as u16;
                    cuts += adjacency
                        .get(b - 1)
                        .unwrap()
                        .iter()
                        .filter(|&&c| !(a <= c && c < b + 1))
                        .count() as u16;
                    cuts + adjacency.get(b).unwrap().contains(&a) as u16
                })
                .max()
                .unwrap_or_default()
        })
        .max()
        .unwrap()
}
