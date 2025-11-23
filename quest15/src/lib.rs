use std::{fmt::Display, fs::File, io::Write, ops::RangeInclusive};

use clap::Parser;
use itertools::Itertools;
use pathfinding::prelude::astar;
use rustc_hash::FxHashSet as HashSet;

#[derive(Debug, Parser)]
#[command()]
struct Args {
    #[arg(short, long, help = "Draw the path and wall hug to a PPM file")]
    draw: bool,

    #[arg(short, long, help = "Reduce the wall-hugging path")]
    reduce: bool,
}

#[inline]
pub fn solve() -> (impl Display, impl Display, impl Display) {
    color_eyre::install().unwrap();
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
    do_solve(input);
    "TODO"
}

#[inline]
pub fn solve_part3() -> impl Display {
    // let input = include_str!("part3.txt");
    // draw(input);
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

fn do_solve(input: &'static str) {
    let args = Args::parse();

    let mut dir = vec2(0, -1);
    let mut end = vec2(0, 0);
    let mut wall_cells = HashSet::default();
    let mut wall_lines = Vec::new();
    input.trim().split(",").for_each(|bit| {
        let (turn, n) = bit.split_at(1);
        let n = n.parse::<i64>().unwrap();
        match turn {
            "R" => dir.rotate_right(),
            "L" => dir.rotate_left(),
            _ => unreachable!(),
        };
        wall_lines.push(segment(end, end + dir * n));
        for _ in 0..n {
            end += dir;
            wall_cells.insert(end);
        }
    });

    let (segments, cursor_visited) = follow_wall(end, &wall_cells, &wall_lines);
    println!("INITIAL WALL-FOLLOW STEPS = {}", cursor_visited.len());

    let (segments, cursor_visited) = if args.reduce {
        reduce_segments(&wall_cells, segments, end)
    } else {
        (segments, cursor_visited)
    };
    debug_assert_eq!(segments.iter().fold(vec2(0, -1), |acc, &v| acc + v), end);
    debug_assert_eq!(
        segments.iter().map(|v| v.mag()).sum::<i64>() + 1,
        cursor_visited.len() as i64
    );

    let (path, len) = astar(
        &vec2(0, 0),
        |&Vec2(cx, cy)| {
            [vec2(cx - 1, cy), vec2(cx + 1, cy), vec2(cx, cy - 1), vec2(cx, cy + 1)]
                .into_iter()
                .filter(|&n| end == n || !wall_cells.contains(&n))
                .map(|n| (n, 1))
        },
        |&c| c.dist(&end),
        |&p| p == end,
    )
    .unwrap();
    println!("TOTAL ASTAR STEPS = {len}");

    if args.draw {
        draw(end, wall_cells, cursor_visited, path);
    }
}

fn draw(end: Vec2, walls: HashSet<Vec2>, cursor_visited: HashSet<Vec2>, path: Vec<Vec2>) {
    let shortest_path = path.into_iter().collect::<HashSet<_>>();
    let (x_min, x_max) = walls.iter().map(|&Vec2(x, _)| x).minmax().into_option().unwrap();
    let (y_min, y_max) = walls.iter().map(|&Vec2(_, y)| y).minmax().into_option().unwrap();
    let mut f = File::create(&format!("{}_{}.ppm", end.0, end.1)).unwrap();
    writeln!(f, "P3").unwrap();
    writeln!(f, "# everybody codes!").unwrap();
    writeln!(f, "{} {}", x_max - x_min + 1, y_max - y_min + 1).unwrap();
    writeln!(f, "255").unwrap();
    for y in y_min..=y_max {
        for x in x_min..=x_max {
            let is_start_or_end = (x, y) == (0, 0) || vec2(x, y) == end;

            let is_wall = walls.contains(&vec2(x, y));

            let on_path = shortest_path.contains(&vec2(x, y));
            let on_hug = cursor_visited.contains(&vec2(x, y));

            const BLACK: (u8, u8, u8) = (0x00, 0x00, 0x00);
            const DARK_PURPLE: (u8, u8, u8) = (0x7E, 0x25, 0x53);
            const DARK_GREEN: (u8, u8, u8) = (0x00, 0x87, 0x51);
            const LIGHT_GRAY: (u8, u8, u8) = (0xC2, 0xC3, 0xC7);
            const RED: (u8, u8, u8) = (0xFF, 0x00, 0x4D);

            let (r, g, b) = match (is_start_or_end, is_wall, on_path, on_hug) {
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

fn reduce_segments(walls: &HashSet<Vec2>, mut segments: Vec<Vec2>, end: Vec2) -> (Vec<Vec2>, HashSet<Vec2>) {
    println!("{}: {segments:?}", segments.len());
    'outer: loop {
        let mut cursor = vec2(0, -1);
        for (i, w) in segments.windows(3).enumerate() {
            cursor += w[0];

            // See if we can swap w[1] and w[2] -> if so, we can merge w[0] and w[2]
            debug_assert!(w[0].same_dir(w[2]));
            if apply(cursor, w[2]).all(|step| !walls.contains(&step))
                && apply(cursor + w[2], w[1]).all(|step| !walls.contains(&step))
            {
                let to_merge = w[2];
                segments[i] += to_merge;
                segments.remove(i + 2);

                // Since the segments are in order [Horizontal, Vertical, Horizontal, Vertical]
                // (or w.l.o.g. the opposite), if we merged two of the same direction, we can merge
                // the following two.
                debug_assert!(segments[i + 1].same_dir(segments[i + 2]));
                let to_merge = segments[i + 2];
                segments[i + 1] += to_merge;
                segments.remove(i + 2);

                continue 'outer;
            }

            // See if we can shorten fst and snd and still not hit a wall
            let &[mut fst, snd, mut trd] = w else { unreachable!() };
            while fst.mag() > 1 && trd.mag() > 1 {
                let fst_step = fst.normalized();
                if apply(cursor - w[0], fst - fst_step).all(|step| !walls.contains(&step))
                    && apply(cursor - w[0] + fst - fst_step, snd).all(|step| !walls.contains(&step))
                    && apply(cursor - w[0] + fst - fst_step + snd, trd + fst_step).all(|step| !walls.contains(&step))
                {
                    fst -= fst_step;
                    trd += fst_step;
                } else {
                    break;
                }
            }
            debug_assert_eq!(cursor - w[0] + fst + snd + trd, cursor + w[1] + w[2]);

            if fst != w[0] || trd != w[2] {
                segments[i] = fst;
                segments[i + 2] = trd;
                continue 'outer;
            }
        }
        break;
    }
    println!("{}: {segments:?}", segments.len());

    let mut cursor_visited = HashSet::default();
    let mut cursor = vec2(0, -1);
    for &dir in segments.iter() {
        for p in apply(cursor, dir) {
            debug_assert!(
                end == p || !walls.contains(&p),
                "ran into wall at {p:?} when applying {dir:?} from {cursor:?}"
            );
            cursor_visited.insert(p);
        }
        cursor += dir;
    }
    println!("TOTAL REDUCED STEPS = {}", cursor_visited.len());
    (segments, cursor_visited)
}

fn follow_wall(end: Vec2, wall_cells: &HashSet<Vec2>, wall_lines: &[Segment]) -> (Vec<Vec2>, HashSet<Vec2>) {
    let mut cursor = vec2(0, -1);
    let mut heading = vec2(-1, 0);
    let mut visited = HashSet::default();

    let mut steps: Vec<Vec2> = vec![];
    fn add_step(steps: &mut Vec<Vec2>, step: Vec2) {
        if let Some(last) = steps.last_mut()
            && last.same_dir(step)
        {
            *last += step;
        } else {
            steps.push(step);
        }
    }

    let mut it = wall_lines.iter().peekable();

    'mainloop: while let Some(hugged_wall) = it.next() {
        println!(
            "@ {cursor:?} heading {heading:?}, looking for wall hug towards {:?}",
            heading.rotated_left()
        );

        let lefthand_dir = heading.rotated_left();
        let target = hugged_wall.end + heading - lefthand_dir;
        println!("\tFound target {target:?}");

        let hit_candidate = target - heading;

        if let Some(bee_start) = segment(cursor, hit_candidate - heading)
            .into_iter()
            .find(|p| p.1 == end.1)
            && segment(bee_start, end)
                .into_iter()
                .all(|p| p == end || !wall_cells.contains(&p))
        {
            add_step(&mut steps, bee_start - cursor);
            add_step(&mut steps, end - bee_start);
            visited.extend(segment(bee_start, end));
            visited.extend(segment(cursor, bee_start));
            break 'mainloop;
        }

        if it.peek().expect("should be more walls to hug").contains(hit_candidate) {
            visited.extend(segment(cursor, hit_candidate - heading));
            add_step(&mut steps, (hit_candidate - heading) - cursor);
            cursor = hit_candidate - heading;
            let left_dir = heading.rotated_left();
            debug_assert!(wall_cells.contains(&(cursor + left_dir)));
            heading.rotate_right();
            eprintln!("\tRan into wall at {hit_candidate:?}, new heading {heading:?}");
            continue 'mainloop;
        }

        visited.extend(segment(cursor, target));
        add_step(&mut steps, target - cursor);
        cursor = target;
        heading.rotate_left();
        add_step(&mut steps, heading);
        cursor += heading;
        visited.insert(cursor);
    }

    (steps, visited)
}
