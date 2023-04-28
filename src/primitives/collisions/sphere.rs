use crate::primitives::{Decimal, Ray, Vec3};

pub fn hit_sphere (centre: Vec3, radius: Decimal, ray: Ray) -> bool {
    let oc = ray.origin() - centre;
    
    let a = ray.direction().dot(ray.direction());
    let b = 2.0 * oc.dot(ray.direction());
    let c = radius.mul_add(-radius, oc.dot(oc)); //oc.dot(oc) - radius*radius
    
    let discriminant = b*b - 4.0*a*c;
    discriminant > 0.0
}