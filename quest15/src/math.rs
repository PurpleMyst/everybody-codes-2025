use std::mem::swap;

use std::ops::AddAssign;

use std::ops::Add;

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

impl AddAssign for Vec2 {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
        self.1 += rhs.1;
    }
}

impl Vec2 {
    pub(crate) fn rotate_right(&mut self) {
        // (x + y * i) * i = -y + x * i
        self.1 = -self.1;
        swap(&mut self.0, &mut self.1);
    }

    pub(crate) fn rotate_left(&mut self) {
        // (x + y * i) * (-i) = y - x * i
        self.0 = -self.0;
        swap(&mut self.0, &mut self.1);
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

    pub(crate) fn opposite_dir(&self, other: Vec2) -> bool {
        (self.is_horizontal() && other.is_horizontal() && self.0 == -other.0)
            || (self.is_vertical() && other.is_vertical() && self.1 == -other.1)
    }

    pub(crate) fn mag(&self) -> i64 {
        self.0.abs() + self.1.abs()
    }
}

#[inline(always)]
pub(crate) fn vec2(x: i64, y: i64) -> Vec2 {
    Vec2(x, y)
}

#[cfg(test)]
mod tests {
    use super::Vec2;

    #[test]
    fn test_rotate() {
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
