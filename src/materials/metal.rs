use crate::core::vec3::Vec3;
use crate::core::ray::Ray;
use crate::core::hit_record::HitRecord;
use crate::core;
use crate::materials;

#[derive(Copy, Clone)]
pub struct Metal {
    albedo: Vec3,
}

impl Metal {
    pub fn new(albedo: Vec3) -> Self {
        Self { albedo }
    }

    pub fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord) -> Option<(Vec3, Ray)> {
        let reflected = materials::reflect(ray_in.direction.unit_vector(), hit_record.normal);
        let scattered = Ray::new(hit_record.p, reflected);
        if core::dot(scattered.direction, hit_record.normal) > 0.0 {
            Some((self.albedo, scattered))
        } else {
            None
        }
    }
}
