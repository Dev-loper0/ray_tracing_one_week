#[derive(Debug)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Vec3 {
        Vec3 { x, y, z }
    }

    // methods
    pub fn op_add(&mut self, u: &Vec3) {
        self.x += u.x;
        self.y += u.y;
        self.z += u.z;
    }

    pub fn op_mul(&mut self, t: f64) {
        self.x *= t;
        self.y *= t;
        self.z *= t;
    }

    pub fn op_div(&mut self, t: f64) {
        self.x *= 1 as f64 / t;
        self.y *= 1 as f64 / t;
        self.z *= 1 as f64 / t;
    }

    pub fn length(&mut self) -> f64 {
        f64::sqrt(self.length_squard())
    }

    pub fn length_squard(&mut self) -> f64 {
        self.x.powi(2) + self.y.powi(2) + self.z.powi(2)
    }
}

//type aliases for Vec3
pub type Point = Vec3;
pub type Color = Vec3;

// functions
pub fn add(u: &Vec3, v: &Vec3) -> Vec3 {
    let x = u.x + v.x;
    let y = u.y + v.y;
    let z = u.z + v.z;

    Vec3::new(x, y, z)
}

pub fn sub(u: &Vec3, v: &Vec3) -> Vec3 {
    let x = u.x - v.x;
    let y = u.y - v.y;
    let z = u.z - v.z;

    Vec3::new(x, y, z)
}

pub fn mul(u: &Vec3, v: &Vec3) -> Vec3 {
    let x = u.x * v.x;
    let y = u.y * v.y;
    let z = u.z * v.z;

    Vec3::new(x, y, z)
}
pub fn op_mul(u: &Vec3, t: f64) -> Vec3 {
    let x = u.x * t;
    let y = u.y * t;
    let z = u.z * t;

    Vec3::new(x, y, z)
}

pub fn div(u: &Vec3, t: f64) -> Vec3 {
    let x = u.x * 1 as f64 / t;
    let y = u.y * 1 as f64 / t;
    let z = u.z * 1 as f64 / t;

    Vec3::new(x, y, z)
}

pub fn dot(u: &Vec3, v: &Vec3) -> f64 {
    u.x * v.x + u.y * v.y + u.z * v.z
}

pub fn cross(u: &Vec3, v: &Vec3) -> Vec3 {
    Vec3::new(
        u.y * v.z - u.z - v.y,
        u.z * v.x - u.x * v.z,
        u.x * v.y - u.y * v.x,
    )
}
