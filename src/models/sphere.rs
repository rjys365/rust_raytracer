use crate::math_util::*;

use super::hittable::{Hittable, HitRecord};

#[derive(Debug,Copy,Clone)]
pub struct Sphere{
    pub center:Point3,
    pub radius:f64,
}

impl Sphere{
    pub fn from(center:Point3,radius:f64)->Self{
        Self{center,radius}
    }
}

impl Hittable for Sphere{
    fn hit(&self,r:&Ray,t_min:f64,t_max:f64)->Option<HitRecord> {
        let oc=*r.get_origin()-self.center;
        let a=r.get_direction().length_squared();
        let half_b=dot(&oc,r.get_direction());
        let c=oc.length_squared()-self.radius*self.radius;

        let discriminant=half_b*half_b-a*c;
        if discriminant<0.0 {return None;}
        let sqrtd=discriminant.sqrt();

        let root1=(-half_b-sqrtd)/a;
        let root2=(-half_b+sqrtd)/a;
        
        let root=if root1<t_min||root1>t_max {root2} else {root1};

        if root1<t_min||root1>t_max {return None;}

        let outward_normal=(r.at(root)-self.center)/self.radius;
        let rec=HitRecord::from(r.at(root),root,r,outward_normal);
        Some(rec)
    }
}
