use std::rc::Rc;

use crate::{material::Material, math_util::{Vec3, Point3}};

use super::{hittable::{Hittable, HitRecord}, aabb::Aabb};

pub struct XyRect{
    pub mat:Rc<dyn Material>,
    pub x0:f64,
    pub x1:f64,
    pub y0:f64,
    pub y1:f64,
    pub k:f64
}

impl XyRect{
    pub fn new(x0:f64,x1:f64,y0:f64,y1:f64,k:f64,mat:Rc<dyn Material>,)->Self{
        Self{
            x0,x1,y0,y1,k,mat
        }
    }
}

impl Hittable for XyRect{
    fn hit(&self, r: &crate::math_util::Ray, t_min: f64, t_max: f64) -> Option<super::hittable::HitRecord> {
        let t=(self.k-r.origin().z())/r.direction().z();
        if t<t_min||t>t_max {return None;}
        let x=r.origin().x()+t*r.direction().x();
        let y=r.origin().y()+t*r.direction().y();
        if x<self.x0||x>self.x1||y<self.y0||y>self.y1 {return None}
        let u=(x-self.x0)/(self.x1-self.x0);
        let v=(y-self.y0)/(self.y1-self.y0);
        let outward_normal=Vec3::new(0.0,0.0,1.0);
        let p=r.at(t);
        let rec=HitRecord::new(p,t,u,v,r,outward_normal,self.mat.clone());
        Some(rec)
    }
    fn bounding_box(&self, _time0: f64, _time1: f64) -> Option<super::aabb::Aabb> {
        Some(Aabb::new(Point3::new(self.x0,self.y0,self.k-0.001),Point3::new(self.x1,self.y1,self.k+0.001)))
    }
}