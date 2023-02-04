pub struct Vec3 {
    data: [f64; 3],
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Vec3 { data: [x, y, z] }
    }

    pub fn x(&self) -> f64 {
        self.data[0]
    }

    pub fn y(&self) -> f64 {
        self.data[1]
    }

    pub fn z(&self) -> f64 {
        self.data[2]
    }
}

//pub type Point3 = Vec3;
pub type Color3 = Vec3;
