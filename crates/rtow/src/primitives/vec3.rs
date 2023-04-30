use crate::{io::images::Pixel, primitives::Decimal};
use rand::{thread_rng, Rng};
use std::{
    fmt::{Display, Formatter},
    ops::{Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Neg, Sub, SubAssign},
};

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Vec3 {
    x: Decimal,
    y: Decimal,
    z: Decimal,
}

impl Display for Vec3 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({},{},{})", self.x, self.y, self.z)
    }
}

impl Vec3 {
    #[must_use]
    pub const fn new(x: Decimal, y: Decimal, z: Decimal) -> Self {
        Self { x, y, z }
    }

    #[must_use]
    pub const fn x(&self) -> Decimal {
        self.x
    }
    #[must_use]
    pub const fn y(&self) -> Decimal {
        self.y
    }
    #[must_use]
    pub const fn z(&self) -> Decimal {
        self.z
    }

    #[must_use]
    pub fn magnitude_squared(&self) -> Decimal {
        self.x * self.x + self.y * self.y + self.z * self.z
    }
    #[must_use]
    pub fn magnitude(&self) -> Decimal {
        self.magnitude_squared().sqrt()
    }
    #[must_use]
    pub fn unit(&self) -> Self {
        *self / self.magnitude()
    }

    #[must_use]
    pub fn dot(&self, Self { x, y, z }: Self) -> Decimal {
        self.x * x + self.y * y + self.z * z
    }
    #[must_use]
    pub fn cross(&self, Self { x, y, z }: Self) -> Self {
        Self::new(
            self.y * z - self.z * y,
            self.z * x - self.x * z,
            self.x * y - self.y * x,
        )
    }

    #[must_use]
    pub fn random() -> Self {
        Self::random_range(0.0, 1.0)
    }
    #[must_use]
    pub fn random_range(min: Decimal, max: Decimal) -> Self {
        let mut rng = thread_rng();
        Self::new(
            rng.gen_range(min..=max),
            rng.gen_range(min..=max),
            rng.gen_range(min..=max),
        )
    }
    #[must_use]
    pub fn random_in_unit_sphere() -> Self {
        loop {
            let p = Self::random_range(-1.0, 1.0);
            if p.magnitude_squared() < 1.0 {
                return p;
            }
        }
    }
    #[must_use]
    pub fn random_unit_vector() -> Self {
        Self::random_in_unit_sphere().unit()
    }
    #[must_use]
    pub fn random_in_hemisphere(normal: Self) -> Self {
        let in_unit_sphere = Self::random_in_unit_sphere();
        if in_unit_sphere.dot(normal) > 0.0 {
            in_unit_sphere
        } else {
            -in_unit_sphere
        }
    }

    #[must_use]
    pub fn near_zero(&self) -> bool {
        self.x.abs() < Decimal::EPSILON
            && self.y.abs() < Decimal::EPSILON
            && self.z.abs() < Decimal::EPSILON
    }

    #[must_use]
    pub fn reflect(&self, normal: Self) -> Self {
        *self - (2.0 * self.dot(normal) * normal)
    }
    #[must_use]
    pub fn refract(&self, normal: Self, i_over_r: Decimal) -> Self {
        let cos_theta = (-*self).dot(normal).min(1.0);
        let r_out_perp = i_over_r * (*self + cos_theta * normal);
        let r_out_parallel = -(1.0 - r_out_perp.magnitude_squared()).abs().sqrt() * normal;
        r_out_perp + r_out_parallel
    }
}

impl Add for Vec3 {
    type Output = Self;

    fn add(self, Self { x, y, z }: Self) -> Self::Output {
        Self::new(self.x + x, self.y + y, self.z + z)
    }
}
impl AddAssign for Vec3 {
    fn add_assign(&mut self, Self { x, y, z }: Self) {
        self.x += x;
        self.y += y;
        self.z += z;
    }
}

impl Sub for Vec3 {
    type Output = Self;

    fn sub(self, Self { x, y, z }: Self) -> Self::Output {
        Self::new(self.x - x, self.y - y, self.z - z)
    }
}
impl SubAssign for Vec3 {
    fn sub_assign(&mut self, Self { x, y, z }: Self) {
        self.x -= x;
        self.y -= y;
        self.z -= z;
    }
}
impl Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        let Self { x, y, z } = self;
        Self::new(-x, -y, -z)
    }
}

impl Mul<Decimal> for Vec3 {
    type Output = Self;

    fn mul(self, rhs: Decimal) -> Self::Output {
        let Self { x, y, z } = self;
        Self::new(x * rhs, y * rhs, z * rhs)
    }
}
impl MulAssign<Decimal> for Vec3 {
    fn mul_assign(&mut self, rhs: Decimal) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
    }
}
impl Mul<Vec3> for Decimal {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        rhs * self
    }
}
impl Mul for Vec3 {
    type Output = Self;

    fn mul(self, Self { x, y, z }: Self) -> Self::Output {
        Self::new(self.x * x, self.y * y, self.z * z)
    }
}
impl MulAssign for Vec3 {
    fn mul_assign(&mut self, Self { x, y, z }: Self) {
        self.x *= x;
        self.y *= y;
        self.z *= z;
    }
}

impl Div<Decimal> for Vec3 {
    type Output = Self;

    fn div(self, rhs: Decimal) -> Self::Output {
        let Self { x, y, z } = self;
        Self::new(x / rhs, y / rhs, z / rhs)
    }
}
impl DivAssign<Decimal> for Vec3 {
    fn div_assign(&mut self, rhs: Decimal) {
        self.x /= rhs;
        self.y /= rhs;
        self.z /= rhs;
    }
}

impl Index<usize> for Vec3 {
    type Output = Decimal;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!("index vec3 oob"),
        }
    }
}
impl IndexMut<usize> for Vec3 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            _ => panic!("index vec3 oob"),
        }
    }
}

impl Pixel for Vec3 {
    fn rgb(&self) -> [Decimal; 3] {
        [self.x, self.y, self.z]
    }
}
