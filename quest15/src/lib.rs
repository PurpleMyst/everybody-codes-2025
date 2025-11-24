use std::fmt::Display;

mod math;
use math::*;

#[inline]
pub fn solve() -> (impl Display, impl Display, impl Display) {
    (solve_part1(), solve_part2(), solve_part3())
}

#[inline]
pub fn solve_part1() -> impl Display {
    let input = include_str!("part1.txt");
    do_solve(input, false)
}

#[inline]
pub fn solve_part2() -> impl Display {
    let input = include_str!("part2.txt");
    do_solve(input, false)
}

#[inline]
pub fn solve_part3() -> impl Display {
    let input = include_str!("part3.txt");
    do_solve(input, true)
}

fn do_solve(input: &'static str, hug_right: bool) -> i64 {
    let mut dir = vec2(0, -1);
    let mut end = vec2(0, 0);
    let mut wall_lines = Vec::new();

    // let mut wall_cells = HashSet::default();
    input.trim().split(",").for_each(|bit| {
        let (turn, n) = bit.split_at(1);
        let n = n.parse::<i64>().unwrap();
        match turn {
            "R" => dir.rotate_right(),
            "L" => dir.rotate_left(),
            _ => unreachable!(),
        };
        wall_lines.push(segment(end, end + dir * n));
        end += dir * n;
    });
    eprintln!("✅ WALL LINES = {wall_lines:?}");
    eprintln!("✅ END POS = {end:?}");

    let wf_path = if hug_right {
        follow_right_wall(end, &wall_lines)
    } else {
        follow_left_wall(end, &wall_lines)
    };
    println!("INITIAL WALL-FOLLOW STEPS = {}", wf_path.total());

    let wf_path = reduce_steps(&wall_lines, wf_path);
    debug_assert_eq!(wf_path.steps.iter().fold(wf_path.start, |acc, &v| acc + v), end);
    println!("FINAL WALL-FOLLOW STEPS = {}", wf_path.total());

    wf_path.total()
}

fn reduce_steps(wall_lines: &[Segment], WallFollowPath { start, mut steps }: WallFollowPath) -> WallFollowPath {
    println!("{}: {steps:?}", steps.len());
    'outer: loop {
        let mut cursor = start;

        if let Some(i) = steps.iter().position(|s| s.is_zero()) {
            eprintln!("🧼 Removing zero step at index {i}");
            steps.remove(i);
            let to_merge = steps.remove(i);
            debug_assert!(steps[i - 1].same_dir(to_merge));
            steps[i - 1] += to_merge;
            continue 'outer;
        }

        debug_assert_eq!(steps.windows(3).find(|w| !w[0].same_dir(w[2])), None);

        for (i, w) in steps.windows(3).enumerate() {
            cursor += w[0];

            // Check if merging fst and trd is worthwile, and if so check if it is possible without
            // banging into a wall.
            debug_assert!(w[0].same_dir(w[2]));
            if ((w[0] + w[2]).mag() < w[0].mag() + w[2].mag()
                || (i + 3 < steps.len()
                    && (steps[i + 1] + steps[i + 3]).mag() < steps[i + 1].mag() + steps[i + 3].mag()))
                && segment(cursor, cursor + w[2]).intersects_none(wall_lines)
                && segment(cursor + w[2], cursor + w[2] + w[1]).intersects_none(wall_lines)
            {
                eprintln!("🧲 Merging steps at index {i}: {w:?}");

                let to_merge = w[2];
                steps[i] += to_merge;
                steps.remove(i + 2);

                // Since the segments are in order [Horizontal, Vertical, Horizontal, Vertical]
                // (or w.l.o.g. the opposite), if we merged two of the same direction, we can merge
                // the following two.
                debug_assert!(steps[i + 1].same_dir(steps[i + 2]));
                let to_merge = steps[i + 2];
                steps[i + 1] += to_merge;
                steps.remove(i + 2);

                continue 'outer;
            }

            // Check if we can shorten fst and trd by pushing some of fst into trd.
            let &[mut fst, snd, mut trd] = w else { unreachable!() };
            if (trd + fst.normalized()).mag() < trd.mag() {
                while fst.mag() > 1 && trd.mag() > 1 {
                    let fst_step = fst.normalized();
                    if segment_delta(cursor - w[0], fst - fst_step).intersects_none(wall_lines)
                        && segment_delta(cursor - w[0] + fst - fst_step, snd).intersects_none(wall_lines)
                        && segment_delta(cursor - w[0] + fst - fst_step + snd, trd + fst_step)
                            .intersects_none(wall_lines)
                    {
                        fst -= fst_step;
                        trd += fst_step;
                    } else {
                        break;
                    }
                }
                debug_assert_eq!(cursor - w[0] + fst + snd + trd, cursor + w[1] + w[2]);

                if fst != w[0] || trd != w[2] {
                    eprintln!("✂️ Shortening steps at index {i}: {w:?} -> {fst:?}, {snd:?}, {trd:?}");
                    steps[i] = fst;
                    steps[i + 2] = trd;
                    continue 'outer;
                }
            }
        }
        break;
    }
    println!("{}: {steps:?}", steps.len());

    WallFollowPath { start, steps }
}

struct WallFollowPath {
    start: Vec2,
    steps: Vec<Vec2>,
}

impl WallFollowPath {
    fn new(start: Vec2) -> Self {
        Self {
            start,
            steps: Vec::new(),
        }
    }

