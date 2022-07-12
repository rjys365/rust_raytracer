use crate::math_util::{Point3, Ray, Vec3};

#[derive(Clone, Copy)]
pub struct Camera {
    pub origin: Point3,
    pub lower_left_corner: Point3,
    pub horizontal: Vec3,
    pub vertical: Vec3,
}

impl Default for Camera {
    fn default() -> Self {
        Self::new(90.0,16.0/9.0)
    }
}

impl Camera {
    pub fn new(
        vfov:f64,//vertical field-of-view in degrees
        aspect_ratio:f64
    ) -> Self {
        let theta=vfov.to_radians();
        let h= f64::tan(theta/2.0);
        let viewport_height=2.0*h;
        let viewport_width=aspect_ratio*viewport_height;
        const FOCAL_LENGTH: f64 = 1.0;

        let origin = Point3::default();
        let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
        let vertical = Vec3::new(0.0, viewport_height, 0.0);
        let lower_left_corner =
            origin - horizontal / 2.0 - vertical / 2.0 - Vec3::new(0.0, 0.0, FOCAL_LENGTH);
        Self {
            origin,
            lower_left_corner,
            horizontal,
            vertical,
        }
    }
    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        Ray::new(
            self.origin,
            self.lower_left_corner + u * self.horizontal + v * self.vertical - self.origin,
        )
    }
}
