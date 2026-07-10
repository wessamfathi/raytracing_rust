extern crate overload;
use overload::overload;
use std::ops;

#[derive(Clone, Copy)]
pub struct Vec3 {
    pub e: [f64; 3],
}

overload!(- (u: ?Vec3) -> Vec3 { Vec3::new(-u.e[0], -u.e[1], -u.e[2]) } );

overload!((u: ?Vec3) + (v: ?Vec3) -> Vec3 {
	Vec3::new(u.e[0] + v.e[0], u.e[1] + v.e[1], u.e[2] + v.e[2])
} );

overload!((u: ?Vec3) - (v: ?Vec3) -> Vec3 {
	Vec3::new(u.e[0] - v.e[0], u.e[1] - v.e[1], u.e[2] - v.e[2])
} );

overload!((u: ?Vec3) * (v: ?Vec3) -> Vec3 {
	Vec3::new(u.e[0] * v.e[0], u.e[1] * v.e[1], u.e[2] * v.e[2])
} );

overload!((v: ?Vec3) * (t: f64) -> Vec3 {
	Vec3::new(v.e[0] * t, v.e[1] * t, v.e[2] * t)
} );

overload!((t: f64) * (v: ?Vec3) -> Vec3 {
	v * t
} );

overload!((v: ?Vec3) / (t: f64) -> Vec3 {
	(1.0/t) * v
} );

overload!((u: &mut Vec3) += (v: Vec3) {
    u.e[0] += v.e[0];
    u.e[1] += v.e[1];
    u.e[2] += v.e[2];
});

overload!((u: &mut Vec3) -= (v: Vec3) {
    u.e[0] -= v.e[0];
    u.e[1] -= v.e[1];
    u.e[2] -= v.e[2];
});

overload!((u: &mut Vec3) *= (v: Vec3) {
    u.e[0] *= v.e[0];
    u.e[1] *= v.e[1];
    u.e[2] *= v.e[2];
});

overload!((u: &mut Vec3) /= (v: Vec3) {
    u.e[0] /= v.e[0];
    u.e[1] /= v.e[1];
    u.e[2] /= v.e[2];
});

impl Vec3 {
    pub fn vec3() -> Vec3 {
        Vec3 { e: [0.0, 0.0, 0.0] }
    }

    pub fn new(e0: f64, e1: f64, e2: f64) -> Vec3 {
        Vec3 { e: [e0, e1, e2] }
    }

    fn get(&self, index: usize) -> f64 {
        self.e[index]
    }

    pub fn x(&self) -> f64 {
        self.e[0]
    }

    pub fn y(&self) -> f64 {
        self.e[1]
    }

    pub fn z(&self) -> f64 {
        self.e[2]
    }

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn length_squared(&self) -> f64 {
        self.e[0] * self.e[0] + self.e[1] * self.e[1] + self.e[2] * self.e[2]
    }

    pub fn cross(&self, other: Vec3) -> Vec3 {
        Vec3 {
            e: [
                self.e[1] * other.e[2] - self.e[2] * other.e[1],
                self.e[2] * other.e[0] - self.e[0] * other.e[2],
                self.e[0] * other.e[1] - self.e[1] * other.e[0],
            ],
        }
    }

    pub fn unit_vector(&self) -> Vec3 {
        self / self.length()
    }

    pub fn sqrt(&mut self) {
        self.e[0] = self.e[0].sqrt();
        self.e[1] = self.e[1].sqrt();
        self.e[2] = self.e[2].sqrt();
    }

    pub fn print(&self) -> String {
        let r = super::clamp(self.e[0], 0.0, 0.999);
        let g = super::clamp(self.e[1], 0.0, 0.999);
        let b = super::clamp(self.e[2], 0.0, 0.999);

        format!(
            "{} {} {}\n",
            (256.0 * r) as i32,
            (256.0 * g) as i32,
            (256.0 * b) as i32
        )
    }
}
