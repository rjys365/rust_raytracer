use rand::{seq::SliceRandom, thread_rng};

use crate::math_util::{rand_double, Point3};

const POINT_COUNT: usize = 256;
pub struct Perlin {
    ranfloat: [f64; POINT_COUNT],
    perm_x: [i32; POINT_COUNT],
    perm_y: [i32; POINT_COUNT],
    perm_z: [i32; POINT_COUNT],
}

impl Default for Perlin{
    fn default() -> Self {
        Self::new()
    }
}

impl Perlin {
    fn generate_perm() -> [i32; POINT_COUNT] {
        let mut p: [i32; POINT_COUNT] = [0; POINT_COUNT];
        for (i,it) in p.iter_mut().enumerate() {
            *it = i as i32;
        }
        let mut rng = thread_rng();
        p.shuffle(&mut rng);
        p
    }
    pub fn new() -> Self {
        let mut ranfloat: [f64; POINT_COUNT] = [0.0; POINT_COUNT];
        for i in ranfloat.iter_mut() {
            *i = rand_double();
        }
        let perm_x = Self::generate_perm();
        let perm_y = Self::generate_perm();
        let perm_z = Self::generate_perm();
        Self {
            ranfloat,
            perm_x,
            perm_y,
            perm_z,
        }
    }
    pub fn noise(&self, p: &Point3) -> f64 {
        let i = ((4.0 * p.x()) as i32 & 255) as usize;
        let j = ((4.0 * p.y()) as i32 & 255) as usize;
        let k = ((4.0 * p.z()) as i32 & 255) as usize;
        self.ranfloat[(self.perm_x[i] ^ self.perm_y[j] ^ self.perm_z[k]) as usize]
    }
}
