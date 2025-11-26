use std::fmt::Display;

use rayon::prelude::*;

#[inline]
pub fn solve() -> (impl Display, impl Display, impl Display) {
    (solve_part1(), solve_part2(), solve_part3())
}

#[inline]
pub fn solve_part1() -> impl Display {
    let input = include_str!("part1.txt");
    let width = input.lines().next().unwrap().len();
    let map = grid::Grid::from_vec(input.lines().flat_map(|line| line.bytes()).collect(), width);

    let volcano_pos = map.indexed_iter().find(|&(_, &cell)| cell == b'@').unwrap().0;

    map.indexed_iter()
        .filter_map(|((y, x), &cell)| {
            (cell.is_ascii_digit() && x.abs_diff(volcano_pos.0).pow(2) + y.abs_diff(volcano_pos.1).pow(2) <= 100)
                .then_some((cell - b'0') as u64)
        })
        .sum::<u64>()
}

#[inline]
pub fn solve_part2() -> impl Display {
    let input = include_str!("part2.txt");
    let width = input.lines().next().unwrap().len();
    let map = grid::Grid::from_vec(input.lines().flat_map(|line| line.bytes()).collect(), width);
    let volcano_pos = map.indexed_iter().find(|&(_, &cell)| cell == b'@').unwrap().0;
    let dist_map = grid::Grid::from_vec(
        map.indexed_iter()
            .map(|((y, x), _)| x.abs_diff(volcano_pos.0).pow(2) + y.abs_diff(volcano_pos.1).pow(2))
            .collect::<Vec<usize>>(),
        width,
    );

    (1..=width / 2)
        .map(|radius| {
            let destruction = dist_map
                .indexed_iter()
                .filter_map(|((x, y), &d)| {
                    (d != 0 && d > (radius - 1).pow(2) && d <= radius.pow(2)).then(|| (map[(y, x)] - b'0') as u64)
                })
                .sum::<u64>();
            (destruction, destruction * radius as u64)
        })
        .max_by_key(|it| it.0)
        .unwrap()
        .1
}

pub fn dijkstra_bounded<FSuccessors, FSuccess, S>(
    start: (usize, usize, bool),
    bounds: (usize, usize),
    max_cost: usize,
    successors: FSuccessors,
    success: FSuccess,
) -> Option<usize>
where
    FSuccessors: Fn(&(usize, usize, bool)) -> S,
    S: IntoIterator<Item = ((usize, usize, bool), usize)>,
    FSuccess: Fn(&(usize, usize, bool)) -> bool,
{
    use ::bucket_queue::*;

    let (width, height) = bounds;
    let capacity = width * height * 2;

    let mut min_costs = vec![usize::MAX; capacity];

    let mut queue = BucketQueue::<Vec<(usize, usize, bool)>>::new();

    let to_index = |(x, y, flag): (usize, usize, bool)| -> usize { ((y * width) + x) * 2 + (flag as usize) };

    let idx = to_index(start);
    min_costs[idx] = 0;
    queue.push(start, 0);

    while !queue.is_empty() {
        let cost = queue.min_priority().unwrap();
        let node = queue.pop_min().unwrap();

        if success(&node) {
            return Some(cost);
        }

        for (neighbor, move_cost) in successors(&node) {
            let new_cost = cost + move_cost;

            if new_cost > max_cost {
                continue;
            }

            let neighbor_idx = to_index(neighbor);

            if new_cost < min_costs[neighbor_idx] {
                min_costs[neighbor_idx] = new_cost;
                queue.push(neighbor, new_cost);
            }
        }
    }

    None
}

#[inline]
pub fn solve_part3() -> impl Display {
    let input = include_str!("part3.txt");
    let width = input.lines().next().unwrap().len();
    let map = grid::Grid::from_vec(input.lines().flat_map(|line| line.bytes()).collect(), width);
    let volcano_pos = map.indexed_iter().find(|&(_, &cell)| cell == b'@').unwrap().0;
    let volcano_pos = (volcano_pos.1, volcano_pos.0);
    let start = map.indexed_iter().find(|&(_, &cell)| cell == b'S').unwrap().0;
    let start = (start.1, start.0);

    let dist_map = grid::Grid::from_vec(
        map.indexed_iter()
            .map(|((y, x), _)| x.abs_diff(volcano_pos.0).pow(2) + y.abs_diff(volcano_pos.1).pow(2))
            .collect::<Vec<usize>>(),
        width,
    );

    (1..width / 2)
        .into_par_iter()
        .find_map_first(|radius| {
            let map = &map;
            let dist_map = &dist_map;

            let mut left = usize::MAX;
            let mut right = usize::MIN;
            let mut bottom = usize::MIN;
            dist_map
                .indexed_iter()
                .filter_map(|((x, y), &d)| (d <= (radius + 1) * (radius + 1)).then_some((x, y)))
                .for_each(|(x, y)| {
                    left = left.min(x);
                    right = right.max(x);
                    bottom = bottom.max(y);
                });

            let cost = dijkstra_bounded(
                (start.0, start.1, false),
                (map.cols(), map.rows()),
                30 * (radius + 1),
                |&(x, y, z)| {
                    [
                        (x.wrapping_sub(1), y),
                        (x.wrapping_add(1), y),
                        (x, y.wrapping_sub(1)),
                        (x, y.wrapping_add(1)),
                    ]
                    .into_iter()
                    .filter(move |&(x, y)| dist_map.get(y, x).is_some_and(|&d| d > radius.pow(2)))
                    .filter_map(move |(nx, ny)| {
                        let nz = if x == volcano_pos.0 && y > volcano_pos.1 {
                            nx > volcano_pos.0
                        } else if nx == volcano_pos.0 && y > volcano_pos.1 {
                            x < volcano_pos.0
                        } else {
                            z
                        };

                        Some(match map.get(ny, nx)? {
                            b @ b'0'..=b'9' => ((nx, ny, nz), (b - b'0') as usize),
                            b'S' => ((nx, ny, nz), 0),
                            _ => return None,
                        })
                    })
                },
                |&(x, y, z)| (x, y) == start && z,
            )?;

            Some(cost * radius)
        })
        .unwrap()
}
