use std::rc::Rc;

use crate::{models::{hittable::HittableList, sphere::Sphere, xz_rect::XzRect, yz_rect::YzRect, moving_sphere::MovingSphere, constant_medium::ConstantMedium}, math_util::{Vec3, Color}, material::{lambertian::Lambertian, diffuse_light::DiffuseLight, dielectric::Dieletric}, texture::image_texture::ImageTexture};

pub fn ball()->HittableList{
    let mut objects=HittableList::default();
    let s3=f64::sqrt(3.0);

    let ball_pos:[Vec3;10]=[
        Vec3::new(0.0,0.0,0.0),//0
        Vec3::new(-1.0,0.0,-s3),//1
        Vec3::new(1.0,0.0,-s3),//2
        Vec3::new(-2.0,0.0,-2.0*s3),//3
        Vec3::new(0.0,0.0,-2.0*s3),//4
        Vec3::new(2.0,0.0,-2.0*s3),//5
        Vec3::new(-3.0,0.0,-3.0*s3),//6
        Vec3::new(-1.0,0.0,-3.0*s3),//7
        Vec3::new(1.0,0.0,-3.0*s3),//8
        Vec3::new(3.0,0.0,-3.0*s3),//9
    ];
    let ball_col:[Vec3;10]=[
        Vec3::new_u(15,191,103),//1
        Vec3::new_u(77,210,219),//2
        Vec3::new_u(13,184,149),//2
        Vec3::new_u(11,75,224),//3
        Vec3::new_u(11,47,224),//3
        Vec3::new_u(68,59,245),//3
        Vec3::new_u(212, 110, 219),//4
        Vec3::new_u(173, 49, 153),//4
        Vec3::new_u(173, 16, 118),//4
        Vec3::new_u(140,14,67)//4
    ];
    let glass=Rc::new(Dieletric::new(1.5));

    for i in 0..10 {
        let bound=Rc::new(Sphere::new(ball_pos[i]*100.0,100.0,glass.clone()));
        objects.add(Rc::new(ConstantMedium::new_solid_color(bound.clone(),0.2,ball_col[i])));
        objects.add(bound);
        //objects.add(Rc::new(Sphere::new(ball_pos[i]*100.0,100.0,Rc::new(Lambertian::new_solid_color(ball_col[i])))));//triangle balls
    }

    objects.add(Rc::new(Sphere::new(Vec3::new(0.0,0.0,5.0)*100.0,100.0,Rc::new(Lambertian::new_texture(Rc::new(ImageTexture::new("texture_image/earthmap.jpg")))))));//red ball

    objects.add(Rc::new(MovingSphere::new(Vec3::new(230.0,0.0,395.0),Vec3::new(230.0,300.0,395.0),0.0,1.0,100.0,Rc::new(Lambertian::new_solid_color(Color::new(0.2,0.2,0.2))))));
    objects.add(Rc::new(MovingSphere::new(Vec3::new(-330.0,0.0,395.0),Vec3::new(-200.0,0.0,290.0),0.0,1.0,100.0,Rc::new(Lambertian::new_solid_color(Color::new(0.2,0.2,0.2))))));

    objects.add(Rc::new(XzRect::new(-10000.0,10000.0,-10000.0,10000.0,-100.0,Rc::new(Lambertian::new_solid_color(Color::new(0.2,0.9,0.2))))));//floor

    objects.add(Rc::new(XzRect::new(-400.0,200.0,-200.0,200.0,1000.0,Rc::new(DiffuseLight::new_solid_color(Color::new(16.0,16.0,16.0))))));//lamp
    objects.add(Rc::new(YzRect::new(-200.0,10000.0,-10000.0,10000.0,800.0,Rc::new(Lambertian::new_solid_color(Color::new(0.2,0.9,0.2))))));//back wall

    objects
}