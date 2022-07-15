use std::rc::Rc;

use crate::{
    material::Material,
    math_util::{Point3, Vec3},
};

use super::{
    aabb::Aabb,
    hittable::{HitRecord, Hittable},
};

pub struct YzRect {
    pub mat: Rc<dyn Material>,
    pub y0: f64,
    pub y1: f64,
    pub z0: f64,
    pub z1: f64,
    pub k: f64,
}

impl YzRect {
    pub fn new(y0: f64, y1: f64, z0: f64, z1: f64, k: f64, mat: Rc<dyn Material>) -> Self {
        Self {
            y0,
            y1,
            z0,
            z1,
            k,
            mat,
        }
    }
}

impl Hittable for YzRect {
    fn hit(
        &self,
        r: &crate::math_util::Ray,
        t_min: f64,
        t_max: f64,
    ) -> Option<super::hittable::HitRecord> {
        let t = (self.k - r.origin().x()) / r.direction().x();
        if t < t_min || t > t_max {
            return None;
        }
        let y = r.origin().y() + t * r.direction().y();
        let z = r.origin().z() + t * r.direction().z();
        if y < self.y0 || y > self.y1 || z < self.z0 || z > self.z1 {
            return None;
        }
        let u = (y - self.y0) / (self.y1 - self.y0);
        let v = (z - self.z0) / (self.z1 - self.z0);
        let outward_normal = Vec3::new(1.0, 0.0, 0.0);
        let p = r.at(t);
        let rec = HitRecord::new(p, t, u, v, r, outward_normal, self.mat.clone());
        Some(rec)
    }
    fn bounding_box(&self, _time0: f64, _time1: f64) -> Option<super::aabb::Aabb> {
        Some(Aabb::new(
            Point3::new(self.k - 0.001, self.y0, self.z0),
            Point3::new(self.k + 0.001, self.y1, self.z1),
        ))
    }
}
