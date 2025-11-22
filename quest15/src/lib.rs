use std::{collections::{HashSet, VecDeque}, fmt::Display};

use itertools::Itertools;

#[inline]
pub fn solve() -> (impl Display, impl Display, impl Display) {
    (solve_part1(), solve_part2(), solve_part3())
}

#[inline]
pub fn solve_part1() -> impl Display {
    let input = include_str!("part1.txt");
    do_solve(input)
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

fn do_solve(input: &'static str) -> usize {
    let mut walls = HashSet::new();
    let mut dir: (i64, i64) = ( 0, -1 );
    let mut pos = (0, 0);
    input.trim().split(",").for_each(|bit| {
        let (turn, n) = bit.split_at(1);
        let n = n.parse::<usize>().unwrap();
        dir = match turn {
            // (x + iy) * i = -y + ix
            "R" => (-dir.1, dir.0),
            // (x + bi) * -i = 
            "L" => (dir.1, -dir.0),
            _ => unreachable!(),
        };
        for _ in 0..n  {
            pos.0 += dir.0;
            pos.1 += dir.1;
            walls.insert(pos);
        }
    });

    let mut q = VecDeque::new();
    q.push_back((0, 0, 0));
    let mut visited = HashSet::new();
    while let Some((cx,cy, d)) = q.pop_front() {
        if (cx, cy) == pos {
            return d;
        }
        if !visited.insert((cx, cy)) {
            continue;
        }

        q.extend(
        [
            (cx - 1, cy),
            (cx + 1, cy),
            (cx, cy-1),
            (cx, cy+1),
        ]
            .into_iter()
            .filter(|n| *n == pos || !walls.contains(n))
            .map(|(nx, ny)| (nx,ny,d+1))
        )
    }
    unreachable!();
}
