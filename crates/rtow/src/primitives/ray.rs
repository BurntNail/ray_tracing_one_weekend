use crate::primitives::{collisions::Hittable, Colour, Decimal, Vec3};
use rand::rngs::ThreadRng;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Ray {
    origin: Vec3,
    direction: Vec3,
}

impl Ray {
    #[must_use]
    pub const fn new(origin: Vec3, direction: Vec3) -> Self {
        Self { origin, direction }
    }

    #[must_use]
    pub const fn origin(&self) -> Vec3 {
        self.origin
    }
    #[must_use]
    pub const fn direction(&self) -> Vec3 {
        self.direction
    }
    #[must_use]
    pub fn at(&self, t: Decimal) -> Vec3 {
        self.origin + self.direction * t
    }

    #[must_use]
    pub fn colour(&self, world: &dyn Hittable, depth: usize, rng: &mut ThreadRng) -> Colour {
        if depth == 0 {
            return Colour::default();
        }

        if let Some(hit) = world.hit(*self, 0.00001, Decimal::INFINITY) {
            return if let Some((attenuation, scattered)) = hit.material.scatter(*self, hit, rng) {
                attenuation * scattered.colour(world, depth - 1, rng)
            } else {
                Colour::default()
            };
        }

        let unit = self.direction.unit();
        let t = 0.5 * (unit.y() + 1.0);
        Colour::new(1.0, 1.0, 1.0) * (1.0 - t) + Colour::new(0.5, 0.7, 1.0) * t
    }
}
