use std::fmt::Display;

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

    for _ in 0..2 {
        let len = ducks.len();

        loop {
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

            round += 1;
        }
        ducks.reverse();
    }


    round
}

#[inline]
pub fn solve_part3() -> impl Display {
    let ducks = include_str!("part3.txt")
        .lines()
        .map(|line| line.parse::<u64>().unwrap())
        .collect::<Vec<u64>>();
    let mean = ducks.iter().sum::<u64>() / ducks.len() as u64;
    ducks.iter().map(|&duck| mean.saturating_sub(duck)).sum::<u64>()
}



