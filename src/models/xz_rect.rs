use std::rc::Rc;

use crate::{
    material::Material,
    math_util::{Point3, Vec3},
};

use super::{
    aabb::Aabb,
    hittable::{HitRecord, Hittable},
};

pub struct XzRect {
    pub mat: Rc<dyn Material>,
    pub x0: f64,
    pub x1: f64,
    pub z0: f64,
    pub z1: f64,
    pub k: f64,
}

impl XzRect {
    pub fn new(x0: f64, x1: f64, z0: f64, z1: f64, k: f64, mat: Rc<dyn Material>) -> Self {
        Self {
            x0,
            x1,
            z0,
            z1,
            k,
            mat,
        }
    }
}

impl Hittable for XzRect {
    fn hit(
        &self,
        r: &crate::math_util::Ray,
        t_min: f64,
        t_max: f64,
    ) -> Option<super::hittable::HitRecord> {
        let t = (self.k - r.origin().y()) / r.direction().y();
        if t < t_min || t > t_max {
            return None;
        }
        let x = r.origin().x() + t * r.direction().x();
        let z = r.origin().z() + t * r.direction().z();
        if x < self.x0 || x > self.x1 || z < self.z0 || z > self.z1 {
            return None;
        }
        let u = (x - self.x0) / (self.x1 - self.x0);
        let v = (z - self.z0) / (self.z1 - self.z0);
        let outward_normal = Vec3::new(0.0, 1.0, 0.0);
        let p = r.at(t);
        let rec = HitRecord::new(p, t, u, v, r, outward_normal, self.mat.clone());
        Some(rec)
    }
    fn bounding_box(&self, _time0: f64, _time1: f64) -> Option<super::aabb::Aabb> {
        Some(Aabb::new(
            Point3::new(self.x0, self.k - 0.001, self.z0),
            Point3::new(self.x1, self.k + 0.001, self.z1),
        ))
    }
}
