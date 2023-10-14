use crate::primitives::{collisions::HitRecord, Colour, Decimal, Point3, Ray};
use rand::{rngs::ThreadRng, Rng};
use std::fmt::Debug;

#[derive(Debug, Copy, Clone)]
pub enum Material {
    ///albedo
    LambertianDiffuse(Colour),
    ///albedo, fuzz
    MetalReflection(Colour, Decimal),
    ///index of refraction
    DielectricRefraction(Decimal),
}

impl Material {
    #[must_use]
    pub fn scatter(
        &self,
        ray_in: Ray,
        hit_record: HitRecord,
        rng: &mut ThreadRng,
    ) -> Option<(Colour, Ray)> {
        match self {
            Self::LambertianDiffuse(albedo) => {
                let mut scatter_dir = hit_record.normal + Point3::random_unit_vector(rng);

                if scatter_dir.near_zero() {
                    scatter_dir = hit_record.normal;
                }

                Some((*albedo, Ray::new(hit_record.point, scatter_dir)))
            }
            Self::MetalReflection(albedo, fuzz) => {
                let reflected = ray_in.direction().unit().reflect(hit_record.normal);
                let scattered = Ray::new(
                    hit_record.point,
                    reflected + *fuzz * Point3::random_in_unit_sphere(rng),
                );
                if scattered.direction().dot(hit_record.normal) > 0.0 {
                    Some((*albedo, scattered))
                } else {
                    None
                }
            }
            Self::DielectricRefraction(ior) => {
                fn reflectance(cos: Decimal, reference_index: Decimal) -> Decimal {
                    //shlick's approximation
                    let r_0 = ((1.0 - reference_index) / (1.0 + reference_index)).powi(2);
                    r_0 + (1.0 - r_0) * (1.0 - cos).powi(5)
                }

                let refraction_ratio = if hit_record.front_face {
                    1.0 / ior
                } else {
                    *ior
                };

                let unit_direction = ray_in.direction().unit();
                let cos_theta = (-unit_direction).dot(hit_record.normal).min(1.0);
                let sin_theta = (1.0 - cos_theta * cos_theta).sqrt(); //identities

                let direction = if refraction_ratio * sin_theta > 1.0
                    || reflectance(cos_theta, refraction_ratio) > rng.gen_range(0.0..=1.0)
                {
                    //can't refract
                    unit_direction.reflect(hit_record.normal)
                } else {
                    //can refract
                    unit_direction.refract(hit_record.normal, refraction_ratio)
                };

                Some((
                    Colour::new(1.0, 1.0, 1.0),
                    Ray::new(hit_record.point, direction),
                ))
            }
        }
    }
}
