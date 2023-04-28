use crate::primitives::{Decimal, Colour, Vec3};
use crate::primitives::collisions::sphere::hit_sphere;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Ray {
    origin: Vec3,
    direction: Vec3
}

impl Ray {
    #[must_use] pub const fn new(origin: Vec3, direction: Vec3) -> Self {
        Self { origin, direction }
    }

    #[must_use] pub const fn origin(&self) -> Vec3 {
        self.origin
    }
    #[must_use] pub const fn direction(&self) -> Vec3 {
        self.direction
    }
    #[must_use] pub fn at (&self, t: Decimal) -> Vec3 {
        self.origin + self.direction * t
    }

    #[must_use] pub fn colour(&self) -> Colour {
        let sphere = hit_sphere(Vec3::new(0.0, 0.0, -1.0), 0.5, *self);
        if sphere > 0.0 {
            let normal = (self.at(sphere) - Vec3::new(0.0, 0.0, -1.0)).unit();
            return 0.5 * Vec3::new(normal.x() + 1.0, normal.y() + 1.0, normal.z() + 1.0);
        }

        let unit = self.direction.unit();
        let t = 0.5 * (unit.y() + 1.0);
        Colour::new(1.0, 1.0, 1.0) * (1.0 - t) + Colour::new(0.5, 0.7, 1.0) * t
    }
}