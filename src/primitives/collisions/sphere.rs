use crate::primitives::{Decimal, Ray, Vec3};

pub fn hit_sphere (centre: Vec3, radius: Decimal, ray: Ray) -> Decimal {
    let oc = ray.origin() - centre;

    let a = ray.direction().dot(ray.direction());
    let b = 2.0 * oc.dot(ray.direction());
    let c = radius.mul_add(-radius, oc.dot(oc)); //oc.dot(oc) - radius*radius

    let discriminant = b*b - 4.0*a*c;

    if discriminant < 0.0 {
        -1.0
    } else {
        (-b - discriminant.sqrt()) / (2.0 * a)
    }
}