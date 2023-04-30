pub mod sphere;

mod hit_record;

use crate::primitives::{Decimal, Ray};
pub use hit_record::HitRecord;
use std::fmt::Debug;

mod hittable_list;
pub use hittable_list::HittableList;

pub trait Hittable: Debug + Send + Sync {
    fn hit(&self, ray: Ray, t_min: Decimal, t_max: Decimal) -> Option<HitRecord>;
}
