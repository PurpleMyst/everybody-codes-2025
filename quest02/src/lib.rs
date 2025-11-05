use std::{
    fmt::Display,
    ops::{Add, Div, Mul},
};

#[derive(Clone, Copy)]
pub struct Complex {
    pub real: i64,
    pub imag: i64,
}

impl Complex {
    pub fn new(real: i64, imag: i64) -> Self {
        Self { real, imag }
    }

    pub fn zero() -> Self {
        Self::new(0, 0)
    }
}

impl Display for Complex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{},{}]", self.real, self.imag)
    }
}

impl Add for Complex {
    type Output = Self;

    fn add(self, other: Complex) -> Self::Output {
        Self {
            real: self.real + other.real,
            imag: self.imag + other.imag,
        }
    }
}

impl Mul for Complex {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            real: self.real * rhs.real - self.imag * rhs.imag,
            imag: self.real * rhs.imag + self.imag * rhs.real,
        }
    }
}

impl Div for Complex {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Self {
            real: self.real / rhs.real,
            imag: self.imag / rhs.imag,
        }
    }
}

#[inline]
pub fn solve() -> (impl Display, impl Display, impl Display) {
    (solve_part1(), solve_part2(), solve_part3())
}

pub fn parse_a(input: &str) -> Complex {
    let (a_r, a_i) = input
        .trim()
        .strip_prefix("A=[")
        .unwrap()
        .strip_suffix("]")
        .unwrap()
        .split_once(',')
        .unwrap();
    let a_r = a_r.parse::<i64>().unwrap();
    let a_i = a_i.parse::<i64>().unwrap();
    Complex::new(a_r, a_i)
}

#[inline]
pub fn solve_part1() -> impl Display {
    let a = parse_a(include_str!("part1.txt"));
    let divisor = Complex::new(10, 10);
    let mut result = Complex::zero();
    for _ in 0..3 {
        result = (result * result) / divisor + a;
    }
    result
}

#[inline]
pub fn solve_part2() -> impl Display {
    let a = parse_a(include_str!("part2.txt"));
    let step = 10;

    count_engraved(a, step)
}

fn count_engraved(a: Complex, step: usize) -> usize {
    let mut result = 0usize;

    for y in (0..=1000).step_by(step) {
        'cell: for x in (0..=1000).step_by(step) {
            let b = Complex::new(a.real + x, a.imag + y);

            let mut check_result = Complex::zero();
            let divisor = Complex::new(100000, 100000);
            for _ in 0..100 {
                check_result = (check_result * check_result) / divisor + b;
                if check_result.real.abs() > 1000000 || check_result.imag.abs() > 1000000 {
                    continue 'cell;
                }
            }

            result += 1;
        }
    }

    result
}

#[inline]
pub fn solve_part3() -> impl Display {
    let a = parse_a(include_str!("part3.txt"));
    let step = 1;

    count_engraved(a, step)
}
