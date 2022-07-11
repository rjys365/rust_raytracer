use image::{RgbImage};
use indicatif::ProgressBar;
use crate::math_util::{Vec3,Point3,Color,Ray,dot,cross};

fn hit_sphere(center:&Point3,radius:f64,r:&Ray)->f64{
    let oc=*r.get_origin()-*center;
    let a=dot(&r.get_direction(),&r.get_direction());
    let b=2.0*dot(&oc,&r.get_direction());
    let c=dot(&oc,&oc)-radius*radius;
    let discriminant=b*b-4.0*a*c;
    if discriminant<0.0 {-1.0} else {(-b-discriminant.sqrt())/(2.0*a)}
}

fn ray_color(r:&Ray)->Color{
    let t=hit_sphere(&Vec3::from(0.0, 0.0, -1.0), 0.5, r);
    if t>0.0{
        let n=(r.at(t)-Vec3::from(0.0,0.0,-1.0)).unit_vector();
        return Vec3::from(n.get_x()+1.0,n.get_y()+1.0,n.get_z()+1.0);
    }
    let unit_direction=r.get_direction().unit_vector();
    let t=0.5*(unit_direction.get_y()+1.0);
    Vec3::from(1.0, 1.0, 1.0)*(1.0-t)+Vec3::from(0.5, 0.7, 1.0)
}

//renders the gradient ackground and a sphere
pub fn render(image_height:u32,image_width:u32,img:&mut RgbImage,progress:&ProgressBar){
    let aspect_ratio:f64=image_width as f64/image_height as f64;

    //Camera
    let viewport_height:f64=2.0;
    let viewport_width=aspect_ratio*viewport_height;
    let focal_length:f64=1.0;

    //Render
    let origin:Point3=Vec3::new();
    let horizontal:Vec3=Vec3{x:viewport_width,y:0.0,z:0.0};
    let vertical:Vec3=Vec3{x:0.0,y:viewport_height,z:0.0};
    let lower_left_corner=origin-horizontal/2.0-vertical/2.0-Vec3{x:0.0,y:0.0,z:focal_length};

    for j in (0..image_height-1).rev(){
        for i in 0..image_width-1{
            let u=i as f64/(image_width-1)as f64;
            let v=j as f64/(image_height-1) as f64;
            let r=Ray::from(&origin, &(lower_left_corner+horizontal*u+vertical*v-origin));
            let pixel_color=ray_color(&r);
            let pixel = img.get_pixel_mut(i,image_height-1-j);
            *pixel=image::Rgb(pixel_color.to_uvec());
            progress.inc(1);
        }
    }
}
