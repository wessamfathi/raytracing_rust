use crate::core::vec3::Vec3;
use crate::core::ray::Ray;
use crate::core::hit_record::HitRecord;
use crate::core::random_in_unit_sphere;

#[derive(Copy, Clone)]
pub struct Lambertian {
    albedo: Vec3,
}

impl Lambertian {
    pub fn new(albedo: Vec3) -> Self {
        Self { albedo }
    }

    pub fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord) -> Option<(Vec3, Ray)> {
        let target = hit_record.p + hit_record.normal + random_in_unit_sphere();
        let scattered = Ray::new(hit_record.p, target - hit_record.p, ray_in.time);
        Some((self.albedo, scattered))
    }
}
