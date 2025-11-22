use std::fmt::Display;

use itertools::Itertools;
use math::vec2;
use rustc_hash::FxHashSet as HashSet;

#[inline]
pub fn solve() -> (impl Display, impl Display, impl Display) {
    (solve_part1(), solve_part2(), solve_part3())
}

#[inline]
pub fn solve_part1() -> impl Display {
    let input = include_str!("part1.txt");
    draw(input);
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
    let _input = include_str!("part3.txt");
    "TODO"
}

mod math;

#[derive(Debug)]
enum Line {
    /// Horizontal line from (x0, y) to (x1, y) inclusive.
    Horizontal { y: i64, x0: i64, x1: i64 },

    /// Vertical line from (x, y0) to (x, y1) inclusive.
    Vertical { x: i64, y0: i64, y1: i64 },
}

impl Line {
    fn extend(&mut self, &math::Vec2(dx, dy): &math::Vec2) {
        match self {
            Line::Horizontal { x0, x1, .. } => {
                debug_assert_eq!(dy, 0);
                debug_assert!(x0 <= x1);
                if dx > 0 {
                    *x1 += dx;
                } else {
                    *x0 += dx;
                }
            }

            Line::Vertical { y0, y1, .. } => {
                debug_assert_eq!(dx, 0);
                debug_assert!(y0 <= y1);
                if dy > 0 {
                    *y1 += dy;
                } else {
                    *y0 += dy;
                }
            }
        }
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

    let mut cursor_pos = vec2(0, -1);
    let mut cursor_dir = vec2(-1, 0);
    let mut cursor_visited = HashSet::default();
    'outer: loop {
        if !cursor_visited.insert(cursor_pos) {
            break;
        }
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

        let mut lh_dir = cursor_dir;
        lh_dir.rotate_left();
        if !walls.contains(&(cursor_pos + lh_dir)) {
            cursor_dir.rotate_left();
        }
    }
    dbg!(cursor_visited.len());

    let (path, len) = pathfinding::prelude::astar(
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
    dbg!(len);

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
