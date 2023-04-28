use crate::primitives::{
    collisions::{HitRecord, Hittable},
    Decimal, Ray, Vec3,
};

#[derive(Copy, Clone, Debug)]
pub struct Sphere {
    centre: Vec3,
    radius: Decimal,
}

impl Sphere {
    #[must_use]
    pub const fn new(centre: Vec3, radius: Decimal) -> Self {
        Self { centre, radius }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: Ray, t_min: Decimal, t_max: Decimal) -> Option<HitRecord> {
        let oc = ray.origin() - self.centre;

        let a = ray.direction().magnitude_squared();
        let half_b = oc.dot(ray.direction());
        let c = oc.magnitude_squared() - self.radius * self.radius;

        let discriminant = half_b.mul_add(half_b, -a * c); // half_b * half_b - a*c

        if discriminant < 0.0 {
            return None;
        }

        let sqrted = discriminant.sqrt();
        let mut root = (-half_b - sqrted) / a;
        if root < t_min || root > t_max {
            root = (-half_b + sqrted) / a;
            if root < t_min || root > t_max {
                return None;
            }
        }

        Some(HitRecord::new(root, ray, |point| {
            (point - self.centre) / self.radius
        }))
    }
}
