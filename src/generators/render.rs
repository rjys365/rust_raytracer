//anti-alias render
use crate::camera::Camera;
// use crate::material::dielectric::Dieletric;
// use crate::material::lambertian::Lambertian;
// use crate::material::metal::Metal;
use crate::math_util::{rand_double, Color, Point3, Ray, Vec3};
use crate::models::bvh_node::BvhNode;
use crate::models::hittable::Hittable;
use crate::models::random_scene::random_scene;
// use crate::models::sphere::Sphere;
use image::RgbImage;
use indicatif::ProgressBar;
//use std::f64::consts::PI;
// use std::rc::Rc;

pub static mut hit_cnt:i64=0;
pub static mut hit_cnt1:i64=0;

fn ray_color(r: &Ray, world: &dyn Hittable, depth: i32) -> Color {
    if depth <= 0 {
        return Color::default();
    }
    if let Some(rec) = world.hit(r, 0.001, f64::INFINITY) {
        unsafe{hit_cnt1+=1;}
        if let Some((attenuation, scattered)) = rec.mat_ptr.scatter(r, &rec) {
            return attenuation * ray_color(&scattered, world, depth - 1);
        } else {
            return Color::default();
        }
    }
    else {unsafe{hit_cnt1+=1;}}
    let unit_direction = r.direction().unit_vector();
    let t = 0.5 * (unit_direction.y() + 1.0);
    Vec3::new(1.0, 1.0, 1.0) * (1.0 - t) + Vec3::new(0.5, 0.7, 1.0)
}

pub fn render(image_height: u32, image_width: u32, img: &mut RgbImage, progress: &ProgressBar) {
    const SAMPLES_PER_PIXEL: u32 = 100;
    const MAX_DEPTH: i32 = 50;
    let aspect_ratio: f64 = image_width as f64 / image_height as f64;

    //World
    let world = random_scene();
    let world_b=BvhNode::new(&world.objects,0.0,1.0,0);
    world_b.traverse();
    //Camera

    let lookfrom = Point3::new(13.0, 2.0, 3.0);
    let lookat = Point3::default();
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let aperture = 0.1;
    let dist_to_focus = 10.0;
    let cam = Camera::new(
        lookfrom,
        lookat,
        vup,
        20.0,
        aspect_ratio,
        aperture,
        dist_to_focus,
        0.0,
        1.0,
    );

    //Render

    // for j in (0..=image_height - 1).rev() {
    //     for i in 0..=image_width - 1 {
    //         let mut pixel_color = Color::default();
    //         for _k in 1..=SAMPLES_PER_PIXEL {
    //             let u = (i as f64 + rand_double()) / (image_width as f64 - 1.0);
    //             let v = (j as f64 + rand_double()) / (image_height as f64 - 1.0);
    //             let r = cam.get_ray(u, v);
    //             //unsafe{hit_cnt=0;}
    //             pixel_color += ray_color(&r, &world, MAX_DEPTH);
    //             //unsafe{hit_cnt1+=1;}
    //             unsafe{
    //                 // if hit_cnt1%10000==0 {println!("{}",hit_cnt as f64 / hit_cnt1 as f64);}
    //                 if hit_cnt as f64 / hit_cnt1 as f64>300.0 {
    //                     println!("AAAAAA");
    //                 }
    //             }
    //         }
    //         pixel_color /= SAMPLES_PER_PIXEL as f64;
    //         let pixel = img.get_pixel_mut(i, image_height - 1 - j);
    //         *pixel = image::Rgb(pixel_color.to_uvec());
    //         progress.inc(1);
    //     }
    // }
    unsafe{println!("{}",hit_cnt as f64 / hit_cnt1 as f64);}
}
