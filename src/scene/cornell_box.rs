use std::rc::Rc;

use crate::{models::{hittable::HittableList, yz_rect::YzRect, xz_rect::XzRect, xy_rect::XyRect, sphere::Sphere, constant_medium::ConstantMedium}, material::{lambertian::Lambertian, diffuse_light::DiffuseLight, dielectric::Dieletric}, math_util::{Color, Point3}};

pub fn cornell_box()->HittableList{
    let mut objects:HittableList=HittableList::default();
    
    let red=Rc::new(Lambertian::new_solid_color(Color::new(0.65,0.05,0.05)));
    let white=Rc::new(Lambertian::new_solid_color(Color::new(0.73,0.73,0.73)));
    let green=Rc::new(Lambertian::new_solid_color(Color::new(0.12,0.45,0.15)));
    let light=Rc::new(DiffuseLight::new_solid_color(Color::new(15.0,15.0,15.0)));

    objects.add(Rc::new(YzRect::new(0.0,555.0,0.0,555.0,555.0,green.clone())));
    objects.add(Rc::new(YzRect::new(0.0,555.0,0.0,555.0,0.0,red)));
    objects.add(Rc::new(XzRect::new(213.0,343.0,227.0,332.0,554.0,light)));
    objects.add(Rc::new(XzRect::new(0.0,555.0,0.0,555.0,0.0,white.clone())));
    objects.add(Rc::new(XzRect::new(0.0,555.0,0.0,555.0,555.0,white.clone())));
    objects.add(Rc::new(XyRect::new(0.0,555.0,0.0,555.0,555.0,white.clone())));
    
    let boundary=Rc::new(Sphere::new(Point3::new(165.0,165.0,165.0),200.0,Rc::new(Dieletric::new(1.5))));
    //objects.add(boundary.clone());
    objects.add(boundary.clone());
    objects.add(Rc::new(ConstantMedium::new_solid_color(boundary, 0.01, Color::new(0.12,0.45,0.15))));


    objects
}