use crate::{
    material::lambertian::Lambertian,
    math_util::Point3,
    models::{hittable::HittableList, sphere::Sphere},
    texture::image_texture::ImageTexture,
};
use std::rc::Rc;

pub fn earth() -> HittableList {
    let earth_texture = Rc::new(ImageTexture::new("texture_image/earthmap.jpg"));
    let earth_surface = Rc::new(Lambertian::new_texture(earth_texture));
    let globe = Rc::new(Sphere::new(Point3::new(0.0, 0.0, 0.0), 2.0, earth_surface));

    HittableList::new(globe)
}
