#![allow(dead_code)]

use std::mem::swap;
use std::ops::AddAssign;
use std::ops::{Add, Sub, Mul, SubAssign};

type Ratio = num_rational::Ratio<i64>;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) struct Vec2(pub(crate) i64, pub(crate) i64);

impl std::fmt::Debug for Vec2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        (self.0, self.1).fmt(f)
    }
}

impl Add for Vec2 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl Sub for Vec2 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0, self.1 - rhs.1)
    }
}

impl AddAssign for Vec2 {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
        self.1 += rhs.1;
    }
}

impl SubAssign for Vec2 {
    fn sub_assign(&mut self, rhs: Self) {
        self.0 -= rhs.0;
        self.1 -= rhs.1;
    }
}

impl Mul<i64> for Vec2 {
    type Output = Self;

    fn mul(self, rhs: i64) -> Self::Output {
        Self(self.0 * rhs, self.1 * rhs)
    }
}

impl Vec2 {
    pub(crate) fn rotate_right(&mut self) {
        // (x + y * i) * i = -y + x * i
        self.1 = -self.1;
        swap(&mut self.0, &mut self.1);
    }

    #[must_use]
    pub(crate) fn rotated_right(&self) -> Vec2 {
        let mut v = *self;
        v.rotate_right();
        v
    }

    pub(crate) fn rotate_left(&mut self) {
        // (x + y * i) * (-i) = y - x * i
        self.0 = -self.0;
        swap(&mut self.0, &mut self.1);
    }

    #[must_use]
    pub(crate) fn rotated_left(&self) -> Self {
        let mut v = *self;
        v.rotate_left();
        v
    }

    pub(crate) fn dist(&self, other: &Self) -> u64 {
        self.0.abs_diff(other.0) + self.1.abs_diff(other.1)
    }

    fn is_horizontal(&self) -> bool {
        self.0 != 0 && self.1 == 0
    }

    fn is_vertical(&self) -> bool {
        self.0 == 0 && self.1 != 0
    }

    pub(crate) fn same_dir(&self, other: Vec2) -> bool {
        (self.is_horizontal() && other.is_horizontal()) || (self.is_vertical() && other.is_vertical())
    }

    pub(crate) fn mag(&self) -> i64 {
        self.0.abs() + self.1.abs()
    }

    #[must_use]
    pub(crate) fn normalized(&self) -> Self {
        let mag = self.mag();
        debug_assert!(mag != 0);
        Vec2(self.0 / mag, self.1 / mag)
    }

    pub(crate) fn is_zero(&self) -> bool {
        self.0 == 0 && self.1 == 0
    }
}

#[inline(always)]
pub(crate) fn vec2(x: i64, y: i64) -> Vec2 {
    Vec2(x, y)
}

/// Line passing through two points, defined/used parametrically as the following locus of points:
/// { start + t * (end - start) | 0 ≤ t ≤ 1 }
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) struct Segment {
    pub(crate) start: Vec2,
    pub(crate) end: Vec2,
}

#[inline(always)]
pub(crate) fn segment(start: Vec2, end: Vec2) -> Segment {
    let this = Segment { start, end };
    if this.is_horizontal() {
        // x = x0 + t(x1 - x0); y = y0
    } else if this.is_vertical() {
        // y = y0 + t(y1 - y0); x = x0
    } else {
        panic!("Only horizontal/vertical lines allowed");
    }

    this
}

#[inline(always)]
pub(crate) fn segment_delta(start: Vec2, delta: Vec2) -> Segment {
    segment(start, start + delta)
}

impl std::fmt::Debug for Segment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}--{:?}", self.start, self.end)
    }
}


impl Segment {
    fn is_horizontal(&self) -> bool {
        self.start.1 == self.end.1
    }

    fn is_vertical(&self) -> bool {
        self.start.0 == self.end.0
    }

