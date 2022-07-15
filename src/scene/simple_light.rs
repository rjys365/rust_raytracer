use std::rc::Rc;

use crate::{
    material::{diffuse_light::DiffuseLight, lambertian::Lambertian},
    math_util::{Color, Point3},
    models::{hittable::HittableList, sphere::Sphere, xy_rect::XyRect},
    texture::noise::NoiseTexture,
};

pub fn simple_light() -> HittableList {
    let mut objects: HittableList = HittableList::default();

    let pertext = Rc::new(NoiseTexture::new_scale(4.0));
    objects.add(Rc::new(Sphere::new(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        Rc::new(Lambertian::new_texture(pertext.clone())),
    )));
    objects.add(Rc::new(Sphere::new(
        Point3::new(0.0, 2.0, 0.0),
        2.0,
        Rc::new(Lambertian::new_texture(pertext)),
    )));

    let difflight = Rc::new(DiffuseLight::new_solid_color(Color::new(4.0, 4.0, 4.0)));
    objects.add(Rc::new(XyRect::new(3.0, 5.0, 1.0, 3.0, -2.0, difflight)));

    objects
}
