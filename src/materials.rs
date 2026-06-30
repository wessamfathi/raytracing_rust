use crate::core::vec3::Vec3;
use crate::core;

pub mod lambertian;
pub mod metal;
pub mod dielectric;

pub fn reflect(v: Vec3, n: Vec3) -> Vec3 {
    v - 2.0 * core::dot(v, n) * n
}

#[derive(Copy, Clone)]
pub enum Material {
    Lambertian(lambertian::Lambertian),
    Metal(metal::Metal),
    Dielectric(dielectric::Dielectric),
}

impl Material {
    pub fn scatter(&self, ray_in: &core::ray::Ray, hit_record: &core::hit_record::HitRecord) -> Option<(Vec3, core::ray::Ray)> {
        match self {
            Material::Lambertian(lambertian) => lambertian.scatter(ray_in, hit_record),
            Material::Metal(metal) => metal.scatter(ray_in, hit_record),
            Material::Dielectric(dielectric) => dielectric.scatter(ray_in, hit_record),
        }
    }
}
