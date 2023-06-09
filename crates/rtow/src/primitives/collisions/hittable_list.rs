use crate::primitives::{
    collisions::{HitRecord, Hittable},
    Decimal, Ray,
};
use std::sync::Arc;

#[derive(Default, Debug, Clone)]
pub struct HittableList(Vec<Arc<Box<dyn Hittable>>>);

impl HittableList {
    pub fn add(&mut self, item: Arc<Box<dyn Hittable>>) {
        self.0.push(item);
    }
    pub fn clear(&mut self) {
        self.0.clear();
    }
}

impl Hittable for HittableList {
    fn hit(&self, ray: Ray, t_min: Decimal, t_max: Decimal) -> Option<HitRecord> {
        let mut smallest = None;

        for hit in self.0.iter().filter_map(|obj| obj.hit(ray, t_min, t_max)) {
            if smallest.map_or(true, |smallest: HitRecord| hit.time < smallest.time) {
                smallest = Some(hit);
            }
        }

        smallest
    }
}
