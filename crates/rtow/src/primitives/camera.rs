use crate::primitives::{Decimal, Point3, Ray, Vec3};
use rand::rngs::ThreadRng;

#[derive(Copy, Clone, Debug)]
pub struct Camera {
    origin: Point3,
    lower_left_corner: Point3,
    horizontal: Vec3,
    vertical: Vec3,
    u: Vec3,
    v: Vec3,
    w: Vec3,
    lens_radius: Decimal,
}

impl Camera {
    pub const FOCAL_LENGTH: Decimal = 1.0;

    #[must_use]
    pub fn new(
        vertical_fov_degrees: Decimal,
        aspect_ratio: Decimal,
        look_from: Vec3,
        look_at: Vec3,
        aperture: Decimal,
        focus_distance: Decimal,
    ) -> Self {
        let height = (vertical_fov_degrees.to_radians() / 2.0).tan();
        let viewport_height = 2.0 * height;
        let viewport_width = aspect_ratio * viewport_height;

        let w = (look_from - look_at).unit();
        let u = Vec3::UP.cross(w).unit();
        let v = w.cross(u);

        let origin = look_from;
        let horizontal = viewport_width * u;
        let vertical = viewport_width * v;

        Self {
            origin,
            lower_left_corner: origin - horizontal / 2.0 - vertical / 2.0 - w,
            horizontal,
            vertical,
            lens_radius: aperture / 2.0,
            u,
            v,
            w,
        }
    }

    #[must_use]
    pub fn get_ray(&self, s: Decimal, t: Decimal, rng: &mut ThreadRng) -> Ray {
        let radius = self.lens_radius * Vec3::random_in_unit_sphere(rng);
        let offset = self.u * radius.x() + self.v * radius.y();

        Ray::new(
            self.origin,
            self.lower_left_corner + s * self.horizontal + t * self.vertical - self.origin,
        )
    }
}
