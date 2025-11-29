use std::{collections::HashSet, fmt::Display, iter::once};

use grid::Grid;
use pathfinding::prelude::bfs;

const TRAMPOLINE: u8 = b'T';
const START: u8 = b'S';
const END: u8 = b'E';

#[inline]
pub fn solve() -> (impl Display, impl Display, impl Display) {
    (solve_part1(), solve_part2(), solve_part3())
}

#[inline]
pub fn solve_part1() -> impl Display {
    let input = include_str!("part1.txt");
    let map = parse(input);
    let adjacency = build_adjacency(&map);
    adjacency.indexed_into_iter().map(|(_, vs)| vs.len()).sum::<usize>() / 2
}

#[inline]
pub fn solve_part2() -> impl Display {
    let input = include_str!("part2.txt");
    let map = parse(input);
    let adjacency = build_adjacency(&map);
    let start = map.indexed_iter().find(|(_, c)| **c == START).unwrap().0;
    let end = map.indexed_iter().find(|(_, c)| **c == END).unwrap().0;

    bfs(&start, |node| adjacency[*node].iter().copied(), |node| *node == end)
        .unwrap()
        .len()
        - 1
}

fn parse(input: &'static str) -> Grid<u8> {
    let cols = input.lines().next().unwrap().len();
    let map = Grid::from_vec(
        input.bytes().filter(|b| !b.is_ascii_whitespace()).collect::<Vec<u8>>(),
        cols,
    );
    map
}

fn build_adjacency(map: &Grid<u8>) -> Grid<HashSet<(usize, usize)>> {
    let mut adjacency: Grid<HashSet<(usize, usize)>> = Grid::new(map.rows(), map.cols());
    for ((y, x), &cell) in map.indexed_iter() {
        if !matches!(cell, TRAMPOLINE | START | END) {
            continue;
        }

        let adj_x = x - y;
        let neighbors = [
            Some((x.wrapping_sub(1), y)),
            Some((x.wrapping_add(1), y)),
            (y != 0 && adj_x % 2 == 0).then(|| (adj_x + 1 + (y - 1), y - 1)),
            (y != map.rows() - 1 && adj_x % 2 != 0).then(|| (adj_x - 1 + (y + 1), y + 1)),
        ];
        for (nx, ny) in neighbors.into_iter().flatten() {
            if matches!(map.get(ny, nx), Some(&TRAMPOLINE) | Some(&START) | Some(&END)) {
                adjacency[(y, x)].insert((ny, nx));
            }
        }
    }
    adjacency
}

fn build_adjacency_3d(from_map: &Grid<u8>, to_map: &Grid<u8>) -> Grid<HashSet<(usize, usize)>> {
    let mut adjacency: Grid<HashSet<(usize, usize)>> = Grid::new(from_map.rows(), from_map.cols());
    for ((y, x), &cell) in from_map.indexed_iter() {
        if !matches!(cell, TRAMPOLINE | START | END) {
            continue;
        }

        let adj_x = x - y;
        let neighbors = [
            Some((x.wrapping_sub(1), y)),
            Some((x.wrapping_add(1), y)),
            (y != 0 && adj_x % 2 == 0).then(|| (adj_x + 1 + (y - 1), y - 1)),
            (y != to_map.rows() - 1 && adj_x % 2 != 0).then(|| (adj_x - 1 + (y + 1), y + 1)),
        ];
        for (nx, ny) in neighbors.into_iter().flatten() {
            if matches!(to_map.get(ny, nx), Some(&TRAMPOLINE) | Some(&START) | Some(&END)) {
                adjacency[(y, x)].insert((ny, nx));
            }
        }
    }
    adjacency
}

#[inline]
pub fn solve_part3() -> impl Display {
    let input = include_str!("part3.txt");
    let map = parse(input);
    let map120 = rot120(&map);
    let map240 = rot120(&map120);

    let adj0 = build_adjacency_3d(&map, &map120);
    let adj120 = build_adjacency_3d(&map120, &map240);
    let adj240 = build_adjacency_3d(&map240, &map);

    let (sy, sx) = map.indexed_iter().find(|(_, c)| **c == START).unwrap().0;

    let path = bfs(
        &(0, sy, sx),
        |&(z, y, x)| {
            match z {
                0 => adj0[(y, x)].iter().copied(),
                1 => adj120[(y, x)].iter().copied(),
                2 => adj240[(y, x)].iter().copied(),
                _ => unreachable!(),
            }
            .map(move |(y, x)| ((z + 1) % 3, y, x))
            .chain({
                let cell = match z {
                    0 => map[(y, x)],
                    1 => map120[(y, x)],
                    2 => map240[(y, x)],
                    _ => unreachable!(),
                };

                (matches!(cell, TRAMPOLINE | START | END)).then_some(((z + 1) % 3, y, x))
            })
        },
        |&(z, y, x)| match z {
            0 => map[(y, x)] == END,
            1 => map120[(y, x)] == END,
            2 => map240[(y, x)] == END,
            _ => unreachable!(),
        },
    )
    .unwrap();

    // for &(pz, py, px) in &path {
    //     for y in 0..map.rows() {
    //         for x in 0..map.cols() {
    //             if (py, px) == (y, x) {
    //                 print!("\x1b[32;7m");
    //             }
    //
    //             let cell = match pz {
    //                 0 => map[(y, x)],
    //                 1 => map120[(y, x)],
    //                 2 => map240[(y, x)],
    //                 _ => unreachable!(),
    //             };
    //
    //             print!("{}\x1b[0m", cell as char);
    //         }
    //         println!();
    //     }
    //     println!();
    // }

    path.len() - 1
}

fn rot120(map: &Grid<u8>) -> Grid<u8> {
    let mut new_map: Grid<u8> = Grid::new(map.rows(), map.cols());
    new_map.fill(b'.');

    for offset in 0..map.cols() {
        let src = (0..map.rows())
            .rev()
            .filter_map(|y| Some((y, map.cols().checked_sub(y + offset + 1)?)))
            .skip_while(|c| map[*c] == b'.');

        let dst = (0..map.rows()).map(|y| (y, y + offset));

        for (s, d) in src.zip(dst) {
            new_map[d] = map[s];
        }
    }
    new_map
}

