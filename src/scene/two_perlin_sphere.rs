use std::rc::Rc;

use crate::{
    material::lambertian::Lambertian,
    math_util::Point3,
    models::{hittable::HittableList, sphere::Sphere},
    texture::noise::NoiseTexture,
};

pub fn two_perlin_sphere() -> HittableList {
    let mut objects = HittableList::default();
    let pertext = Rc::new(NoiseTexture::new_scale(4.0));
    let permat = Rc::new(Lambertian::new_texture(pertext));
    objects.add(Rc::new(Sphere::new(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        permat.clone(),
    )));
    objects.add(Rc::new(Sphere::new(
        Point3::new(0.0, 2.0, 0.0),
        2.0,
        permat,
    )));

    objects
}
