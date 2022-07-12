//anti-alias render
use crate::camera::Camera;
use crate::material::dielectric::Dieletric;
use crate::material::lambertian::Lambertian;
use crate::material::metal::Metal;
use crate::math_util::{rand_double, Color, Point3, Ray, Vec3};
use crate::models::hittable::{Hittable, HittableList};
use crate::models::sphere::Sphere;
use image::RgbImage;
use indicatif::ProgressBar;
//use std::f64::consts::PI;
use std::f64::INFINITY;
use std::rc::Rc;

fn ray_color(r: &Ray, world: &dyn Hittable, depth: i32) -> Color {
    if depth <= 0 {
        return Color::default();
    }
    if let Some(rec) = world.hit(r, 0.001, INFINITY) {
        if let Some((attenuation, scattered)) = rec.mat_ptr.scatter(r, &rec) {
            return attenuation * ray_color(&scattered, world, depth - 1);
        } else {
            return Color::default();
        }
    }
    let unit_direction = r.get_direction().unit_vector();
    let t = 0.5 * (unit_direction.get_y() + 1.0);
    Vec3::new(1.0, 1.0, 1.0) * (1.0 - t) + Vec3::new(0.5, 0.7, 1.0)
}

pub fn render(image_height: u32, image_width: u32, img: &mut RgbImage, progress: &ProgressBar) {
    const SAMPLES_PER_PIXEL: u32 = 100;
    const MAX_DEPTH: i32 = 50;
    //let aspect_ratio:f64=image_width as f64/image_height as f64;

    //World
    let mut world = HittableList::default();

    let material_ground = Rc::new(Lambertian {
        albedo: Color::new(0.8, 0.8, 0.0),
    });
    let material_center = Rc::new(Lambertian {
        albedo: Color::new(0.1, 0.2, 0.5),
    });
    let material_left = Rc::new(Dieletric::new(1.5));
    let material_right = Rc::new(Metal::new(Color::new(0.8, 0.6, 0.2), 0.0));

    world.add(Rc::new(Sphere::new(
        Point3::new(0.0, -100.5, -1.0),
        100.0,
        material_ground,
    )));
    world.add(Rc::new(Sphere::new(
        Point3::new(0.0, 0.0, -1.0),
        0.5,
        material_center,
    )));
    world.add(Rc::new(Sphere::new(
        Point3::new(-1.0, 0.0, -1.0),
        0.5,
        material_left,
    )));
    world.add(Rc::new(Sphere::new(
        Point3::new(1.0, 0.0, -1.0),
        0.5,
        material_right,
    )));
    //Camera
    let cam = Camera::default();

    //Render

    for j in (0..image_height - 1).rev() {
        for i in 0..image_width - 1 {
            let mut pixel_color = Color::default();
            for _k in 1..SAMPLES_PER_PIXEL {
                let u = (i as f64 + rand_double()) / (image_width as f64 - 1.0);
                let v = (j as f64 + rand_double()) / (image_height as f64 - 1.0);
                let r = cam.get_ray(u, v);
                pixel_color += ray_color(&r, &world, MAX_DEPTH);
            }
            pixel_color /= SAMPLES_PER_PIXEL as f64;
            let pixel = img.get_pixel_mut(i, image_height - 1 - j);
            *pixel = image::Rgb(pixel_color.to_uvec());
            progress.inc(1);
        }
    }
}
