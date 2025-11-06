use std::fmt::Display;

use atoi::FromRadix10;

#[inline]
pub fn solve() -> (impl Display, impl Display, impl Display) {
    (solve_part1(), solve_part2(), solve_part3())
}

#[inline]
pub fn solve_part1() -> impl Display {
    let (names, instrs) = include_str!("part1.txt").split_once("\n\n").unwrap();
    let (names, instrs) = parse(names, instrs);
    let mut i = 0usize;

    for (d, n) in instrs {
        match d {
            b'R' => {
                i += (n as usize).min(names.len() - 1 - i);
            }
            b'L' => {
                i = i.saturating_sub(n as usize);
            }
            _ => unreachable!(),
        }
    }

    names[i]
}

#[inline]
pub fn solve_part2() -> impl Display {
    let (names, instrs) = include_str!("part2.txt").split_once("\n\n").unwrap();
    let (names, instrs) = parse(names, instrs);
    let mut i = 0isize;

    for (d, n) in instrs {
        match d {
            b'R' => i = (i + n).rem_euclid(names.len() as isize),
            b'L' => i = (i - n).rem_euclid(names.len() as isize),
            _ => unreachable!(),
        }
    }

    names[usize::try_from(i).unwrap()]
}

#[inline]
pub fn solve_part3() -> impl Display {
    let (names, instrs) = include_str!("part3.txt").split_once("\n\n").unwrap();
    let (mut names, instrs) = parse(names, instrs);
    let l = names.len() as isize;

    for (d, n) in instrs {
        let j = match d {
            b'R' => n.rem_euclid(l),
            b'L' => (-n).rem_euclid(l),
            _ => unreachable!(),
        };
        names.swap(0, j as usize);
    }

    names[0]
}

fn parse(names: &'static str, instrs: &'static str) -> (Vec<&'static str>, impl Iterator<Item = (u8, isize)>) {
    let names: Vec<&str> = names.split(',').collect();
    let instrs = instrs.split(',').map(|d| {
        let cs = d.as_bytes();
        let d = cs[0];
        let n = isize::from_radix_10(&cs[1..]).0;
        (d, n)
    });
    (names, instrs)
}
