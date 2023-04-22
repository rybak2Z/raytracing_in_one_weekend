use crate::rendering::Point3;

use rand::prelude::*;

const POINT_COUNT: usize = 256;

pub struct PerlinNoise {
    random_floats: [f64; POINT_COUNT],
    perm_x: [usize; POINT_COUNT],
    perm_y: [usize; POINT_COUNT],
    perm_z: [usize; POINT_COUNT],
}

impl PerlinNoise {
    pub fn new() -> PerlinNoise {
        let mut random_floats: [f64; 256] = [0.0; POINT_COUNT];
        let mut rng = thread_rng();
        for element in random_floats.iter_mut() {
            *element = rng.gen();
        }

        let perm_x = Self::perlin_generate_perm();
        let perm_y = Self::perlin_generate_perm();
        let perm_z = Self::perlin_generate_perm();

        PerlinNoise {
            random_floats,
            perm_x,
            perm_y,
            perm_z,
        }
    }

    pub fn noise(&self, point: Point3) -> f64 {
        let i = ((4.0 * point.x()) as i32 & 255) as usize;
        let j = ((4.0 * point.y()) as i32 & 255) as usize;
        let k = ((4.0 * point.z()) as i32 & 255) as usize;

        self.random_floats[self.perm_x[i] ^ self.perm_y[j] ^ self.perm_z[k]]
    }

    fn perlin_generate_perm() -> [usize; POINT_COUNT] {
        let mut p = [0; POINT_COUNT];
        for (i, element) in p.iter_mut().enumerate() {
            *element = i;
        }
        Self::permute(&mut p);
        p
    }

    fn permute(p: &mut [usize; POINT_COUNT]) {
        let mut rng = thread_rng();
        for i in (1..=(POINT_COUNT - 1)).rev() {
            let target = rng.gen_range(0..i);
            (p[i], p[target]) = (p[target], p[i]);
        }
    }
}

impl Default for PerlinNoise {
    fn default() -> Self {
        Self::new()
    }
}
