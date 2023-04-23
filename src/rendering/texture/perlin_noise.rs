use crate::rendering::{Point3, Vec3};

use rand::prelude::*;

const POINT_COUNT: usize = 256;

// Uses pointers to keep it at roughly same size scale as the other TextureEnum values
// Makes use of Boxes because normal references would cause annoying lifetime problems
pub struct PerlinNoise {
    random_vectors: Box<[Vec3; POINT_COUNT]>,
    perm_x: Box<[usize; POINT_COUNT]>,
    perm_y: Box<[usize; POINT_COUNT]>,
    perm_z: Box<[usize; POINT_COUNT]>,
}

impl PerlinNoise {
    pub fn new() -> PerlinNoise {
        let mut random_vectors: [Vec3; 256] = [Vec3::default(); 256];
        for vec in random_vectors.iter_mut() {
            *vec = Vec3::random_range(-1.0, 1.0).normalized();
        }

        PerlinNoise {
            random_vectors: Box::new(random_vectors),
            perm_x: Box::new(Self::perlin_generate_perm()),
            perm_y: Box::new(Self::perlin_generate_perm()),
            perm_z: Box::new(Self::perlin_generate_perm()),
        }
    }

    pub fn noise(&self, point: Point3) -> f64 {
        let u = point.x() - point.x().floor();
        let v = point.y() - point.y().floor();
        let w = point.z() - point.z().floor();

        let i = point.x().floor() as isize;
        let j = point.y().floor() as isize;
        let k = point.z().floor() as isize;
        let mut c = [[[Vec3::default(); 2]; 2]; 2];

        for (di, c1) in c.iter_mut().enumerate() {
            for (dj, c2) in c1.iter_mut().enumerate() {
                for (dk, element) in c2.iter_mut().enumerate() {
                    *element = self.random_vectors[
                        self.perm_x[((i + di as isize) & 255) as usize]
                        ^ self.perm_y[((j + dj as isize) & 255) as usize]
                        ^ self.perm_z[((k + dk as isize) & 255) as usize]
                    ];
                }
            }
        }

        Self::perlin_interpolate(c, u, v, w)
    }

    pub fn turbulence(&self, point: Point3, depth: u32) -> f64 {
        let mut accumulated = 0.0;
        let mut temp_point = point;
        let mut weight = 1.0;

        for _ in 0..depth {
            accumulated += weight * self.noise(temp_point);
            weight *= 0.5;
            temp_point = Point3::from(2.0 * Vec3::from(temp_point));
        }

        accumulated.abs()
    }

    fn perlin_interpolate(c: [[[Vec3; 2]; 2]; 2], u: f64, v: f64, w: f64) -> f64 {
        let u = u * u * (3.0 - 2.0 * u);
        let v = v * v * (3.0 - 2.0 * v);
        let w = w * w * (3.0 - 2.0 * w);

        let mut accumulated = 0.0;
        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let (i, j, k) = (i as f64, j as f64, k as f64);
                    let weight_vec = Vec3::new(u - i, v - j, w - k);
                    accumulated += (i * u + (1.0 - i) * (1.0 - u))
                        * (j * v + (1.0 - j) * (1.0 - v))
                        * (k * w + (1.0 - k) * (1.0 - w))
                        * Vec3::dot(c[i as usize][j as usize][k as usize], weight_vec);
                }
            }
        }

        accumulated
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
