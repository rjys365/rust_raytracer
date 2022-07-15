use std::{f64::consts::PI, rc::Rc};

use crate::material::Material;
use crate::math_util::*;

use super::{
    aabb::Aabb,
    hittable::{HitRecord, Hittable},
};

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
    fn get_sphere_uv(p: Point3) -> (f64, f64) {
        // p: a given point on the sphere of radius one, centered at the origin.
        // u: returned value [0,1] of angle around the Y axis from X=-1.
        // v: returned value [0,1] of angle from Y=-1 to Y=+1.
        //     <1 0 0> yields <0.50 0.50>       <-1  0  0> yields <0.00 0.50>
        //     <0 1 0> yields <0.50 1.00>       < 0 -1  0> yields <0.50 0.00>
        //     <0 0 1> yields <0.25 0.50>       < 0  0 -1> yields <0.75 0.50>
        let theta = f64::acos(-p.y());
        let phi = f64::atan2(-p.z(), -p.x()) + PI;
        (phi / (2.0 * PI), theta / PI)
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = *r.origin() - self.center;
        let a = r.direction().length_squared();
        let half_b = dot(&oc, r.direction());
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

        if root < t_min || root > t_max {
            return None;
        }

        let outward_normal = (r.at(root) - self.center) / self.radius;

        let (u, v) = Self::get_sphere_uv(outward_normal);
        let rec = HitRecord::new(
            r.at(root),
            root,
            u,
            v,
            r,
            outward_normal,
            self.mat_ptr.clone(),
        );
        Some(rec)
    }
    fn bounding_box(&self, _time0: f64, _time1: f64) -> Option<Aabb> {
        Some(Aabb::new(
            self.center - Vec3::new(self.radius, self.radius, self.radius),
            self.center + Vec3::new(self.radius, self.radius, self.radius),
        ))
    }
}