    fn add_step(&mut self, step: Vec2) {
        if step.is_zero() {
            return;
        }

        if let Some(last) = self.steps.last_mut()
            && last.same_dir(step)
        {
            *last += step;
        } else {
            self.steps.push(step);
        }
    }

    fn total(&self) -> i64 {
        self.start.mag() + self.steps.iter().map(|v| v.mag()).sum::<i64>()
    }
}

fn follow_left_wall(end: Vec2, wall_lines: &[Segment]) -> WallFollowPath {
    let initial_wall_is_left = wall_lines[0].end.0 < 0;

    let mut cursor = if initial_wall_is_left { vec2(0, -1) } else { vec2(0, 1) };

    let mut heading = if initial_wall_is_left { vec2(-1, 0) } else { vec2(1, 0) };

    let mut steps = WallFollowPath::new(cursor);

    let mut it = wall_lines.iter().peekable();

    'mainloop: while let Some(hugged_wall) = it.next() {
        println!(
            "@ {cursor:?} heading {heading:?}, looking for wall hug towards {:?}",
            heading.rotated_left()
        );

        let lefthand_dir = heading.rotated_left();
        let target = hugged_wall.end + heading - lefthand_dir;
        println!("\t🎯 Found target {target:?}");

        let hit_candidate = target - heading;
        let actually_hit = it.peek().is_some_and(|next_wall| next_wall.contains(hit_candidate));
        let last_bee_candidate = if actually_hit { hit_candidate - heading } else { target };

        // Can we go straight to the end? Check for a candidate within the "known safe" part of our
        // current step.
        if let Some(bee_start) = segment(cursor, last_bee_candidate).point_with_y(end.1)
            && !wall_lines
                .iter()
                .any(|wall| wall.intersection(&segment(bee_start, end)).is_some_and(|p| p != end))
        {
            eprintln!("\t🐝 Found bee line start at {bee_start:?}");
            steps.add_step(bee_start - cursor);
            steps.add_step(end - bee_start);
            break 'mainloop;
        }
        if let Some(bee_start) = segment(cursor, last_bee_candidate).point_with_x(end.0)
            && !wall_lines
                .iter()
                .any(|wall| wall.intersection(&segment(bee_start, end)).is_some_and(|p| p != end))
        {
            eprintln!("\t🐝 Found bee line start at {bee_start:?}");
            steps.add_step(bee_start - cursor);
            steps.add_step(end - bee_start);
            break 'mainloop;
        }

        if let Some(next_wall) = it.peek()
            && next_wall.contains(hit_candidate)
        {
            steps.add_step((hit_candidate - heading) - cursor);
            cursor = hit_candidate - heading;
            // ↓ this assumption should work only when hugging the left wall, it's probably the
            // opposite when hugging the right wall
            heading.rotate_right();
            eprintln!("\t💥 Ran into wall at {hit_candidate:?}, new heading {heading:?}");
            continue 'mainloop;
        }

        steps.add_step(target - cursor);
        cursor = target;
        heading.rotate_left();
        steps.add_step(heading);
        cursor += heading;
    }

    steps
}

fn follow_right_wall(end: Vec2, wall_lines: &[Segment]) -> WallFollowPath {
    let initial_wall_is_right = wall_lines[0].end.0 > 0;

    let mut cursor = if initial_wall_is_right { vec2(0, -1) } else { vec2(0, 1) };
    let mut heading = if initial_wall_is_right { vec2(1, 0) } else { vec2(-1, 0) };

    let mut steps = WallFollowPath::new(cursor);

    let mut it = wall_lines.iter().peekable();

    'mainloop: while let Some(hugged_wall) = it.next() {
        let righthand_dir = heading.rotated_right();
        println!("@ {cursor:?} heading {heading:?}, looking for wall hug towards {righthand_dir:?}");
        let target = hugged_wall.end + heading - righthand_dir;
        println!("\t🎯 Found target {target:?}");

        let hit_candidate = target - heading;
        let actually_hit = it.peek().is_some_and(|next_wall| next_wall.contains(hit_candidate));
        let last_bee_candidate = if actually_hit { hit_candidate - heading } else { target };

        // Can we go straight to the end? Check for a candidate within the "known safe" part of our
        // current step.
        if let Some(bee_start) = segment(cursor, last_bee_candidate).point_with_y(end.1)
            && !wall_lines
                .iter()
                .any(|wall| wall.intersection(&segment(bee_start, end)).is_some_and(|p| p != end))
        {
            eprintln!("\t🐝 Found bee line start at {bee_start:?}");
            steps.add_step(bee_start - cursor);
            steps.add_step(end - bee_start);
            break 'mainloop;
        }
        if let Some(bee_start) = segment(cursor, last_bee_candidate).point_with_x(end.0)
            && !wall_lines
                .iter()
                .any(|wall| wall.intersection(&segment(bee_start, end)).is_some_and(|p| p != end))
        {
            eprintln!("\t🐝 Found bee line start at {bee_start:?}");
            steps.add_step(bee_start - cursor);
            steps.add_step(end - bee_start);
            break 'mainloop;
        }

        if let Some(next_wall) = it.peek()
            && next_wall.contains(hit_candidate)
        {
            steps.add_step((hit_candidate - heading) - cursor);
            cursor = hit_candidate - heading;
            heading.rotate_left();
            eprintln!("\t💥 Ran into wall at {hit_candidate:?}, new heading {heading:?}");
            continue 'mainloop;
        }

        steps.add_step(target - cursor);
        cursor = target;
        heading.rotate_right();
        steps.add_step(heading);
        cursor += heading;
    }

    steps
}
