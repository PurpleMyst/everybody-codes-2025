use std::{fmt::Display, ops::RangeInclusive};

use itertools::Itertools;
use rustc_hash::FxHashSet as HashSet;

#[inline]
pub fn solve() -> (impl Display, impl Display, impl Display) {
    (solve_part1(), solve_part2(), solve_part3())
}

#[inline]
pub fn solve_part1() -> impl Display {
    // let input = include_str!("part1.txt");
    // draw(input);
    "TODO"
}

#[inline]
pub fn solve_part2() -> impl Display {
    let input = include_str!("part2.txt");
    draw(input);
    "TODO"
}

#[inline]
pub fn solve_part3() -> impl Display {
    // let input = include_str!("part3.txt");
    "TODO"
}

mod math;
use math::*;

fn range(a: i64, b: i64) -> RangeInclusive<i64> {
    a.min(b)..=a.max(b)
}

fn apply(pos: Vec2, delta: Vec2) -> impl Iterator<Item = Vec2> {
    if delta.0 != 0 {
        range(pos.0, pos.0 + delta.0).map(Box::new(move |x| vec2(x, pos.1)) as Box<dyn FnMut(i64) -> Vec2>)
    } else {
        debug_assert_ne!(delta.1, 0);
        range(pos.1, pos.1 + delta.1).map(Box::new(move |y| vec2(pos.0, y)) as Box<dyn FnMut(i64) -> Vec2>)
    }
}

fn draw(input: &'static str) {
    let mut dir = vec2(0, -1);
    let mut end = vec2(0, 0);
    let mut walls = HashSet::default();
    input.trim().split(",").for_each(|bit| {
        let (turn, n) = bit.split_at(1);
        let n = n.parse::<i64>().unwrap();
        match turn {
            "R" => dir.rotate_right(),
            "L" => dir.rotate_left(),
            _ => unreachable!(),
        };
        for _ in 0..n {
            end += dir;
            walls.insert(end);
        }
    });

    let (cursor_visited, mut segments) = follow_wall(end, &walls);
    dbg!(cursor_visited.len());

    println!("{}: {segments:?}", segments.len());
    'foo: loop {
        let mut pos = vec2(0, -1);
        for (i, w) in segments.windows(3).enumerate() {
            pos += w[0];
            if w[0].same_dir(w[2])
                && apply(pos, w[2]).all(|step| !walls.contains(&step))
                && apply(pos + w[2], w[1]).all(|step| !walls.contains(&step))
            {
                eprintln!("MERGING {w:?}");
                let a = w[2];
                segments[i] += a;
                segments.remove(i + 2);

                for i in (1..segments.len()).rev() {
                    if segments[i].same_dir(segments[i - 1]) {
                        let k = segments[i];
                        segments[i - 1] += k;
                        segments.remove(i);
                    }
                }

                continue 'foo;
            }
        }
        break;
    }
    println!("{}: {segments:?}", segments.len());
    println!("TOTAL STEPS = {}", segments.iter().map(|v| v.mag()).sum::<i64>());

    let mut cursor_visited = HashSet::default();
    let mut cursor = vec2(0, -1);
    for dir in segments.into_iter() {
        for foo in apply(cursor, dir) {
            cursor_visited.insert(foo);
        }
        cursor += dir;
    }
    println!("calculated new cursor path");

    let (path, _len) = pathfinding::prelude::astar(
        &vec2(0, 0),
        |&math::Vec2(cx, cy)| {
            [vec2(cx - 1, cy), vec2(cx + 1, cy), vec2(cx, cy - 1), vec2(cx, cy + 1)]
                .into_iter()
                .filter(|&n| end == n || !walls.contains(&n))
                .map(|n| (n, 1))
        },
        |&c| c.dist(&end),
        |&p| p == end,
    )
    .unwrap();
    println!("got A* path");
    let path = path.into_iter().collect::<HashSet<_>>();

    let (x_min, x_max) = walls.iter().map(|&math::Vec2(x, _)| x).minmax().into_option().unwrap();
    let (y_min, y_max) = walls.iter().map(|&math::Vec2(_, y)| y).minmax().into_option().unwrap();
    let mut f = std::fs::File::create(&format!("{}_{}.ppm", end.0, end.1)).unwrap();
    use std::io::Write;
    writeln!(f, "P3").unwrap();
    writeln!(f, "# everybody codes!").unwrap();
    writeln!(f, "{} {}", x_max - x_min + 1, y_max - y_min + 1).unwrap();
    writeln!(f, "255").unwrap();
    for y in y_min..=y_max {
        for x in x_min..=x_max {
            let is_start_or_end = (x, y) == (0, 0) || vec2(x, y) == end;

            let is_wall = walls.contains(&vec2(x, y));

            let on_path = path.contains(&vec2(x, y));
            let on_lh = cursor_visited.contains(&vec2(x, y));

            const BLACK: (u8, u8, u8) = (0x00, 0x00, 0x00);
            const DARK_PURPLE: (u8, u8, u8) = (0x7E, 0x25, 0x53);
            const DARK_GREEN: (u8, u8, u8) = (0x00, 0x87, 0x51);
            const LIGHT_GRAY: (u8, u8, u8) = (0xC2, 0xC3, 0xC7);
            const RED: (u8, u8, u8) = (0xFF, 0x00, 0x4D);

            let (r, g, b) = match (is_start_or_end, is_wall, on_path, on_lh) {
                (true, ..) => RED,
                (false, true, ..) => BLACK,
                (false, false, true, true) => DARK_PURPLE,
                (false, false, true, false) => DARK_GREEN,
                (false, false, false, true) => RED,
                (false, false, false, false) => LIGHT_GRAY,
            };

            write!(f, "{r} {g} {b} ").unwrap();
        }
        writeln!(f).unwrap();
    }
}

