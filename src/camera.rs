use crate::math_util::{cross, rand_range, Point3, Ray, Vec3};

#[derive(Clone)]
pub struct Camera {
    origin: Point3,
    lower_left_corner: Point3,
    horizontal: Vec3,
    vertical: Vec3,
    u: Vec3,
    v: Vec3,
    //w: Vec3,
    lens_radius: f64,
    time0: f64,
    time1: f64,
}

impl Default for Camera {
    fn default() -> Self {
        Self::new(
            Point3::default(),
            Vec3::new(0.0, 0.0, -1.0),
            Vec3::new(0.0, 1.0, 0.0),
            90.0,
            16.0 / 9.0,
            0.0,
            1.0,
            0.0,
            0.0,
        )
    }
}

#[allow(clippy::too_many_arguments)]
impl Camera {
    pub fn new(
        lookfrom: Point3,
        lookat: Point3,
        vup: Vec3,
        vfov: f64, //vertical field-of-view in degrees
        aspect_ratio: f64,
        aperture: f64,
        focus_dist: f64,
        time0: f64,
        time1: f64,
    ) -> Self {
        let theta = vfov.to_radians();
        let h = f64::tan(theta / 2.0);
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;

        let w = (lookfrom - lookat).unit_vector();
        let u = (cross(&vup, &w)).unit_vector();
        let v = cross(&w, &u);

        let origin = lookfrom;
        let horizontal = focus_dist * viewport_width * u;
        let vertical = focus_dist * viewport_height * v;
        let lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0 - focus_dist * w;

        let lens_radius = aperture / 2.0;
        Self {
            origin,
            lower_left_corner,
            horizontal,
            vertical,
            u,
            v,
            //w,
            lens_radius,
            time0,
            time1,
        }
    }
    pub fn get_ray(&self, s: f64, t: f64) -> Ray {
        let rd = self.lens_radius * Vec3::random_in_unit_disk();
        let offset = self.u * rd.x() + self.v * rd.y();

        Ray::new_time(
            self.origin + offset,
            self.lower_left_corner + s * self.horizontal + t * self.vertical - self.origin - offset,
            rand_range(self.time0, self.time1),
        )
    }
}
