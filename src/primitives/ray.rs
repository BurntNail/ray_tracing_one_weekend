use crate::primitives::{Decimal, Colour, Vec3};

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
    pub fn at (&self, t: Decimal) -> Vec3 {
        self.origin + self.direction * t
    }

    #[must_use] pub fn sky_colour (&self) -> Colour {
        let unit = self.direction.unit();
        let t = 0.5 * (unit.y() + 1.0);
        Colour::new(1.0, 1.0, 1.0) * (1.0 - t) + Colour::new(0.5, 0.7, 1.0) * t
    }
}