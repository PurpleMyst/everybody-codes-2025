use std::fmt::Display;

#[inline]
pub fn solve() -> (impl Display, impl Display, impl Display) {
    (solve_part1(), solve_part2(), solve_part3())
}

#[inline]
pub fn solve_part1() -> impl Display {
    let (names, instrs) = include_str!("part1.txt").split_once("\n\n").unwrap();
    let mut i = 0usize;

    let names: Vec<&str> = names.split(',').collect();
    let instrs = instrs.split(',').map(|d| {
        let mut cs = d.chars();
        let d = cs.next().unwrap();
        let n: usize = cs.collect::<String>().trim().parse().unwrap();
        (d, n)
    });

    for (d, n) in instrs {
        match d {
            'R' => {
                for _ in 0..n {
                    if i != names.len() - 1 {
                        i += 1;
                    }
                }
            }
            'L' => {
                for _ in 0..n {
                    if i != 0 {
                        i -= 1;
                    }
                }
            }
            _ => unreachable!(),
        }
    }

    names[i]
}

#[inline]
pub fn solve_part2() -> impl Display {
    let (names, instrs) = include_str!("part2.txt").split_once("\n\n").unwrap();
    let mut i = 0isize;

    let names: Vec<&str> = names.split(',').collect();
    let instrs = instrs.split(',').map(|d| {
        let mut cs = d.chars();
        let d = cs.next().unwrap();
        let n: isize = cs.collect::<String>().trim().parse().unwrap();
        (d, n)
    });

    for (d, n) in instrs {
        match d {
            'R' => i = (i + n).rem_euclid(names.len() as isize),
            'L' => i = (i - n).rem_euclid(names.len() as isize),
            _ => unreachable!(),
        }
    }

    names[usize::try_from(i).unwrap()]
}

#[inline]
pub fn solve_part3() -> impl Display {
    let (names, instrs) = include_str!("part3.txt").split_once("\n\n").unwrap();

    let mut names: Vec<&str> = names.split(',').collect();
    let instrs = instrs.split(',').map(|d| {
        let mut cs = d.chars();
        let d = cs.next().unwrap();
        let n: isize = cs.collect::<String>().trim().parse().unwrap();
        (d, n)
    });

    for (d, n) in instrs {
        let j = match d {
            'R' => n.rem_euclid(names.len() as isize),
            'L' => (-n).rem_euclid(names.len() as isize),
            _ => unreachable!(),
        };
        names.swap(0, usize::try_from(j).unwrap());
    }

    names[0]
}
