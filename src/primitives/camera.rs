use crate::primitives::{Decimal, Point3, Ray, Vec3};

#[derive(Copy, Clone, Debug)]
pub struct Camera {
    origin: Point3,
    lower_left_corner: Point3,
    horizontal: Vec3,
    vertical: Vec3,
}

impl Default for Camera {
    fn default() -> Self {
        let origin = Point3::default();
        let horizontal = Vec3::new(Self::VIEWPORT_WIDTH, 0.0, 0.0);
        let vertical = Vec3::new(0.0, Self::VIEWPORT_HEIGHT, 0.0);

        Self {
            origin,
            lower_left_corner: origin
                - horizontal / 2.0
                - vertical / 2.0
                - Point3::new(0.0, 0.0, Self::FOCAL_LENGTH),
            horizontal,
            vertical,
        }
    }
}

impl Camera {
    pub const ASPECT_RATIO: Decimal = 16.0 / 9.0;
    pub const VIEWPORT_HEIGHT: Decimal = 2.0;
    pub const VIEWPORT_WIDTH: Decimal = Self::ASPECT_RATIO * Self::VIEWPORT_HEIGHT;
    pub const FOCAL_LENGTH: Decimal = 1.0;

    #[must_use]
    pub fn get_ray(&self, u: Decimal, v: Decimal) -> Ray {
        Ray::new(
            self.origin,
            self.lower_left_corner + u * self.horizontal + v * self.vertical - self.origin,
        )
    }
}
