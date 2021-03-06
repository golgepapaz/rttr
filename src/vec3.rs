use std::ops;


#[derive(Copy, Clone)]
    pub struct Vec3 {
    pub x:f64,
    pub y:f64,
    pub z:f64
}

impl Vec3 {
    pub fn new(x: f64, y:f64, z:f64)->Vec3 {
        Vec3 {x, y, z}
    }
    pub fn squared_length(self)->f64 {
        self.x.powi(2) + self.y.powi(2) + self.z.powi(2)
    }
    pub fn length(self)->f64 {
        self.squared_length().sqrt()
    }
    pub fn unit(self)-> Vec3 {
        self / self.length()
    }

    pub fn dot(self, v:Vec3)->f64 {
        self.x * v.x + self.y * v.y + self.z + v.z
    }
    pub fn cross(self, v:Vec3)-> Vec3 {
        Vec3::new( 
            self.y * v.z - self.z * v.y,
            self.z * v.x - self.x * v.z,
            self.x * v.y - self.y * v.x,
        )
    }
}

impl ops::Div<Vec3> for Vec3 {
    type Output = Vec3;
    fn div(self, v:Vec3) ->Vec3 {
        Vec3::new(self.x / v.x, self.y / v.y, self.z / v.z)       
    }
}
impl ops::Div<f64> for Vec3 {
    type Output = Vec3;
    fn div(self, v: f64)-> Vec3 {
        Vec3::new(self.x / v, self.y /v, self.z / v)
    }
}

impl ops::Mul<f64> for Vec3 {
    type Output = Vec3;
    fn mul(self, f: f64)-> Vec3 {
        Vec3::new(self.x * f, self.y * f, self.z * f)
    }
}
impl ops::Add<Vec3> for Vec3 {
    type Output = Vec3;
    fn add(self, v:Vec3)-> Vec3 {
        Vec3::new (self.x + v.x, self.y + v.y, self.z + v.z)
    }
}
impl ops::Sub<Vec3> for Vec3 {
    type Output = Vec3;
    fn sub(self, v:Vec3)-> Vec3 {
        Vec3::new (self.x - v.x, self.y - v.y, self.z -v.z)
    }
}