use crate::primitives::{Decimal, Ray, Vec3};

#[must_use] pub fn hit_sphere (centre: Vec3, radius: Decimal, ray: Ray) -> Decimal {
    let oc = ray.origin() - centre;

    let a = ray.direction().magnitude_squared();
    let half_b = oc.dot(ray.direction());
    let c = oc.magnitude_squared() - radius*radius;

    let discriminant = half_b.mul_add(half_b, -a * c); // half_b * half_b - a*c

    if discriminant < 0.0 {
        -1.0
    } else {
        (-half_b - discriminant.sqrt()) / a
    }
}