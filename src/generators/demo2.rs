use image::{RgbImage};
use indicatif::ProgressBar;
use crate::math_util::{Vec3,Point3,Color,Ray,dot,cross};
use crate::models::hittable::{Hittable,HittableList};
use crate::models::sphere::Sphere;
use std::f64::consts::PI;
use std::f64::INFINITY;
use std::rc::Rc;

fn ray_color(r:&Ray,world:&dyn Hittable)->Color{
    if let Some(rec)=world.hit(r, 0.0, INFINITY){
        return 0.5*(rec.normal+Color::from(1.0, 1.0, 1.0));
    }
    let unit_direction=r.get_direction().unit_vector();
    let t=0.5*(unit_direction.get_y()+1.0);
    Vec3::from(1.0, 1.0, 1.0)*(1.0-t)+Vec3::from(0.5, 0.7, 1.0)
}

//renders the gradient ackground and a sphere
pub fn render(image_height:u32,image_width:u32,img:&mut RgbImage,progress:&ProgressBar){
    let aspect_ratio:f64=image_width as f64/image_height as f64;

    //World
    let mut world=HittableList::default();
    world.add(Rc::new(Sphere::from(Point3::from(0.0, 0.0, -1.0), 0.5)));
    world.add(Rc::new(Sphere::from(Point3::from(0.0,-100.5,-1.0),100.0)));

    //Camera
    let viewport_height:f64=2.0;
    let viewport_width=aspect_ratio*viewport_height;
    let focal_length:f64=1.0;

    //Render
    let origin:Point3=Point3::default();
    let horizontal:Vec3=Vec3{x:viewport_width,y:0.0,z:0.0};
    let vertical:Vec3=Vec3{x:0.0,y:viewport_height,z:0.0};
    let lower_left_corner=origin-horizontal/2.0-vertical/2.0-Vec3{x:0.0,y:0.0,z:focal_length};

    for j in (0..image_height-1).rev(){
        for i in 0..image_width-1{
            let u=i as f64/(image_width-1)as f64;
            let v=j as f64/(image_height-1) as f64;
            let r=Ray::from(&origin, &(lower_left_corner+horizontal*u+vertical*v-origin));
            let pixel_color=ray_color(&r,&world);
            let pixel = img.get_pixel_mut(i,image_height-1-j);
            *pixel=image::Rgb(pixel_color.to_uvec());
            progress.inc(1);
        }
    }
}
