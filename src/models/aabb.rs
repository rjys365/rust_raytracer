use crate::math_util::{Point3, Ray};

#[derive(Clone, Copy, Default,Debug)]
pub struct Aabb {
    pub minimum: Point3,
    pub maximum: Point3,
}

impl Aabb {
    pub fn new(minimum: Point3, maximum: Point3) -> Self {
        Aabb { minimum, maximum }
    }
    pub fn min(&self) -> &Point3 {
        &self.minimum
    }
    pub fn max(&self) -> &Point3 {
        &self.maximum
    }
    pub fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> bool {
        for a in 0..=2 {
            let inv_d = 1.0 / r.direction()[a];
            let t0 = (if inv_d < 0.0 {
                self.max()[a]
            } else {
                self.min()[a]
            } - r.origin()[a])
                * inv_d;
            let t1 = (if inv_d < 0.0 {
                self.min()[a]
            } else {
                self.max()[a]
            } - r.origin()[a])
                * inv_d;
            let t_min = f64::max(t_min, t0);
            let t_max = f64::min(t_max, t1);
            if t_max <= t_min {
                return false;
            }
        }
        true
    }
    pub fn surrounding_box(&self, box1: &Aabb) -> Self {
        let small = Point3::new(
            f64::min(self.min().x, box1.min().x),
            f64::min(self.min().y, box1.min().y),
            f64::min(self.min().z, box1.min().z),
        );
        let big = Point3::new(
            f64::max(self.max().x, box1.max().x),
            f64::max(self.max().y, box1.max().y),
            f64::max(self.max().z, box1.max().z),
        );
        Aabb::new(small, big)
    }
}
