use std::fmt::Display;

use rayon::prelude::*;

#[inline]
pub fn solve() -> (impl Display, impl Display, impl Display) {
    (solve_part1(), solve_part2(), solve_part3())
}

fn u8_to_idx(b: u8) -> usize {
    if b.is_ascii_lowercase() {
        (b - b'a') as usize
    } else {
        (b - b'A' + 26) as usize
    }
}

struct Rules([u64; 2 * 26]);

impl Rules {
    fn new(list: &str) -> Self {
        let mut this = Self([0; 2 * 26]);
        for line in list.lines() {
            let (before, afters) = line.split_once(" > ").unwrap();
            let before_idx = u8_to_idx(before.as_bytes()[0]);
            for after in afters.split(',') {
                let after_idx = u8_to_idx(after.as_bytes()[0]);
                this.0[before_idx] |= 1 << after_idx;
            }
        }
        this
    }

    fn allows(&self, name: &[u8]) -> bool {
        name.iter()
            .zip(name.iter().skip(1))
            .all(|(&c1, &c2)| self.can_follow(c1, c2))
    }

    fn can_follow(&self, before: u8, after: u8) -> bool {
        let idx = u8_to_idx(before);
        let rule = self.0[idx];
        (rule & (1 << u8_to_idx(after))) != 0
    }
}

#[inline]
pub fn solve_part1() -> impl Display {
    let (names, rules) = include_str!("part1.txt").split_once("\n\n").unwrap();
    let mut names = names.split(',');
    let rules = Rules::new(rules);

    names.find(|name| rules.allows(name.as_bytes())).unwrap()
}

#[inline]
pub fn solve_part2() -> impl Display {
    let (names, rules) = include_str!("part2.txt").split_once("\n\n").unwrap();
    let names = names.split(',');
    let rules = Rules::new(rules);

    names
        .enumerate()
        .filter(|(_, name)| rules.allows(name.as_bytes()))
        .map(|(i, _)| i + 1)
        .sum::<usize>()
}

#[inline]
pub fn solve_part3() -> impl Display {
    let (names, rules) = include_str!("part3.txt").split_once("\n\n").unwrap();
    let mut names = names.split(',').collect::<Vec<_>>();
    names.sort_unstable_by_key(|n| n.len());

    let rules = Rules::new(rules);

    // Filter prefixes to only those that are not themselves prefixes of other existing ones;
    // we sort by length first to ensure we only keep the shortest unique prefixes, otherwise we
    // could skip over some valid names.
    let mut unique_prefixes = Vec::new();
    for name in names {
        if !rules.allows(name.as_bytes()) {
            continue;
        }

        if !unique_prefixes.iter().any(|prefix| name.starts_with(prefix)) {
            unique_prefixes.push(name);
        }
    }

    unique_prefixes
        .into_par_iter()
        .map(|prefix| {
            let first = prefix.as_bytes().last().copied().unwrap();

            let mut total = 0;
            let mut states = [0; 26];
            states[u8_to_idx(first)] += 1;
            for len in prefix.len()..=11 {
                let mut next_states = [0; 26];
                for (prev_idx, count) in states.into_iter().enumerate() {
                    if count == 0 {
                        continue;
                    }

                    if len >= 7 {
                        total += count;
                    }
                    for next in b'a'..=b'z' {
                        if rules.can_follow(prev_idx as u8 + b'a', next) {
                            next_states[u8_to_idx(next)] += states[prev_idx];
                        }
                    }
                }
                states = next_states;
            }
            total
        })
        .sum::<u32>()
}
