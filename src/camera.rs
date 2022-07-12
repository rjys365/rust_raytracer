use crate::math_util::{cross, Point3, Ray, Vec3};

#[derive(Clone, Copy)]
pub struct Camera {
    pub origin: Point3,
    pub lower_left_corner: Point3,
    pub horizontal: Vec3,
    pub vertical: Vec3,
}

impl Default for Camera {
    fn default() -> Self {
        Self::new(
            Point3::default(),
            Vec3::new(0.0, 0.0, -1.0),
            Vec3::new(0.0, 1.0, 0.0),
            90.0,
            16.0 / 9.0,
        )
    }
}

impl Camera {
    pub fn new(
        lookfrom: Point3,
        lookat: Point3,
        vup: Vec3,
        vfov: f64, //vertical field-of-view in degrees
        aspect_ratio: f64,
    ) -> Self {
        let theta = vfov.to_radians();
        let h = f64::tan(theta / 2.0);
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;

        let w = (lookfrom - lookat).unit_vector();
        let u = (cross(&vup, &w)).unit_vector();
        let v = cross(&w, &u);
        let origin = lookfrom;
        let horizontal = viewport_width * u;
        let vertical = viewport_height * v;
        let lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0 - w;
        Self {
            origin,
            lower_left_corner,
            horizontal,
            vertical,
        }
    }
    pub fn get_ray(&self, s: f64, t: f64) -> Ray {
        Ray::new(
            self.origin,
            self.lower_left_corner + s * self.horizontal + t * self.vertical - self.origin,
        )
    }
}
