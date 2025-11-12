use std::{collections::HashSet, fmt::Display};

#[inline]
pub fn solve() -> (impl Display, impl Display, impl Display) {
    (solve_part1(), solve_part2(), solve_part3())
}

fn str_to_idx(s: &str) -> usize {
    let b = s.as_bytes()[0];
    u8_to_idx(b)
}

fn u8_to_idx(b: u8) -> usize {
    if b.is_ascii_lowercase() {
        (b - b'a') as usize
    } else {
        (b - b'A') as usize
    }
}

#[inline]
pub fn solve_part1() -> impl Display {
    let (names, rules) = include_str!("part1.txt").split_once("\n\n").unwrap();
    let names = names.split(',').collect::<Vec<_>>();

    let mut lowercase_rules = [0u32; 26];
    let mut uppercase_rules = [0u32; 26];
    for line in rules.lines() {
        let (before, afters) = line.split_once(" > ").unwrap();

        for after in afters.split(",") {
            let idx = str_to_idx(before);
            if before.chars().next().unwrap().is_ascii_lowercase() {
                lowercase_rules[idx] |= 1 << str_to_idx(after);
            } else {
                uppercase_rules[idx] |= 1 << str_to_idx(after);
            }
        }
    }

    'names: for name in names {
        for (c1, c2) in name.chars().zip(name.chars().skip(1)) {
            let idx = str_to_idx(&c1.to_string());
            let rule = if c1.is_ascii_lowercase() {
                lowercase_rules[idx]
            } else {
                uppercase_rules[idx]
            };
            if rule != 0 && (rule & (1 << str_to_idx(&c2.to_string()))) == 0 {
                continue 'names;
            }
        }
        return name;
    }
    "NO SOLUTION"
}

#[inline]
pub fn solve_part2() -> impl Display {
    let (names, rules) = include_str!("part2.txt").split_once("\n\n").unwrap();
    let names = names.split(',').collect::<Vec<_>>();

    let mut lowercase_rules = [0u32; 26];
    let mut uppercase_rules = [0u32; 26];
    for line in rules.lines() {
        let (before, afters) = line.split_once(" > ").unwrap();

        for after in afters.split(",") {
            let idx = str_to_idx(before);
            if after.chars().next().unwrap().is_ascii_lowercase() {
                lowercase_rules[idx] |= 1 << str_to_idx(after);
            } else {
                uppercase_rules[idx] |= 1 << str_to_idx(after);
            }
        }
    }

    let mut count = 0usize;
    'names: for (i, name) in names.iter().enumerate() {
        for (c1, c2) in name.chars().zip(name.chars().skip(1)) {
            let idx = str_to_idx(&c1.to_string());
            let rule = if c1.is_ascii_lowercase() {
                lowercase_rules[idx]
            } else {
                uppercase_rules[idx]
            };
            if rule != 0 && (rule & (1 << str_to_idx(&c2.to_string()))) == 0 {
                continue 'names;
            }
        }
        count += 1 + i;
    }
    count
}

#[inline]
pub fn solve_part3() -> impl Display {
    let (names, rules) = include_str!("part3.txt").split_once("\n\n").unwrap();
    let names = names.split(',').collect::<Vec<_>>();

    let mut lowercase_rules = [0u32; 26];
    let mut uppercase_rules = [0u32; 26];
    for line in rules.lines() {
        let (before, afters) = line.split_once(" > ").unwrap();

        for after in afters.split(",") {
            let idx = str_to_idx(before);
            if before.chars().next().unwrap().is_ascii_lowercase() {
                lowercase_rules[idx] |= 1 << str_to_idx(after);
            } else {
                uppercase_rules[idx] |= 1 << str_to_idx(after);
            }
        }
    }

    let mut result = HashSet::new();
    'names: for name in names.iter() {
        for (c1, c2) in name.chars().zip(name.chars().skip(1)) {
            let idx = str_to_idx(&c1.to_string());
            let rule = if c1.is_ascii_lowercase() {
                lowercase_rules[idx]
            } else {
                uppercase_rules[idx]
            };
            if rule != 0 && (rule & (1 << str_to_idx(&c2.to_string()))) == 0 {
                continue 'names;
            }
        }

        let mut states = Vec::new();
        states.push(name.to_string());

        while let Some(state) = states.pop() {
            if state.len() >= 7 && state.len() <= 11 {
                if result.insert(state.clone()) {
                }
            }

            for (c1, c2) in state.chars().zip(state.chars().skip(1)) {
                let idx = str_to_idx(&c1.to_string());
                let rule = if c1.is_ascii_lowercase() {
                    lowercase_rules[idx]
                } else {
                    uppercase_rules[idx]
                };
                if rule != 0 && (rule & (1 << str_to_idx(&c2.to_string()))) == 0 {
                    panic!();
                }
            }

            if state.len() < 11 {
                let &last = state.as_bytes().last().unwrap();
                let i = u8_to_idx(last);
                let next_rules = if last.is_ascii_lowercase() {
                    lowercase_rules[i]
                } else {
                    panic!()
                };
                for n in b'a'..=b'z' {
                    if next_rules & (1 << u8_to_idx(n)) != 0 {
                        states.push(state.clone());
                        states.last_mut().unwrap().push(n as char);
                    }
                }
            }
        }
    }
    result.len()
}
