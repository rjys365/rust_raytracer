use rand::{seq::SliceRandom, thread_rng};

use crate::math_util::{dot, Point3, Vec3};

const POINT_COUNT: usize = 256;
pub struct Perlin {
    ranvec: [Vec3; POINT_COUNT],
    perm_x: [i32; POINT_COUNT],
    perm_y: [i32; POINT_COUNT],
    perm_z: [i32; POINT_COUNT],
}

impl Default for Perlin {
    fn default() -> Self {
        Self::new()
    }
}

#[allow(clippy::needless_range_loop)]
impl Perlin {
    fn generate_perm() -> [i32; POINT_COUNT] {
        let mut p: [i32; POINT_COUNT] = [0; POINT_COUNT];
        for (i, it) in p.iter_mut().enumerate() {
            *it = i as i32;
        }
        let mut rng = thread_rng();
        p.shuffle(&mut rng);
        p
    }
    fn perlin_interp(c: [[[Vec3; 2]; 2]; 2], u: f64, v: f64, w: f64) -> f64 {
        let uu = u * u * (3.0 - 2.0 * u);
        let vv = v * v * (3.0 - 2.0 * v);
        let ww = w * w * (3.0 - 2.0 * w);
        let mut accum = 0.0;
        for ii in 0..2 {
            for jj in 0..2 {
                for kk in 0..2 {
                    let i = ii as f64;
                    let j = jj as f64;
                    let k = kk as f64;
                    let weight_v = Vec3::new(u - i as f64, v - j as f64, w - k as f64);
                    accum += (i * uu + (1.0 - i) * (1.0 - uu))
                        * (j * vv + (1.0 - j) * (1.0 - vv))
                        * (k * ww + (1.0 - k) * (1.0 - ww))
                        * dot(&c[ii][jj][kk], &weight_v);
                }
            }
        }
        accum
    }

    pub fn new() -> Self {
        let mut ranvec: [Vec3; POINT_COUNT] = [Vec3::default(); POINT_COUNT];
        for i in ranvec.iter_mut() {
            *i = Vec3::random_range(-1.0, 1.0).unit_vector();
        }
        let perm_x = Self::generate_perm();
        let perm_y = Self::generate_perm();
        let perm_z = Self::generate_perm();
        Self {
            ranvec,
            perm_x,
            perm_y,
            perm_z,
        }
    }
    pub fn noise(&self, p: &Point3) -> f64 {
        let u = p.x() - p.x().floor();
        let v = p.y() - p.y().floor();
        let w = p.z() - p.z().floor();

        let i = p.x().floor() as i32;
        let j = p.y().floor() as i32;
        let k = p.z().floor() as i32;
        let mut c: [[[Vec3; 2]; 2]; 2] = [[[Vec3::default(); 2]; 2]; 2];

        for di in 0..2 {
            for dj in 0..2 {
                for dk in 0..2 {
                    c[di][dj][dk] = self.ranvec[(self.perm_x[((i + di as i32) & 255) as usize]
                        ^ self.perm_y[((j + dj as i32) & 255) as usize]
                        ^ self.perm_z[((k + dk as i32) & 255) as usize])
                        as usize]
                }
            }
        }
        (Self::perlin_interp(c, u, v, w) + 1.0) * 0.5
    }
}
