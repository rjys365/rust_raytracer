//anti-alias render
use crate::camera::Camera;
// use crate::material::dielectric::Dieletric;
// use crate::material::lambertian::Lambertian;
// use crate::material::metal::Metal;
use crate::math_util::{rand_double, Color, Point3, Ray, Vec3};
// use crate::models::bvh_node::BvhNode;
use crate::models::hittable::Hittable;
use crate::scene::cornell_box::cornell_box;
// use crate::scene::simple_light::simple_light;
// use crate::scene::random_scene::random_scene;
// use crate::scene::two_perlin_sphere::two_perlin_sphere;
// use crate::scene::earth::earth;
// use crate::scene::two_sphere::two_sphere;
// use crate::models::sphere::Sphere;
use image::RgbImage;
use indicatif::ProgressBar;
//use std::f64::consts::PI;
// use std::rc::Rc;

fn ray_color(r: &Ray, world: &dyn Hittable, depth: i32, background: &Color) -> Color {
    if depth <= 0 {
        return Color::default();
    }
    if let Some(rec) = world.hit(r, 0.001, f64::INFINITY) {
        if let Some((attenuation, scattered)) = rec.mat_ptr.scatter(r, &rec) {
            attenuation * ray_color(&scattered, world, depth - 1, background)
        } else {
            rec.mat_ptr.emitted(rec.u, rec.v, &rec.p)
        }
    } else {
        *background
    }
}

pub fn render(image_height: u32, image_width: u32, img: &mut RgbImage, progress: &ProgressBar) {
    const SAMPLES_PER_PIXEL: u32 = 200;
    const MAX_DEPTH: i32 = 50;
    let aspect_ratio: f64 = image_width as f64 / image_height as f64;

    //World
    let background = Color::new(0.0, 0.0, 0.0);
    let world = cornell_box();
    // let world=BvhNode::new(&world.objects,0.0,1.0);
    //Camera

    let lookfrom = Point3::new(278.0, 278.0, -800.0);
    let lookat = Point3::new(278.0, 278.0, 0.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let aperture = 0.0;
    let dist_to_focus = 10.0;
    let cam = Camera::new(
        lookfrom,
        lookat,
        vup,
        40.0,
        aspect_ratio,
        aperture,
        dist_to_focus,
        0.0,
        1.0,
    );

    //Render

    for j in (0..=image_height - 1).rev() {
        for i in 0..=image_width - 1 {
            let mut pixel_color = Color::default();
            for _k in 1..=SAMPLES_PER_PIXEL {
                let u = (i as f64 + rand_double()) / (image_width as f64 - 1.0);
                let v = (j as f64 + rand_double()) / (image_height as f64 - 1.0);
                let r = cam.get_ray(u, v);
                pixel_color += ray_color(&r, &world, MAX_DEPTH, &background);
            }
            pixel_color /= SAMPLES_PER_PIXEL as f64;
            let pixel = img.get_pixel_mut(i, image_height - 1 - j);
            *pixel = image::Rgb(pixel_color.to_uvec());
            progress.inc(1);
        }
    }
}
