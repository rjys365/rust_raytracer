use super::{
    aabb::Aabb,
    hittable::{HitRecord, Hittable},
};
use crate::{
    material::Material,
    math_util::{dot, Point3, Ray, Vec3},
};
use std::{f64::consts::PI, rc::Rc};

#[derive(Clone)]
pub struct MovingSphere {
    pub center0: Point3,
    pub center1: Point3,
    pub time0: f64,
    pub time1: f64,
    pub radius: f64,
    pub mat_ptr: Rc<dyn Material>,
}

impl MovingSphere {
    pub fn center(&self, time: f64) -> Point3 {
        self.center0
            + ((time - self.time0) / (self.time1 - self.time0)) * (self.center1 - self.center0)
    }
    pub fn new(
        center0: Point3,
        center1: Point3,
        time0: f64,
        time1: f64,
        radius: f64,
        mat_ptr: Rc<dyn Material>,
    ) -> Self {
        Self {
            center0,
            center1,
            time0,
            time1,
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

impl Hittable for MovingSphere {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = *r.origin() - self.center(r.time());
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

        let outward_normal = (r.at(root) - self.center(r.time())) / self.radius;
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
    fn bounding_box(&self, t0: f64, t1: f64) -> Option<Aabb> {
        let t0 = f64::max(self.time0, t0);
        let t1 = f64::min(self.time1, t1);
        let box0 = Aabb::new(
            self.center(t0) - Vec3::new(self.radius, self.radius, self.radius),
            self.center(t0) + Vec3::new(self.radius, self.radius, self.radius),
        );
        let box1 = Aabb::new(
            self.center(t1) - Vec3::new(self.radius, self.radius, self.radius),
            self.center(t1) + Vec3::new(self.radius, self.radius, self.radius),
        );
        Some(Aabb::surrounding_box(&box0, &box1))
    }
}