fn follow_wall(end: math::Vec2, walls: &HashSet<math::Vec2>) -> (HashSet<math::Vec2>, Vec<math::Vec2>) {
    let mut cursor_pos = vec2(0, -1);
    let mut cursor_dir = vec2(-1, 0);
    let mut cursor_visited = HashSet::default();
    let mut segments: Vec<math::Vec2> = Vec::new();
    'outer: loop {
        assert!(cursor_visited.insert(cursor_pos));
        if cursor_pos.1 == end.1 {
            let beeline_dir = if cursor_pos.0 < end.0 {
                vec2(1, 0)
            } else if cursor_pos.0 > end.0 {
                vec2(-1, 0)
            } else {
                vec2(0, 0)
            };
            let old_cursor_pos = cursor_pos;
            while !walls.contains(&cursor_pos) {
                cursor_pos += beeline_dir;
                if cursor_pos == end {
                    break 'outer;
                }
                if let Some(segment) = segments.last_mut()
                    && segment.0.signum() == beeline_dir.0.signum()
                    && segment.1.signum() == beeline_dir.1.signum()
                {
                    segment.0 += beeline_dir.0;
                    segment.1 += beeline_dir.1;
                } else {
                    segments.push(beeline_dir);
                }
                cursor_visited.insert(cursor_pos);
            }
            cursor_pos = old_cursor_pos;
        }

        if walls.contains(&(cursor_pos + cursor_dir)) {
            let left_dir = {
                let mut d = cursor_dir;
                d.rotate_left();
                d
            };
            if walls.contains(&(cursor_pos + left_dir)) {
                cursor_dir.rotate_right();
            } else {
                cursor_dir.rotate_left();
            }
        }
        cursor_pos += cursor_dir;
        if let Some(segment) = segments.last_mut()
            && segment.0.signum() == cursor_dir.0.signum()
            && segment.1.signum() == cursor_dir.1.signum()
        {
            segment.0 += cursor_dir.0;
            segment.1 += cursor_dir.1;
        } else {
            segments.push(cursor_dir);
        }

        let mut lh_dir = cursor_dir;
        lh_dir.rotate_left();
        if !walls.contains(&(cursor_pos + lh_dir)) {
            cursor_dir.rotate_left();
        }
    }
    (cursor_visited, segments)
}
