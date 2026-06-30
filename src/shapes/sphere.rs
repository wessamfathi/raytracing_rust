use crate::core::dot;
use crate::core::hit_record::HitRecord;
use crate::core::ray::Ray;
use crate::core::vec3::Vec3;
use crate::materials::Material;

pub struct Sphere {
    pub center: Vec3,
    pub center0: Vec3,
    pub center1: Vec3,
    pub radius: f64,
    pub time0: f64,
    pub time1: f64,
	pub is_moving: bool,
	pub material: Material,
}

impl Sphere {
	pub fn new_moving(center0: Vec3, center1: Vec3, radius: f64, time0: f64, time1: f64, material: Material) -> Self {
        Sphere {
			center: center0,
            center0,
            center1,
            radius,
            time0,
            time1,
			is_moving: true,
            material,
        }
    }

    pub fn center_moving(&self, time: f64) -> Vec3 {
        self.center0 + ((time - self.time0) / (self.time1 - self.time0)) * (self.center1 - self.center0)
    }

	pub fn hit(&self, r: Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
		if self.is_moving {
			self.hit_moving(r, t_min, t_max, rec)
		} else {
			self.hit_static(r, t_min, t_max, rec)
		}
	}

	pub fn hit_moving(&self, r: Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let oc = r.origin - self.center_moving(r.time);
        let a = r.direction.length_squared();
        let half_b = dot(oc, r.direction);
        let c = oc.length_squared() - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;
        if discriminant > 0.0 {
            let sqrt_d = discriminant.sqrt();
            let mut temp = (-half_b - sqrt_d) / a;
            if temp < t_max && temp > t_min {
                rec.t = temp;
                rec.p = r.at(rec.t);
                let outward_normal = (rec.p - self.center_moving(r.time)) / self.radius;
                rec.set_face_normal(r, outward_normal);
                rec.material = Some(self.material.clone());
                return true;
            }
            temp = (-half_b + sqrt_d) / a;
            if temp < t_max && temp > t_min {
                rec.t = temp;
                rec.p = r.at(rec.t);
                let outward_normal = (rec.p - self.center_moving(r.time)) / self.radius;
                rec.set_face_normal(r, outward_normal);
                rec.material = Some(self.material.clone());
                return true;
            }
        }
        false
    }

    pub fn hit_static(&self, r: Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let oc = r.origin - self.center;
		let a = dot(r.direction, r.direction);
		let b = dot(oc, r.direction);
		let c = dot(oc, oc) - self.radius * self.radius;

		let discriminant = b * b - a * c;
		if discriminant > 0.0 {
			let temp = (-b - discriminant.sqrt()) / a;
			if temp < t_max && temp > t_min {
				rec.t = temp;
                                rec.p = r.at(rec.t);
                                let outward_normal = (rec.p - self.center) / self.radius;
                                rec.set_face_normal(r, outward_normal);
                                rec.material = Some(self.material);
                                return true;
			}
			
			let temp = (-b + discriminant.sqrt()) / a;
			if temp < t_max && temp > t_min {
				rec.t = temp;
                                rec.p = r.at(rec.t);
                                let outward_normal = (rec.p - self.center) / self.radius;
                                rec.set_face_normal(r, outward_normal);
                                rec.material = Some(self.material);
                                return true;
			}
		}
		false
    }
}
