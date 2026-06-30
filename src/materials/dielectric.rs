use crate::core;
use crate::core::vec3::Vec3;
use crate::core::ray::Ray;
use crate::core::hit_record::HitRecord;

use super::reflect;

#[derive(Copy, Clone)]
pub struct Dielectric {
    ref_idx: f64,
}

impl Dielectric {
    pub fn new(ref_idx: f64) -> Self {
        Self { ref_idx }
    }

    fn schlick(&self, cosine: f64) -> f64 {
        let r0 = (1.0 - self.ref_idx) / (1.0 + self.ref_idx);
        let r0 = r0 * r0;
        r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
    }

    pub fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord) -> Option<(Vec3, Ray)> {
        let outward_normal: Vec3;
        let reflected = reflect(ray_in.direction, hit_record.normal);
        let ni_over_nt: f64;
        let attenuation = Vec3::new(1.0, 1.0, 1.0);
        let mut refracted = Vec3::new(0.0, 0.0, 0.0);
        let cosine: f64;

        if core::dot(ray_in.direction, hit_record.normal) > 0.0 {
            outward_normal = -hit_record.normal;
            ni_over_nt = self.ref_idx;
            cosine = self.ref_idx * core::dot(ray_in.direction, hit_record.normal) / ray_in.direction.length();
        } else {
            outward_normal = hit_record.normal;
            ni_over_nt = 1.0 / self.ref_idx;
            cosine = -core::dot(ray_in.direction, hit_record.normal) / ray_in.direction.length();
        }

        let scattered: Ray;
        let reflect_prob: f64;
        if self.refract(&ray_in.direction, &outward_normal, ni_over_nt, &mut refracted) {
            reflect_prob = self.schlick(cosine);
        } else {
            reflect_prob = 1.0;
        }

        if core::random() < reflect_prob {
            scattered = Ray::new(hit_record.p, reflected, ray_in.time);
        } else {
            scattered = Ray::new(hit_record.p, refracted, ray_in.time);
        }

        Some((attenuation, scattered))
    }

    fn refract(&self, v: &Vec3, n: &Vec3, ni_over_nt: f64, refracted: &mut Vec3) -> bool {
        let uv = v.unit_vector();
        let dt = core::dot(uv, *n);
        let discriminant = 1.0 - ni_over_nt * ni_over_nt * (1.0 - dt * dt);
        if discriminant > 0.0 {
            *refracted = ni_over_nt * (uv - n * dt) - n * discriminant.sqrt();
            true
        } else {
            false
        }
    }
}