use std::fmt::Display;

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

    cases.lines().map(|case| eval(&plants, case)).sum::<i64>()
}

pub fn solve_part3() -> impl Display {
    let input = include_str!("part3.txt");
    let (plant_desc, cases) = input.rsplit_once("\n\n").unwrap();
    let plants = parse(plant_desc);

    let best = eval_best(&plants);

    cases
        .lines()
        .map(|case| eval(&plants, case))
        .filter_map(|s| (s != 0).then_some(s.abs_diff(best)))
        .sum::<u64>()
}

fn eval(plants: &[Plant], case: &str) -> i64 {
    let mut calculated = vec![0; plants.len()];

    for ((i, p), k) in plants.iter().enumerate().zip(case.split(' ')) {
        debug_assert!(p.branches.is_empty());
        calculated[i] = k.parse().unwrap();
    }

    for (i, p) in plants.iter().enumerate().skip((case.len() + 1) / 2) {
        if let Some(brightness) = p
            .branches
            .iter()
            .try_fold(0, |acc, b| Some(b.thickness * calculated[b.to] + acc))
        {
            calculated[i] = if brightness >= p.thickness { brightness } else { 0 };
        }
    }

    *calculated.last().unwrap()
}

fn eval_best(plants: &[Plant]) -> i64 {
    let inputs = plants.iter().position(|p| !p.branches.is_empty()).unwrap();
    let mut calculated = vec![0; plants.len() - inputs];

    for (i, p) in plants.iter().skip(inputs).enumerate() {
        if let Some(brightness) = p.branches.iter().try_fold(0, |acc, b| {
            if b.to < inputs {
                Some(if b.thickness > 0 { b.thickness + acc } else { acc })
            } else {
                Some(b.thickness * calculated[b.to - inputs] + acc)
            }
        }) {
            calculated[i] = if brightness >= p.thickness { brightness } else { 0 };
        }
    }
    *calculated.last().unwrap()
}
