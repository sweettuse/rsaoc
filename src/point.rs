use std::{ops::{Add, Deref, Sub, Neg, Mul, Rem}, collections::HashMap};

use glam::IVec2;
use once_cell::sync::Lazy;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

pub type Point = IVec2;


#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy, EnumIter)]
pub enum Dir {
    North,
    East,
    South,
    West,
}

const _LOOKUP: [Dir; 7] = [Dir::North, Dir::East, Dir::South, Dir::West, Dir::North, Dir::East, Dir::South];
static _IDX_MAP: Lazy<HashMap<Dir, usize>> = Lazy::new(|| {
    let mut res = HashMap::new();
    res.insert(Dir::North, 0);
    res.insert(Dir::East, 1);
    res.insert(Dir::South, 2);
    res.insert(Dir::West, 3);
    res
});


impl Dir {
    pub fn rotate(&self, num_rotations: i32) -> Self {
        let normalized = num_rotations.rem_euclid(4) as usize;
        _LOOKUP[_IDX_MAP[self] + normalized]
    }

    pub fn offset(&self) -> Point {
        match self {
            Dir::North => Point::new(0, -1),
            Dir::South => Point::new(0, 1),
            Dir::East => Point::new(1, 0),
            Dir::West => Point::new(-1, 0),
        }
    }
}

impl Add<Dir> for Dir {
    type Output = Point;
    fn add(self, rhs: Dir) -> Self::Output {
        self.offset() + rhs.offset()
    }
}

impl Add<Dir> for Point {
    type Output = Point;

    fn add(self, rhs: Dir) -> Self::Output {
        self + rhs.offset()
    }
}

impl Sub<Dir> for Point {
    type Output = Point;

    fn sub(self, rhs: Dir) -> Self::Output {
        self - rhs.offset()
    }
}

impl Mul<i32> for Dir {
    type Output = Point;

    fn mul(self, rhs: i32) -> Self::Output {
        rhs * self.offset()
    }
}

impl Mul<Dir> for i32 {
    type Output = Point;

    fn mul(self, rhs: Dir) -> Self::Output {
        rhs * self
    }
}

impl Neg for Dir {
    type Output = Dir;

    fn neg(self) -> Self::Output {
        match self {
            Dir::North => Dir::South,
            Dir::South => Dir::North,
            Dir::East => Dir::West,
            Dir::West => Dir::East,
        }
    }
}

pub fn point_inclusive_mod(p1: &Point, p2:&Point) -> Point {
    Point::new(p1.x.rem_euclid(p2.x + 1), p1.y.rem_euclid(p2.y + 1))
}

#[cfg(test)]
mod test {

    use std::iter::zip;

    use crate::point;

    use super::*;

    #[test]
    fn test_dir_rotation() {
        zip(Dir::iter(), 0..).for_each(|(d, r)| {
            let new_dir = Dir::North.rotate(r);
            assert_eq!(new_dir, d);
        });
        zip(Dir::iter(), 4..).for_each(|(d, r)| {
            let new_dir = Dir::North.rotate(r);
            assert_eq!(new_dir, d);
        });
        assert_eq!(Dir::West.rotate(-1), Dir::South);
        assert_eq!(Dir::West.rotate(-2), Dir::East);
        assert_eq!(Dir::West.rotate(-3), Dir::North);
        assert_eq!(Dir::West.rotate(-7 * 7 * 7), Dir::North);
    }

    #[test]
    fn test_point_mod() {
        let mut p = Point::new(0, 0);
        let bound = Point::new(4, 5);
        assert_eq!(point_inclusive_mod(&p, &bound), p);
        p = Point::new(4, 5);
        assert_eq!(point_inclusive_mod(&p, &bound), p);
        p = Point::new(5, 6);
        assert_eq!(point_inclusive_mod(&p, &bound), Point::new(0, 0));
        p = Point::new(-1, -1);
        assert_eq!(point_inclusive_mod(&p, &bound), Point::new(4, 5));
    }
}