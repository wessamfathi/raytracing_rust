use crate::core;
use crate::core::ray::Ray;
use crate::core::vec3::Vec3;

use super::PI;

pub const ASPECT_RATIO: f64 = 16.0 / 9.0;

pub struct Camera {
    origin: Vec3,
    lower_left_corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
    u: Vec3,
    v: Vec3,
    w: Vec3,
    lens_radius: f64,
    open_time: f64,
    close_time: f64,
}

impl Camera {
    pub fn new(look_from: Vec3, look_at: Vec3, v_up: Vec3, v_fov: f64, aspect: f64, aperture: f64, focus_dist: f64, open_time: f64, close_time: f64) -> Camera {
        let lens_radius = aperture / 22.0;
        let theta = v_fov * PI / 180.0;
        let half_height = (theta / 2.0).tan();
        let half_width = aspect * half_height;
        let origin = look_from;
        let w = (look_from - look_at).unit_vector();
        let u = v_up.cross(w).unit_vector();
        let v = w.cross(u);
        let lower_left_corner = origin - half_width * focus_dist * u - half_height * focus_dist * v - focus_dist * w;
        let horizontal = 2.0 * half_width * focus_dist * u;
        let vertical = 2.0 * half_height * focus_dist * v;

        Camera {
            origin,
            lower_left_corner,
            horizontal,
            vertical,
            u,
            v,
            w,
            lens_radius,
            open_time,
            close_time,
        }
    }

    fn random_in_unit_disk() -> Vec3 {
        loop {
            let p = Vec3::new(core::random(), core::random(), 0.0) * 2.0 - Vec3::new(1.0, 1.0, 0.0);
            if p.length_squared() < 1.0 {
                return p;
            }
        }
    }

    pub fn get_ray(&self, s: f64, t: f64) -> Ray {
        let rd = Camera::random_in_unit_disk() * self.lens_radius;
        let offset = self.u * rd.x() + self.v * rd.y();
        let time = self.open_time + core::random() * (self.close_time - self.open_time);
        Ray {
            origin: self.origin + offset,
            direction: self.lower_left_corner + s * self.horizontal + t * self.vertical - self.origin - offset,
            time,
        }
    }
}
