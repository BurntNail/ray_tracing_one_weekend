use crate::primitives::{materials::Material, Decimal, Point3, Ray, Vec3};

#[derive(Debug, Copy, Clone)]
pub struct HitRecord {
    pub point: Point3,
    pub normal: Vec3,
    pub material: Material,
    pub time: Decimal,
    pub front_face: bool,
}

impl HitRecord {
    pub fn new(
        time: Decimal,
        ray: Ray,
        calc_normal: impl Fn(Vec3) -> Vec3,
        material: Material,
    ) -> Self {
        let point = ray.at(time);
        let normal = calc_normal(point);

        let mut s = Self {
            point,
            time,
            material,
            normal,
            front_face: false,
        };
        s.set_front_face(ray, normal);
        s
    }

    pub fn set_front_face(&mut self, ray: Ray, outward_normal: Vec3) {
        self.front_face = ray.direction().dot(outward_normal) < 0.0;
        self.normal = if self.front_face {
            outward_normal
        } else {
            -outward_normal
        };
    }
}
