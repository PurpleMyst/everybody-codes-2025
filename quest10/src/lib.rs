use std::fmt::Display;

mod part1;
mod part2;
mod part3;


#[inline]
pub fn solve() -> (impl Display, impl Display, impl Display) {
    (solve_part1(), solve_part2(), solve_part3())
}

#[inline]
pub fn solve_part1() -> impl Display {
    part1::solve()
}

#[inline]
pub fn solve_part2() -> impl Display {
    part2::solve()
}

#[inline]
pub fn solve_part3() -> impl Display {
    part3::solve()
}
