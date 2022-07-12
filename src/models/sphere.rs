use std::rc::Rc;

use crate::material::Material;
use crate::math_util::*;

use super::hittable::{HitRecord, Hittable};

#[derive(Clone)]
pub struct Sphere {
    pub center: Point3,
    pub radius: f64,
    pub mat_ptr: Rc<dyn Material>,
}

impl Sphere {
    pub fn new(center: Point3, radius: f64, mat_ptr: Rc<dyn Material>) -> Self {
        Self {
            center,
            radius,
            mat_ptr,
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = *r.get_origin() - self.center;
        let a = r.get_direction().length_squared();
        let half_b = dot(&oc, r.get_direction());
        let c = oc.length_squared() - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return None;
        }
        let sqrtd = discriminant.sqrt();

        let root1 = (-half_b - sqrtd) / a;
        let root2 = (-half_b + sqrtd) / a;

        let root = if root1 < t_min || root1 > t_max {
            root2
        } else {
            root1
        };

        if root1 < t_min || root1 > t_max {
            return None;
        }

        let outward_normal = (r.at(root) - self.center) / self.radius;
        let rec = HitRecord::new(r.at(root), root, r, outward_normal, self.mat_ptr.clone());
        Some(rec)
    }
}