    pub(crate) fn intersection(&self, other: &Segment) -> Option<Vec2> {
        if self.is_horizontal() && other.is_vertical() {
            let y = self.start.1;
            let x = other.start.0;
            let x0 = self.start.0.min(self.end.0);
            let x1 = self.start.0.max(self.end.0);
            let y0 = other.start.1.min(other.end.1);
            let y1 = other.start.1.max(other.end.1);
            if x >= x0 && x <= x1 && y >= y0 && y <= y1 {
                Some(Vec2(x, y))
            } else {
                None
            }
        } else if self.is_vertical() && other.is_horizontal() {
            other.intersection(self)
        } else if self.is_horizontal() && other.is_horizontal() {
            let y = self.start.1;
            if y != other.start.1 {
                return None;
            }
            let x0 = self.start.0.min(self.end.0);
            let x1 = self.start.0.max(self.end.0);
            let ox0 = other.start.0.min(other.end.0);
            let ox1 = other.start.0.max(other.end.0);
            let ix0 = x0.max(ox0);
            let ix1 = x1.min(ox1);
            if ix0 <= ix1 {
                Some(Vec2(ix0, y)) // Return the start of the intersection segment
            } else {
                None
            }
        } else if self.is_vertical() && other.is_vertical() {
            let x = self.start.0;
            if x != other.start.0 {
                return None;
            }
            let y0 = self.start.1.min(self.end.1);
            let y1 = self.start.1.max(self.end.1);
            let oy0 = other.start.1.min(other.end.1);
            let oy1 = other.start.1.max(other.end.1);
            let iy0 = y0.max(oy0);
            let iy1 = y1.min(oy1);
            if iy0 <= iy1 {
                Some(Vec2(x, iy0)) // Return the start of the intersection segment
            } else {
                None
            }
        } else {
            unreachable!()
        }
    }

    pub(crate) fn intersects_any(&self, others: &[Segment]) -> bool {
        others.iter().any(|other| self.intersection(other).is_some())
    }

    pub(crate) fn intersects_none(&self, others: &[Segment]) -> bool {
        !self.intersects_any(others)
    }

    pub(crate) fn contains(&self, point: Vec2) -> bool {
        if self.is_horizontal() {
            let y = self.start.1;
            if point.1 != y {
                return false;
            }
            let x0 = self.start.0.min(self.end.0);
            let x1 = self.start.0.max(self.end.0);
            point.0 >= x0 && point.0 <= x1
        } else if self.is_vertical() {
            let x = self.start.0;
            if point.0 != x {
                return false;
            }
            let y0 = self.start.1.min(self.end.1);
            let y1 = self.start.1.max(self.end.1);
            point.1 >= y0 && point.1 <= y1
        } else {
            false
        }
    }

    /// Returns the point on the segment with the given x-coordinate, if it exists.
    pub(crate) fn point_with_x(&self, y: i64) -> Option<Vec2> {
        if self.is_horizontal() {
            let x0 = self.start.0.min(self.end.0);
            let x1 = self.start.0.max(self.end.0);
            if y >= x0 && y <= x1 {
                Some(Vec2(y, self.start.1))
            } else {
                None
            }
        } else if self.is_vertical() {
            if self.start.0 == y {
                Some(self.start)
            } else {
                None
            }
        } else {
            unreachable!()
        }
    }

    /// Returns the point on the segment with the given y-coordinate, if it exists.
    pub(crate) fn point_with_y(&self, y: i64) -> Option<Vec2> {
        if self.is_horizontal() {
            if self.start.1 == y {
                Some(self.start)
            } else {
                None
            }
        } else if self.is_vertical() {
            let y0 = self.start.1.min(self.end.1);
            let y1 = self.start.1.max(self.end.1);
            if y >= y0 && y <= y1 {
                Some(Vec2(self.start.0, y))
            } else {
                None
            }
        } else {
            unreachable!()
        }
    }

}

impl IntoIterator for Segment {
    type Item = Vec2;
    type IntoIter = LineIterator;

    // #[track_caller]
    fn into_iter(self) -> Self::IntoIter {
        // let loc = std::panic::Location::caller();
        // eprintln!("[{}:{}:{}] ⚠️ THIS IS SLOW ⚠️ ", loc.file(), loc.line(), loc.column());
        LineIterator {
            line: self,
            current: self.start,
            step: self
                .end
                .sub(self.start)
                .normalized(),
            done: false,
        }
    }
}

pub(crate) struct LineIterator {
    line: Segment,
    current: Vec2,
    step: Vec2,
    done: bool,
}

impl Iterator for LineIterator {
    type Item = Vec2;

    fn next(&mut self) -> Option<Self::Item> {
        if self.done {
            return None;
        }

        let result = self.current;

        if result == self.line.end {
            self.done = true;
        } else {
            self.current += self.step;
        }

        Some(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rotate_frame() {
        let up = Vec2(0, -1);
        let right = Vec2(1, 0);
        let down = Vec2(0, 1);
        let left = Vec2(-1, 0);

        let mut v = up;
        v.rotate_right();
        assert_eq!(v, right);
        v.rotate_right();
        assert_eq!(v, down);
        v.rotate_right();
        assert_eq!(v, left);
        v.rotate_right();
        assert_eq!(v, up);

        let mut v = up;
        v.rotate_left();
        assert_eq!(v, left);
        v.rotate_left();
        assert_eq!(v, down);
        v.rotate_left();
        assert_eq!(v, right);
        v.rotate_left();
        assert_eq!(v, up);
    }
}
