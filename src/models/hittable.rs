use crate::math_util::*;
use std::rc::Rc;

#[derive(Debug,Default, Copy, Clone)]
pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub t: f64,
    front_face:bool
}

impl HitRecord{
    pub fn set_face_normal(r:&Ray,outward_normal:Vec3)->(bool,Vec3){
        let front_face=dot(r.get_direction(),&outward_normal)<0.0;
        (front_face,if front_face {outward_normal} else {-outward_normal})
    }
    pub fn from(p:Point3,t:f64,r:&Ray,outward_normal:Vec3)->HitRecord{
        let (front_face,normal)=HitRecord::set_face_normal(r,outward_normal);
        HitRecord{
            p,
            normal,
            t,
            front_face
        }
    }
}

pub trait Hittable {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}

#[derive(Default)]
pub struct HittableList{
    pub objects:Vec<Rc<dyn Hittable>>
}

impl HittableList{
    pub fn add(&mut self,object:Rc<dyn Hittable>){
        self.objects.push(object);
    }
    pub fn from(object:Rc<dyn Hittable>)->HittableList{
        let mut list=HittableList::default();
        list.add(object);
        list
    }
    pub fn clear(&mut self){
        self.objects.clear();
    }
}

impl Hittable for HittableList{
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut rec=HitRecord::default();
        let mut hit_anything=false;
        let mut closest_so_far=t_max;
        for object in self.objects.iter(){
            if let Some(temp_rec)=object.hit(r, t_min, closest_so_far){
                hit_anything=true;
                closest_so_far=temp_rec.t;
                rec=temp_rec;
            }
        }
        if hit_anything {Some(rec)} else {None}
    }
}

