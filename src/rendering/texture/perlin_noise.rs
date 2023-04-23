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
        let u = point.x() - point.x().floor();
        let v = point.y() - point.y().floor();
        let w = point.z() - point.z().floor();

        let u = u * u * (3.0 - 2.0 * u);
        let v = v * v * (3.0 - 2.0 * v);
        let w = w * w * (3.0 - 2.0 * w);

        let i = point.x().floor() as isize;
        let j = point.y().floor() as isize;
        let k = point.z().floor() as isize;
        let mut c = [[[0.0; 2]; 2]; 2];

        for (di, c1) in c.iter_mut().enumerate() {
            for (dj, c2) in c1.iter_mut().enumerate() {
                for (dk, element) in c2.iter_mut().enumerate() {
                    *element = self.random_floats[
                        self.perm_x[((i + di as isize) & 255) as usize]
                        ^ self.perm_y[((j + dj as isize) & 255) as usize]
                        ^ self.perm_z[((k + dk as isize) & 255) as usize]];
                }
            }
        }

        trilinear_interp(c, u, v, w)
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

fn trilinear_interp(c: [[[f64; 2]; 2]; 2], u: f64, v: f64, w: f64) -> f64 {
    let mut accumulated = 0.0;
    for i in 0..2 {
        for j in 0..2 {
            for k in 0..2 {
                let (i, j, k) = (i as f64, j as f64, k as f64);
                accumulated += (i * u + (1.0 - i) * (1.0 - u))
                    * (j * v + (1.0 - j) * (1.0 - v))
                    * (k * w + (1.0 - k) * (1.0 - w))
                    * c[i as usize][j as usize][k as usize];
            }
        }
    }

    accumulated
}
