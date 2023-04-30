pub mod camera;
pub mod collisions;
pub mod materials;
mod ray;
mod vec3;

pub use ray::Ray;
pub use vec3::Vec3;

pub type Decimal = f32;

pub type Point3 = Vec3;
pub type Colour = Vec3;
