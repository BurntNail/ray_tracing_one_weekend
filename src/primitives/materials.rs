use crate::primitives::{collisions::HitRecord, Colour, Decimal, Point3, Ray};
use std::fmt::Debug;

#[derive(Debug, Copy, Clone)]
pub enum Material {
    LambertianDiffuse(Colour),
    Metal(Colour, Decimal),
}

impl Material {
    #[must_use]
    pub fn scatter(&self, ray_in: Ray, hit_record: HitRecord) -> Option<(Colour, Ray)> {
        match self {
            Self::LambertianDiffuse(albedo) => {
                let mut scatter_dir = hit_record.normal + Point3::random_unit_vector();

                if scatter_dir.near_zero() {
                    scatter_dir = hit_record.normal;
                }

                Some((*albedo, Ray::new(hit_record.point, scatter_dir)))
            }
            Self::Metal(albedo, fuzz) => {
                let fuzz = fuzz.max(1.0);

                let reflected = ray_in.direction().unit().reflect(hit_record.normal);
                let scattered = Ray::new(
                    hit_record.point,
                    reflected + fuzz * Point3::random_in_unit_sphere(),
                );
                if scattered.direction().dot(hit_record.normal) > 0.0 {
                    Some((*albedo, scattered))
                } else {
                    None
                }
            }
        }
    }
}
