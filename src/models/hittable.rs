use crate::{material::Material, math_util::*};
use std::rc::Rc;

#[derive(Clone)]
pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,
    pub mat_ptr: Rc<dyn Material>,
}

impl HitRecord {
    pub fn set_face_normal(r: &Ray, outward_normal: Vec3) -> (bool, Vec3) {
        let front_face = dot(r.get_direction(), &outward_normal) < 0.0;
        (
            front_face,
            if front_face {
                outward_normal
            } else {
                -outward_normal
            },
        )
    }
    pub fn new(
        p: Point3,
        t: f64,
        r: &Ray,
        outward_normal: Vec3,
        mat_ptr: Rc<dyn Material>,
    ) -> HitRecord {
        let (front_face, normal) = HitRecord::set_face_normal(r, outward_normal);
        HitRecord {
            p,
            normal,
            t,
            front_face,
            mat_ptr,
        }
    }
}

pub trait Hittable {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}

#[derive(Default)]
pub struct HittableList {
    pub objects: Vec<Rc<dyn Hittable>>,
}

impl HittableList {
    pub fn add(&mut self, object: Rc<dyn Hittable>) {
        self.objects.push(object);
    }
    pub fn new(object: Rc<dyn Hittable>) -> HittableList {
        let mut list = HittableList::default();
        list.add(object);
        list
    }
    pub fn clear(&mut self) {
        self.objects.clear();
    }
}

impl Hittable for HittableList {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut rec: Option<HitRecord> = None;
        let mut closest_so_far = t_max;
        for object in self.objects.iter() {
            if let Some(temp_rec) = object.hit(r, t_min, closest_so_far) {
                closest_so_far = temp_rec.t;
                rec = Some(temp_rec);
            }
        }
        rec
    }
}
