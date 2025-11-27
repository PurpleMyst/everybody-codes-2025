use std::fmt::Display;
use std::fs::File;
use std::io::Write;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct Plant {
    thickness: i64,
    branches: Vec<Branch>,
}
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
struct Branch {
    to: usize,
    thickness: i64,
}

#[inline]
pub fn solve() -> (impl Display, impl Display, impl Display) {
    (solve_part1(), solve_part2(), solve_part3())
}

#[inline]
pub fn solve_part1() -> impl Display {
    let input = include_str!("part1.txt");
    let plants = parse(input);

    let mut calculated = vec![None; plants.len()];
    for (i, p) in plants.iter().enumerate() {
        if p.branches.is_empty() {
            calculated[i] = Some(1);
        }
    }
    'outer: while calculated.iter().any(Option::is_none) {
        for (i, p) in plants.iter().enumerate() {
            if calculated[i].is_some() {
                continue;
            }

            if let Some(brightness) = p
                .branches
                .iter()
                .try_fold(0, |acc, b| Some(b.thickness * calculated[b.to]? + acc))
            {
                calculated[i] = Some(if brightness < p.thickness { 0 } else { brightness });
                continue 'outer;
            }
        }
        unreachable!();
    }

    calculated.last().unwrap().unwrap()
}

fn parse(input: &'static str) -> Vec<Plant> {
    let plants = input
        .split("\n\n")
        .map(|plant_s| {
            let mut it = plant_s.lines().peekable();
            let thickness = it.next().unwrap().split(' ').last().unwrap();
            let thickness = thickness[..thickness.len() - 1].parse().unwrap();
            let branches = if it.peek().unwrap().contains("free") {
                vec![]
            } else {
                it.map(|line| {
                    let mut it2 = line.split(' ');
                    let to = it2.nth(4).unwrap().parse::<usize>().unwrap() - 1;
                    let thickness = it2.last().unwrap().parse().unwrap();
                    Branch { to, thickness }
                })
                .collect()
            };
            Plant { thickness, branches }
        })
        .collect::<Vec<_>>();
    plants
}

#[inline]
pub fn solve_part2() -> impl Display {
    let input = include_str!("part2.txt");
    let (plant_desc, cases) = input.rsplit_once("\n\n").unwrap();
    let plants = parse(plant_desc);

    cases.lines().map(|case| {
        eval(&plants, case)
    }).sum::<i64>()
}

pub fn solve_part3() -> impl Display {
    let input = include_str!("part3.txt");
    let (plant_desc, cases) = input.rsplit_once("\n\n").unwrap();
    let plants = parse(plant_desc);

    let best = eval(&plants, "0 0 1 0 1 0 1 0 0 0 0 1 1 1 1 1 0 0 0 1 0 0 0 0 0 1 0 0 0 0 1 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 1 0 0 0 0 0 0 1 0 0 0 0 0 1 0 0 1 0 0 0 0 0 1 0 0 0 1 1 1 1 1 0 0");

    cases.lines().map(|case| {
        eval(&plants, case)
    })
    .filter_map(|s| (s != 0).then_some(s.abs_diff(best)))
    .sum::<u64>()
}

fn eval(plants: &[Plant], case: &str) -> i64 {
    let mut calculated = vec![None; plants.len()];
    for ((i, p), k) in plants.iter().enumerate().zip(case.split(' ')) {
        debug_assert!(p.branches.is_empty());
        calculated[i] = Some(k.parse().unwrap());
    }
    'outer: while calculated.iter().any(Option::is_none) {
        for (i, p) in plants.iter().enumerate() {
            if calculated[i].is_some() {
                continue;
            }

            if let Some(brightness) = p
                .branches
                .iter()
                .try_fold(0, |acc, b| Some(b.thickness * calculated[b.to]? + acc))
            {
                calculated[i] = Some(if brightness >= p.thickness { brightness } else { 0 });
                continue 'outer;
            }
        }
        unreachable!();
    }
    calculated.last().unwrap().unwrap()
}

#[inline]
pub fn toposort() {
    let input = include_str!("part3.txt");
    let (plant_desc, _cases) = input.rsplit_once("\n\n").unwrap();
    let plants = parse(plant_desc);

    let mut calculated = vec![false; plants.len()];
    for (i, p) in plants.iter().enumerate() {
        if p.branches.is_empty() {
            calculated[i] = true;
        }
    }

    let mut program = File::create("program.txt").unwrap();
    'outer: while calculated.iter().any(|&done| !done) {
        for (i, p) in plants.iter().enumerate() {
            if calculated[i] {
                continue;
            }

            if p
                .branches
                .iter()
                .all(|b| calculated[b.to])
            {
                write!(program, "p[{}] = threshold({}, ", i + 1, p.thickness).unwrap();
                for (i, branch) in p.branches.iter().enumerate() {
                    if i != 0 {
                        write!(program, " + ").unwrap();
                    }
                    write!(program, "({}) * p[{}]", branch.thickness, branch.to + 1).unwrap();
                }

                write!(program, ") # depends on [").unwrap();
                let mut j = 0;
                for branch in p.branches.iter() {
                    if !plants[branch.to].branches.is_empty() {
                        continue;
                    }
                    if j != 0 {
                        write!(program, ", ").unwrap();
                    }
                    write!(program, "p[{}]", branch.to + 1).unwrap();
                    j += 1;
                }
                writeln!(program, "] (len = {j})").unwrap();

                calculated[i] = true;
                continue 'outer;
            }
        }
        unreachable!();
    }
}

