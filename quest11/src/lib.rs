use std::{cmp::Ordering, fmt::Display};

const BOLD: &str = "\x1b[1m";
const DIM: &str = "\x1b[2m";

const RED: &str = "\x1b[31m";
const GREEN: &str = "\x1b[32m";
const YELLOW: &str = "\x1b[33m";

const RESET: &str = "\x1b[0m";

#[inline]
pub fn solve() -> (impl Display, impl Display, impl Display) {
    (solve_part1(), solve_part2(), solve_part3())
}

#[inline]
pub fn solve_part1() -> impl Display {
    let mut ducks = include_str!("part1.txt")
        .lines()
        .map(|line| line.parse::<u64>().unwrap())
        .collect::<Vec<u64>>();
    let mut round = 0;
    'outer: loop {
        // First phase
        loop {
            let mut moved = false;
            for i in 0..ducks.len() - 1 {
                if ducks[i + 1] < ducks[i] {
                    ducks[i + 1] += 1;
                    ducks[i] -= 1;
                    moved = true;
                }
            }
            round += 1;
            if round == 11 {
                break 'outer;
            }
            if !moved {
                break;
            }
        }

        // Second phase
        loop {
            let mut moved = false;
            for i in 0..ducks.len() - 1 {
                if ducks[i + 1] > ducks[i] {
                    ducks[i] += 1;
                    ducks[i + 1] -= 1;
                    moved = true;
                }
            }
            round += 1;
            if round == 11 {
                break 'outer;
            }
            if !moved {
                break;
            }
        }
    }

    ducks
        .into_iter()
        .enumerate()
        .map(|(i, n)| (1 + i as u64) * n)
        .sum::<u64>()
}

#[inline]
pub fn solve_part2() -> impl Display {
    let mut ducks = include_str!("part2.txt")
        .lines()
        .map(|line| line.parse::<u64>().unwrap())
        .collect::<Vec<u64>>();
    let mut round = 0;
        // First phase

    print!("\t");
    for c in &ducks {
        print!("{DIM}{c}{RESET} ");
    }
    println!();
    for _ in 0..2 {
        print!("\t");
        for c in &ducks {
            print!("{}-", "-".repeat((1 + c.ilog10()) as _));
        }
        println!("\x1b[D ");
        let len = ducks.len();

        loop {
            let prev = ducks.clone();
            let mut moved = false;

            let mut i = 0;
            'outer: while i < ducks.len() -1 {
                if ducks[i] <= ducks[i + 1] {
                    i += 1;
                    continue;
                }

                for j in i + 1..ducks.len() - 1 {
                    if ducks[j] + 1 <= ducks[j + 1] {
                        ducks[j] += 1;
                        ducks[i] -= 1;
                        moved = true;
                        i = j + 1;
                        continue 'outer;
                    }
                }

                ducks[len - 1] += 1;
                ducks[i] -= 1;
                moved = true;
                break;
            }

            if !moved {
                break;
            }

            print!("\t");
            for (p, c) in prev.into_iter().zip(ducks.iter()) {
                match c.cmp(&p) {
                    Ordering::Less => print!("{BOLD}{RED}{c}{RESET} "),
                    Ordering::Equal => print!("{DIM}{YELLOW}{c}{RESET} "),
                    Ordering::Greater => print!("{BOLD}{GREEN}{c}{RESET} "),
                }
            }
            println!();

            round += 1;
        }
        ducks.reverse();
    }


    round
}

#[inline]
pub fn solve_part3() -> impl Display {
    "TODO"
}



