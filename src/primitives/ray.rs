use crate::primitives::{Backing, Vec3};

pub struct Ray {
    origin: Vec3,
    direction: Vec3
}

impl Ray {
    #[must_use] pub const fn new(origin: Vec3, direction: Vec3) -> Self {
        Self { origin, direction }
    }

    pub const fn origin(&self) -> Vec3 {
        self.origin
    }
    pub const fn direction(&self) -> Vec3 {
        self.direction
    }
    pub fn at (&self, t: Backing) -> Vec3 {
        self.origin + self.direction * t
    }
}