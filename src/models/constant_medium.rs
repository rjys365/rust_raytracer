use std::rc::Rc;

use crate::{material::Material, texture::{Texture, solid_color::SolidColor}, math_util::{Color, Vec3, Ray, rand_double}, models::hittable::HitRecord};

use super::hittable::Hittable;

pub struct ConstantMedium{
    pub boundary:Rc<dyn Hittable>,
    pub phase_function:Rc<dyn Material>,
    pub neg_inv_density:f64,
}

impl ConstantMedium{
    pub fn new_texture(boundary:Rc<dyn Hittable>,density:f64,texture:Rc<dyn Texture>)->Self{
        Self{
            boundary,
            neg_inv_density:-1.0/density,
            phase_function: Rc::new(Isotropic::new_texture(texture))
        }
    }
    pub fn new_solid_color(boundary:Rc<dyn Hittable>,density:f64,c:Color)->Self{
        Self { boundary, phase_function:Rc::new(Isotropic::new_solid_color(c)), neg_inv_density: -1.0/density }
    }
}

impl Hittable for ConstantMedium{
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<super::hittable::HitRecord> {
        const ENABLE_DEBUG:bool=false;
        let debugging=ENABLE_DEBUG&&rand_double()<0.00001;

        let mut rec1:HitRecord;
        let mut rec2:HitRecord;

        if let Some(r1)=self.boundary.hit(r,f64::NEG_INFINITY,f64::INFINITY){
            rec1=r1;
        }
        else{
            return None;
        }

        if let Some(r2)=self.boundary.hit(r,rec1.t+0.0001,f64::INFINITY){
            rec2=r2;
        }
        else{
            return None;
        }

        if debugging{println!("t_min={},t_max={}",rec1.t,rec2.t)}

        rec1.t=f64::max(t_min,rec1.t);
        rec2.t=f64::min(t_max,rec2.t);

        if(rec1.t>=rec2.t){return None;}

        rec1.t=f64::max(0.0,rec1.t);

        let rec1=rec1;
        let rec2=rec2;

        let ray_length=r.direction().length();
        let distance_inside_boundary=(rec2.t-rec1.t)*ray_length;
        let hit_distance=self.neg_inv_density*f64::ln(rand_double());

        if(hit_distance>distance_inside_boundary){return None;}

        let rect=rec1.t+hit_distance/ray_length;
        let recp=r.at(rect);

        if debugging {println!("hit_distance={}\nrec.t={}",hit_distance,rect);}

        let recnormal=Vec3::new(1.0,0.0,0.0);
        Some(HitRecord::new(recp,rect,0.0,0.0,r,recnormal,self.phase_function.clone()))
    }
    fn bounding_box(&self, time0: f64, time1: f64) -> Option<super::aabb::Aabb> {
        self.boundary.bounding_box(time0, time1)
    }
}

pub struct Isotropic{
    albedo:Rc<dyn Texture>
}

impl Isotropic{
    pub fn new_solid_color(c:Color)->Self{
        Self { albedo: Rc::new(SolidColor::new(c)) }
    }
    pub fn new_texture(t:Rc<dyn Texture>)->Self{
        Self {albedo:t}
    }
}

impl Material for Isotropic{
    fn scatter(&self, r_in: &crate::math_util::Ray, rec: &super::hittable::HitRecord) -> Option<(Color, crate::math_util::Ray)> {
        let scattered=Ray::new_time(rec.p,Vec3::random_in_unit_sphere(),r_in.time());
        let attenuation=self.albedo.value(rec.u, rec.v, rec.p);
        Some((attenuation,scattered))
    }
}