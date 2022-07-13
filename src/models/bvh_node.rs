use std::{cmp::Ordering, rc::Rc};

use crate::math_util::{rand_int, Ray};

use super::{
    aabb::Aabb,
    hittable::{HitRecord, Hittable},
};

pub struct BvhNode {
    pub left: Option<Rc<dyn Hittable>>,
    pub right: Option<Rc<dyn Hittable>>,
    pub bx: Aabb, //box
}

impl BvhNode {
    pub fn new(src_objects: &[Rc<dyn Hittable>], time0: f64, time1: f64) -> BvhNode {
        let mut objects = Vec::from(src_objects);
        let axis = rand_int(0, 2) as usize;
        let comp = |a: &Rc<dyn Hittable>, b: &Rc<dyn Hittable>| {
            let box_a = a.bounding_box(0.0, 0.0).unwrap();
            let box_b = b.bounding_box(0.0, 0.0).unwrap(); //may panic?
            box_a.min()[axis].total_cmp(&box_b.min()[axis])
        };
        let object_span = src_objects.len();
        let left: Option<Rc<dyn Hittable>>;
        let right: Option<Rc<dyn Hittable>>;
        if object_span == 1 {
            left = Some(objects[0].clone());
            right = None;
        } else if object_span == 2 {
            if comp(&objects[0], &objects[1]) == Ordering::Less {
                left = Some(objects[0].clone());
                right = Some(objects[1].clone());
            } else {
                left = Some(objects[1].clone());
                right = Some(objects[0].clone());
            }
        } else {
            objects.sort_by(comp);
            let llen = object_span / 2;
            //let rlen = object_span - llen;
            let (lvec, rvec) = objects.split_at(llen);
            left = Some(Rc::new(BvhNode::new(lvec, time0, time1)));
            right = Some(Rc::new(BvhNode::new(rvec, time0, time1)));
        }
        let bx: Aabb;

        if let Some(lpt) = &left {
            let lbx_r = lpt.bounding_box(time0, time1);
            if let Some(lbx) = lbx_r {
                if let Some(rpt) = &right {
                    if let Some(rbx) = rpt.bounding_box(time0, time1) {
                        bx = Aabb::surrounding_box(&lbx, &rbx);
                    } else {
                        bx = lbx
                    }
                } else {
                    bx = lbx
                }
            } else {
                bx = right.as_ref().unwrap().bounding_box(time0, time1).unwrap();
            }
        } else {
            bx = right.as_ref().unwrap().bounding_box(time0, time1).unwrap(); //MAY PANIC?
        }
        Self { left, right, bx }
    }
}

impl Hittable for BvhNode {
    fn bounding_box(&self, _time0: f64, _time1: f64) -> Option<Aabb> {
        Some(self.bx)
    }
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut ret:Option<HitRecord>=None;
        let mut ok:bool=false;
        if let Some(lchild)=&self.left{
            if let Some(lrec)=lchild.hit(r, t_min, t_max){
                ret=Some(lrec);
                ok=true;
            }
        }
        if let Some(rchild)=&self.right{
            if let(Some(rrec))=rchild.hit(r, t_min, if ok {ret.as_ref().unwrap().t} else{t_max}){
                ret=Some(rrec);
                ok=true;
            }
        }
        if ok {ret} else{None}
    }
}
