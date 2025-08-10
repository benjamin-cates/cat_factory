#[derive(Copy, PartialEq, Eq, Hash, Ord, PartialOrd)]
#[turbo::serialize]
pub struct Point(i32, i32);

impl Point {
    pub fn x(&self) -> i32 {
        self.0
    }
    pub fn y(&self) -> i32 {
        self.1
    }
}
impl Display for Point {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        f.write_fmt(format_args!("({}, {})", self.0, self.1))
    }
}
impl<TR: Into<Point>> std::ops::Add<TR> for Point {
    type Output = Point;
    fn add(self, rhs: TR) -> Self::Output {
        let rhs: Point = rhs.into();
        let lhs: Point = self.into();
        Point(lhs.0 + rhs.0, lhs.1 + rhs.1)
    }
}
impl<TR: Into<Point>> std::ops::Add<TR> for &Point {
    type Output = Point;
    fn add(self, rhs: TR) -> Self::Output {
        let rhs: Point = rhs.into();
        let lhs: Point = self.into();
        Point(lhs.0 + rhs.0, lhs.1 + rhs.1)
    }
}
impl<T: Into<Point>> std::ops::AddAssign<T> for Point {
    fn add_assign(&mut self, rhs: T) {
        let point = rhs.into();
        self.0 += point.0;
        self.1 += point.1;
    }
}
impl<TR: Into<Point>> std::ops::Sub<TR> for Point {
    type Output = Point;
    fn sub(self, rhs: TR) -> Self::Output {
        let rhs: Point = rhs.into();
        let lhs: Point = self.into();
        Point(lhs.0 - rhs.0, lhs.1 - rhs.1)
    }
}
impl<TR: Into<Point>> std::ops::Sub<TR> for &Point {
    type Output = Point;
    fn sub(self, rhs: TR) -> Self::Output {
        let rhs: Point = rhs.into();
        let lhs: Point = self.into();
        Point(lhs.0 - rhs.0, lhs.1 - rhs.1)
    }
}
impl<T: Into<Point>> std::ops::SubAssign<T> for Point {
    fn sub_assign(&mut self, rhs: T) {
        let point = rhs.into();
        self.0 -= point.0;
        self.1 -= point.1;
    }
}
impl std::ops::Mul<i32> for Point {
    type Output = Point;
    fn mul(self, rhs: i32) -> Self::Output {
        Point(self.0 * rhs, self.1 * rhs)
    }
}
impl std::ops::Mul<i32> for &Point {
    type Output = Point;
    fn mul(self, rhs: i32) -> Self::Output {
        Point(self.0 * rhs, self.1 * rhs)
    }
}
impl std::ops::MulAssign<i32> for Point {
    fn mul_assign(&mut self, rhs: i32) {
        self.0 *= rhs;
        self.1 *= rhs;
    }
}
impl From<(i32, i32)> for Point {
    fn from(value: (i32, i32)) -> Self {
        Point(value.0, value.1)
    }
}
impl From<Point> for (i32, i32) {
    fn from(value: Point) -> Self {
        (value.0, value.1)
    }
}
impl From<&Point> for Point {
    fn from(value: &Point) -> Self {
        *value
    }
}
impl From<(usize, usize)> for Point {
    fn from(value: (usize, usize)) -> Self {
        Point(value.0 as i32, value.1 as i32)
    }
}
#[derive(Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[turbo::serialize]
pub enum Direction {
    North,
    South,
    East,
    West,
}

use std::fmt::{Display, Formatter};

use Direction::*;

impl From<Direction> for Point {
    fn from(value: Direction) -> Self {
        match value {
            North => (0, -1),
            South => (0, 1),
            East => (1, 0),
            West => (-1, 0),
        }
        .into()
    }
}
impl From<&Direction> for Point {
    fn from(value: &Direction) -> Self {
        match *value {
            North => (0, -1),
            South => (0, 1),
            East => (1, 0),
            West => (-1, 0),
        }
        .into()
    }
}

impl Direction {
    pub fn iter_all() -> impl Iterator<Item = Direction> {
        [North, South, East, West].into_iter()
    }
    pub fn opposite(&self) -> Direction {
        match self {
            North => South,
            South => North,
            East => West,
            West => East,
        }
    }
    pub fn deflect_slash(&self) -> Direction {
        match self {
            North => East,
            South => West,
            West => South,
            East => North,
        }
    }
    pub fn deflect_backslash(&self) -> Direction {
        match self {
            North => West,
            South => East,
            East => South,
            West => North,
        }
    }
    pub fn step(&self) -> Point {
        (*self).into()
    }

    pub fn rotate_right(&self) -> Direction {
        match self {
            North => East,
            East => South,
            South => West,
            West => North,
        }
    }
    pub fn rotate_left(&self) -> Direction {
        match self {
            North => West,
            East => North,
            South => East,
            West => South,
        }
    }
}
impl TryFrom<char> for Direction {
    type Error = ();
    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'R' => Ok(East),
            'D' => Ok(South),
            'L' => Ok(West),
            'U' => Ok(North),
            'r' => Ok(East),
            'd' => Ok(South),
            'l' => Ok(West),
            'u' => Ok(North),
            'v' => Ok(South),
            '^' => Ok(North),
            '>' => Ok(East),
            '<' => Ok(West),
            _ => Err(()),
        }
    }
}
