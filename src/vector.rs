use crate::{Angle, Cardinal, Direction, Point, Size, Transform};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::ops::{
    Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Rem, RemAssign, Sub, SubAssign,
};

#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct Vector<T = f32> {
    pub dx: T,
    pub dy: T,
}

impl<T: en::Num> Vector<T> {
    pub fn new(dx: T, dy: T) -> Self {
        Self { dx, dy }
    }

    pub fn uniform(d: T) -> Self {
        Self::new(d, d)
    }

    pub fn from_dx(dx: T) -> Self {
        Self { dx, dy: T::zero() }
    }

    pub fn from_dy(dy: T) -> Self {
        Self { dx: T::zero(), dy }
    }

    pub fn zero() -> Self {
        Self::uniform(T::zero())
    }

    pub fn one() -> Self {
        Self::uniform(T::one())
    }

    pub fn from_array([dx, dy]: [T; 2]) -> Self {
        Self::new(dx, dy)
    }

    pub fn from_tuple((dx, dy): (T, T)) -> Self {
        Self::new(dx, dy)
    }

    pub fn with_dx(self, dx: T) -> Self {
        Self::new(dx, self.dy)
    }

    pub fn with_dy(self, dy: T) -> Self {
        Self::new(self.dx, dy)
    }

    pub fn dot_product(self, rhs: Self) -> T {
        self.dx * rhs.dx + self.dy * rhs.dy
    }

    pub fn magnitude_squared(self) -> T {
        self.dot_product(self)
    }

    pub fn magnitude(self) -> T
    where
        T: en::Float,
    {
        self.magnitude_squared().sqrt()
    }

    pub fn normalize(self) -> Self
    where
        T: en::Float,
    {
        self / self.magnitude()
    }

    pub fn angle(self) -> Angle<T>
    where
        T: en::Float,
    {
        Angle::from_xy(self.dx, self.dy)
    }

    pub fn scaled(self, rhs: Size<T>) -> Self {
        Self::new(self.dx * rhs.width(), self.dy * rhs.height())
    }

    pub fn perpendicular(self) -> Self
    where
        T: Neg<Output = T>,
    {
        Self::new(-self.dy, self.dx)
    }

    pub fn yx(self) -> Self {
        Self::new(self.dy, self.dx)
    }

    pub fn transform(self, transform: Transform<T>) -> Self {
        transform.transform_vector(self)
    }

    pub fn map<U: en::Num>(&self, mut f: impl FnMut(T) -> U) -> Vector<U> {
        Vector::new(f(self.dx), f(self.dy))
    }

    pub fn map_dx(&self, mut f: impl FnMut(T) -> T) -> Self {
        self.with_dx(f(self.dx))
    }

    pub fn map_dy(&self, mut f: impl FnMut(T) -> T) -> Self {
        self.with_dy(f(self.dy))
    }

    impl_casts_and_cast!(Vector);

    pub fn to_array(self) -> [T; 2] {
        [self.dx, self.dy]
    }

    pub fn to_tuple(self) -> (T, T) {
        (self.dx, self.dy)
    }

    pub fn to_point(self) -> Point<T> {
        Point::zero() + self
    }

    pub fn to_size(self) -> Size<T> {
        self.into()
    }
}

impl<T: en::Num> From<Size<T>> for Vector<T> {
    fn from(size: Size<T>) -> Self {
        Self::new(size.width(), size.height())
    }
}

impl<T: en::Num> Add for Vector<T> {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.dx + rhs.dx, self.dy + rhs.dy)
    }
}

impl<T: en::Num> AddAssign for Vector<T> {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs
    }
}

impl<T: en::Num> Sub for Vector<T> {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(self.dx - rhs.dx, self.dy - rhs.dy)
    }
}

impl<T: en::Num> SubAssign for Vector<T> {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs
    }
}

impl<T: en::Num> Mul for Vector<T> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self::new(self.dx * rhs.dx, self.dy * rhs.dy)
    }
}

impl<T: en::Num> MulAssign for Vector<T> {
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs
    }
}

impl<T: en::Num> Mul<T> for Vector<T> {
    type Output = Self;
    fn mul(self, rhs: T) -> Self::Output {
        self.map(move |x| x * rhs)
    }
}

impl<T: en::Num> MulAssign<T> for Vector<T> {
    fn mul_assign(&mut self, rhs: T) {
        *self = *self * rhs
    }
}

impl<T: en::Num> Div for Vector<T> {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Self::new(self.dx / rhs.dx, self.dy / rhs.dy)
    }
}

impl<T: en::Num> DivAssign for Vector<T> {
    fn div_assign(&mut self, rhs: Self) {
        *self = *self / rhs
    }
}

impl<T: en::Num> Div<T> for Vector<T> {
    type Output = Self;
    fn div(self, rhs: T) -> Self::Output {
        self.map(move |x| x / rhs)
    }
}

impl<T: en::Num> DivAssign<T> for Vector<T> {
    fn div_assign(&mut self, rhs: T) {
        *self = *self / rhs
    }
}

impl<T: en::Num> Rem for Vector<T> {
    type Output = Self;

    fn rem(self, rhs: Self) -> Self::Output {
        Self::new(self.dx % rhs.dx, self.dy % rhs.dy)
    }
}

impl<T: en::Num> RemAssign for Vector<T> {
    fn rem_assign(&mut self, rhs: Self) {
        *self = *self % rhs
    }
}

impl<T: en::Num> Rem<T> for Vector<T> {
    type Output = Self;
    fn rem(self, rhs: T) -> Self::Output {
        self.map(move |x| x % rhs)
    }
}

impl<T: en::Num> RemAssign<T> for Vector<T> {
    fn rem_assign(&mut self, rhs: T) {
        *self = *self % rhs
    }
}

impl<T: Neg<Output = T> + en::Num> Neg for Vector<T> {
    type Output = Self;
    fn neg(self) -> Self::Output {
        self.map(move |x| -x)
    }
}

impl<T: en::Float> From<Direction> for Vector<T> {
    fn from(direction: Direction) -> Self {
        direction.angle().unit_vector()
    }
}

impl<T: en::Float> From<Cardinal> for Vector<T> {
    fn from(cardinal: Cardinal) -> Self {
        cardinal.angle().unit_vector()
    }
}

#[cfg(feature = "euclid")]
impl<T, U> From<Vector<T>> for euclid::Vector2D<T, U> {
    fn from(v: Vector<T>) -> euclid::Vector2D<T, U> {
        Self::new(v.dx, v.dy)
    }
}

#[cfg(feature = "euclid")]
impl<T, U> From<euclid::Vector2D<T, U>> for Vector<T> {
    fn from(v: euclid::Vector2D<T, U>) -> Vector<T> {
        Self { dx: v.x, dy: v.y }
    }
}
