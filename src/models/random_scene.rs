use crate::material::dielectric::Dieletric;
use crate::material::lambertian::Lambertian;
use crate::material::metal::Metal;
use crate::math_util::{rand_double, rand_range, Color, Point3, Vec3};
use crate::texture::checker_texture::CheckerTexture;

use super::hittable::HittableList;
use super::moving_sphere::MovingSphere;
use super::sphere::Sphere;
use std::rc::Rc;
pub fn random_scene() -> HittableList {
    let mut world: HittableList = HittableList::default();
    let checker = Rc::new(CheckerTexture::new_color(
        Color::new(0.2, 0.3, 0.1),
        Color::new(0.9, 0.9, 0.9),
    ));
    // let ground_material = Rc::new(Lambertian::new(Color::new(0.5, 0.5, 0.5)));
    world.add(Rc::new(Sphere::new(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        Rc::new(Lambertian::new_texture(checker)),
    )));
    let material_glass = Rc::new(Dieletric::new(1.5));
    for a in -11..10 {
        for b in -11..10 {
            let choose_mat = rand_double();
            let center = Point3::new(
                a as f64 + 0.9 * rand_double(),
                0.2,
                b as f64 + 0.9 * rand_double(),
            );
            if (center - Point3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_mat < 0.8 {
                    //diffuse
                    let albedo = Color::random() * Color::random();
                    let sphere_material = Rc::new(Lambertian::new_solid_color(albedo));
                    let center2 = center + Vec3::new(0.0, rand_range(0.0, 0.5), 0.0);
                    world.add(Rc::new(MovingSphere::new(
                        center,
                        center2,
                        0.0,
                        1.0,
                        0.2,
                        sphere_material,
                    )));
                } else if choose_mat < 0.95 {
                    //metal
                    let albedo = Color::random_range(0.5, 1.0);
                    let fuzz = rand_range(0.0, 0.5);
                    let sphere_material = Rc::new(Metal::new(albedo, fuzz));
                    world.add(Rc::new(Sphere::new(center, 0.2, sphere_material)));
                } else {
                    //glass
                    world.add(Rc::new(Sphere::new(center, 0.2, material_glass.clone())));
                }
            }
        }
    }
    world.add(Rc::new(Sphere::new(
        Point3::new(0.0, 1.0, 0.0),
        1.0,
        material_glass,
    )));

    let material2 = Rc::new(Lambertian::new_solid_color(Color::new(0.4, 0.2, 0.1)));
    world.add(Rc::new(Sphere::new(
        Point3::new(-4.0, 1.0, 0.0),
        1.0,
        material2,
    )));

    let material3 = Rc::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0));
    world.add(Rc::new(Sphere::new(
        Point3::new(4.0, 1.0, 0.0),
        1.0,
        material3,
    )));

    world
}
