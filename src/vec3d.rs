#[derive(Debug, Clone, Copy)]
struct Vec3d {
    x: f64,
    y: f64,
    z: f64,
}

impl Vec3d {
    fn new(x: f64, y: f64, z: f64) -> Self {
        Vec3d { x, y, z }
    }
    
    fn zero() -> Self {
        Vec3d { x: 0.0, y: 0.0, z: 0.0 }
    }
    
    fn length(&self) -> f64 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }
    
    fn dot(&self, other: Vec3d) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    fn cross(&self, other: Vec3d) -> Vec3d {
        Vec3d {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }
}

impl std::ops::Add for Vec3d {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Vec3d {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl std::ops::Sub for Vec3d {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Vec3d {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}
