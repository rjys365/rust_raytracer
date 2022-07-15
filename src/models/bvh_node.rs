use std::{cmp::Ordering, rc::Rc};

use crate::{math_util::{rand_int, Ray}, generators::{render, self}};

use super::{
    aabb::Aabb,
    hittable::{HitRecord, Hittable},
};

pub struct BvhNode {
    pub left: Option<Rc<dyn Hittable>>,
    pub right: Option<Rc<dyn Hittable>>,
    pub bx: Aabb, //box
    pub siz:usize,
    pub leaf:bool,
    pub cnt:i32,
}

static mut BVH_CNT:i32=0;
impl BvhNode {
    pub fn new(src_objects: &[Rc<dyn Hittable>], time0: f64, time1: f64,axis:usize) -> BvhNode {
        let cnt;
        unsafe{
            cnt=BVH_CNT;
            BVH_CNT+=1;
        }
        let mut objects = Vec::from(src_objects);
        //let axis = rand_int(0, 2) as usize;
        let comp = |a: &Rc<dyn Hittable>, b: &Rc<dyn Hittable>| {
            let box_a = a.bounding_box(time0, time1).unwrap();
            let box_b = b.bounding_box(time0, time1).unwrap(); //may panic?
            box_a.min()[axis].partial_cmp(&box_b.min()[axis]).unwrap()
        };
        let object_span = src_objects.len();
        let left: Option<Rc<dyn Hittable>>;
        let right: Option<Rc<dyn Hittable>>;
        let leaf:bool;
        if object_span == 1 {
            left = Some(objects[0].clone());
            right = Some(objects[0].clone());
            leaf=true;
        } else if object_span == 2 {
            if comp(&objects[0], &objects[1]) == Ordering::Less {
                left = Some(objects[0].clone());
                right = Some(objects[1].clone());
            } else {
                left = Some(objects[1].clone());
                right = Some(objects[0].clone());
            }
            leaf=true;
        } else {
            objects.sort_by(comp);
            // for ob in objects.iter(){
            //     if let Some(bdb)=ob.as_ref().bounding_box(0.0, 0.0){
            //         dbg!(bdb.min());
            //     }
            // }
            let llen = object_span / 2;
            //let rlen = object_span - llen;
            let (lvec, rvec) = objects.split_at(llen);
            left = Some(Rc::new(BvhNode::new(lvec, time0, time1,(axis+1)%3)));
            right = Some(Rc::new(BvhNode::new(rvec, time0, time1,(axis+1)%3)));
            leaf=false;
        }
        
        let bx=Aabb::surrounding_box(&left.as_ref().unwrap().bounding_box(time0, time1).unwrap(), &right.as_ref().unwrap().bounding_box(time0, time1).unwrap());
        Self { left, right, bx ,siz:object_span as usize,leaf,cnt}
    }
}

impl Hittable for BvhNode {
    fn bounding_box(&self, _time0: f64, _time1: f64) -> Option<Aabb> {
        Some(self.bx)
    }
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        // unsafe{generators::render::hit_cnt+=1;}
        // unsafe{
        //     if generators::render::hit_cnt as f64 / generators::render::hit_cnt1 as f64>300.0 {
        //         dbg!(&self.bx);
        //     }
        // }
        if !self.bx.hit(r, t_min, t_max) {
            return None;
        }
        let l_result = if let Some(lchild) = &self.left {
            lchild.hit(r, t_min, t_max)
        } else {
            None
        };
        let t_max=if let Some(lrec)=&l_result {lrec.t} else{t_max};
        let r_result = if let Some(rchild) = &self.right {
            rchild.hit(r, t_min, t_max)
        } else {
            None
        };
        if r_result.is_none() {l_result} else{r_result}
    }
    fn print_no(&self) {
        print!("{}",self.cnt);
    }
    fn traverse(&self){
        let mut que:std::collections::VecDeque<Rc<dyn Hittable>>=std::collections::VecDeque::default();
        if self.leaf {return;}
        println!("current node:{}",self.cnt);
        println!("size:{}",self.siz);
        dbg!(self.bounding_box(0.0, 1.0).unwrap());
        print!("left:");
        self.left.as_ref().unwrap().as_ref().print_no();
        println!();
        print!("right:");
        self.right.as_ref().unwrap().as_ref().print_no();
        println!();
        self.left.as_ref().unwrap().as_ref().traverse();
        self.right.as_ref().unwrap().as_ref().traverse();
        while(!que.is_empty()){

        }
    }
}
