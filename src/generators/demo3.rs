//anti-alias render
use image::{RgbImage};
use indicatif::ProgressBar;
use rand::Rng;
use crate::camera::Camera;
use crate::material::lambertian::Lambertian;
use crate::material::metal::Metal;
use crate::math_util::{Vec3,Point3,Color,Ray/*,dot,cross*/};
use crate::models::hittable::{Hittable,HittableList};
use crate::models::sphere::Sphere;
use rand::prelude::ThreadRng;
//use std::f64::consts::PI;
use std::f64::INFINITY;
use std::rc::Rc;

fn ray_color(r:&Ray,world:&dyn Hittable,depth:i32,rng: &mut ThreadRng)->Color{
    if depth<=0 {return Color::default();}
    if let Some(rec)=world.hit(r, 0.001, INFINITY){
        if let Some((attenuation,scattered))=rec.mat_ptr.scatter(r, &rec, rng){
            return attenuation*ray_color(&scattered, world, depth-1, rng);
        }
        else {return Color::default();}
    }
    let unit_direction=r.get_direction().unit_vector();
    let t=0.5*(unit_direction.get_y()+1.0);
    Vec3::from(1.0, 1.0, 1.0)*(1.0-t)+Vec3::from(0.5, 0.7, 1.0)
}

pub fn render(image_height:u32,image_width:u32,img:&mut RgbImage,progress:&ProgressBar){
    let mut rng=rand::thread_rng();
    const SAMPLES_PER_PIXEL:u32=100;
    const MAX_DEPTH:i32=50;
    //let aspect_ratio:f64=image_width as f64/image_height as f64;

    //World
    let mut world=HittableList::default();

    let material_ground=Rc::new(Lambertian{albedo:Color::from(0.8,0.8,0.0)});
    let material_center=Rc::new(Lambertian{albedo:Color::from(0.7,0.3,0.3)});
    let material_left=Rc::new(Metal{albedo:Color::from(0.8,0.8,0.8)});
    let material_right=Rc::new(Metal{albedo:Color::from(0.8,0.6,0.2)});

    world.add(Rc::new(Sphere::from(Point3::from(0.0,-100.5,-1.0),100.0,material_ground)));
    world.add(Rc::new(Sphere::from(Point3::from(0.0,0.0,-1.0),0.5,material_center)));
    world.add(Rc::new(Sphere::from(Point3::from(-1.0,0.0,-1.0),0.5,material_left)));
    world.add(Rc::new(Sphere::from(Point3::from(1.0,0.0,-1.0),0.5,material_right)));
    //Camera
    let cam=Camera::default();

    //Render

    for j in (0..image_height-1).rev(){
        for i in 0..image_width-1{
            let mut pixel_color=Color::default();
            for _k in 1..SAMPLES_PER_PIXEL{
                let u=(i as f64 + rng.gen::<f64>())/(image_width as f64 - 1.0);
                let v=(j as f64 + rng.gen::<f64>())/(image_height as f64 -1.0);
                let r=cam.get_ray(u, v);
                pixel_color+=ray_color(&r, &world,MAX_DEPTH,&mut rng);
            }
            pixel_color/=SAMPLES_PER_PIXEL as f64;
            let pixel = img.get_pixel_mut(i,image_height-1-j);
            *pixel=image::Rgb(pixel_color.to_uvec());
            progress.inc(1);
        }
    }
}
