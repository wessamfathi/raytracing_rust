use rand::{Rng, thread_rng};
use vec3::Vec3;

pub mod camera;
pub mod hit_record;
pub mod ray;
pub mod vec3;

pub const PI: f64 = 3.1415926535897932385;
pub const INFINITY: f64 = f64::INFINITY;

pub fn dot(u: Vec3, v: Vec3) -> f64 {
    u.e[0] * v.e[0] + u.e[1] * v.e[1] + u.e[2] * v.e[2]
}

fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * PI / 180.0
}

pub fn random() -> f64 {
    // Returns a random real in [0,1)
    let mut rng = thread_rng();
    rng.gen_range(0.0..1.0)
}

fn random_range(min: f64, max: f64) -> f64 {
    // Returns a random real in [min,max])
    let mut rng = thread_rng();
    rng.gen_range(min..max)
}

pub fn random_in_unit_sphere() -> Vec3 {
    loop {
        let p = 2.0 * Vec3::new(random(), random(), random()) - Vec3::new(1.0, 1.0, 1.0);
        if p.length_squared() < 1.0 {
            return p;
        }
    }
}
