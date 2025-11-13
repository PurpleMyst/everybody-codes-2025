use std::fmt::Display;

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

    fn intersect(&self, other: &Self) -> Option<(f64, f64)> {
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
            return None;
        }

        let x_num = (x1 * y2 - y1 * x2) * (x3 - x4) - (x1 - x2) * (x3 * y4 - y3 * x4);
        let y_num = (x1 * y2 - y1 * x2) * (y3 - y4) - (y1 - y2) * (x3 * y4 - y3 * x4);

        Some((x_num / denum, y_num / denum))
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
    let mut count = 0;
    let mut lines = Vec::new();

    for cur_idx in it {
        let prev_point = idx_to_point(prev_idx);
        let cur_point = idx_to_point(cur_idx);
        let line = Line::new(prev_point, cur_point);
        for other_line in &lines {
            if let Some((x, y)) = line.intersect(other_line)
                && x.hypot(y) < 1.0 - 1e-12
            {
                count += 1;
            }
        }

        prev_idx = cur_idx;
        lines.push(line);
    }

    count
}

#[inline]
pub fn solve_part3() -> impl Display {
    const NAILS: usize = 256;
    let input = include_str!("part3.txt").trim();
    let mut it = input.split(',').map(|n| n.parse::<usize>().unwrap());

    let idx_to_point = |idx| {
        let theta = idx as f64 * std::f64::consts::TAU / NAILS as f64;
        (theta.cos(), theta.sin())
    };

    let mut prev_idx = it.next().unwrap();
    let mut lines = Vec::new();
    let mut hit = [[false; NAILS + 1]; NAILS + 1];

    for cur_idx in it {
        let prev_point = idx_to_point(prev_idx);
        let cur_point = idx_to_point(cur_idx);
        let line = Line::new(prev_point, cur_point);
        lines.push(line);
        hit[prev_idx][cur_idx] = true;
        hit[cur_idx][prev_idx] = true;
        prev_idx = cur_idx;
    }

    (1..=NAILS).into_par_iter().map(|i| {
        (i + 1..=NAILS).map(|j| {
            let i_point = idx_to_point(i);
            let j_point = idx_to_point(j);
            let strike = Line::new(i_point, j_point);

            hit[i][j] as usize
                + lines
                    .iter()
                    .filter(|thread| {
                        let Some((x, y)) = thread.intersect(&strike) else {
                            return false;
                        };
                        let d = x.hypot(y);
                        let inside = d < 1.0 - 1e-12;
                        inside
                    })
                    .count()
        }).max().unwrap_or_default()
    }).max().unwrap()
}
