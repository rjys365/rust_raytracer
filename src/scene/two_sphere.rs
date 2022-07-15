use std::rc::Rc;

use crate::{
    material::lambertian::Lambertian,
    math_util::{Color, Point3},
    models::{hittable::HittableList, sphere::Sphere},
    texture::checker_texture::CheckerTexture,
};

pub fn two_sphere() -> HittableList {
    let mut objects: HittableList = HittableList::default();
    let checker = Rc::new(CheckerTexture::new_color(
        Color::new(0.2, 0.3, 0.1),
        Color::new(0.9, 0.9, 0.9),
    ));
    objects.add(Rc::new(Sphere::new(
        Point3::new(0.0, -10.0, 0.0),
        10.0,
        Rc::new(Lambertian::new_texture(checker.clone())),
    )));
    objects.add(Rc::new(Sphere::new(
        Point3::new(0.0, 10.0, 0.0),
        10.0,
        Rc::new(Lambertian::new_texture(checker)),
    )));
    objects
}
