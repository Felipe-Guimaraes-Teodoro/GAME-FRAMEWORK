use std::{ops::{Add, Div, Mul, Sub}, process::Output};

pub fn distance(a: f32, b: f32) -> f32{
    return f32::sqrt(a*a + b*b);
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Vector2D {
    pub x: f32,
    pub y: f32,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Vector3D {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Vector4D {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

impl Vector2D {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    // returns the dot product of two given vectors
    pub fn dot(vec1: Vector2D, vec2: Vector2D) -> f32 {
        return vec1.x*vec2.x + vec1.y*vec2.y;
    }

    pub fn magnitude(self) -> f32 {
        return f32::sqrt(self.x*self.x + self.y*self.y)
    }

    pub fn normalize(self) -> Vector2D{
        return Vector2D::new(self.x, self.y)/self.magnitude();
    }
}

impl Vector3D {
    pub const ZERO: Vector3D = Vector3D { x: 0.0, y: 0.0, z: 0.0};

    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self{
            x, y, z
        }
    }

    // AKA: distance, length, module, .....
    pub fn magnitude(self) -> f32 {
        return f32::sqrt(self.x*self.x + self.y*self.y + self.z*self.z);
    }

    // retunrs the dot product between two 3d vectors
    pub fn dot(vec1: Vector3D, vec2: Vector3D) -> f32 {
        return vec1.x*vec2.x + vec1.y*vec2.y + vec1.z*vec2.z;
    }
    
    // returns the cross product of two given vectors
    pub fn cross(vec1: Vector3D, vec2: Vector3D) -> Vector3D {
        return Vector3D::new(vec1.y * vec2.z - vec1.z * vec1.y, vec1.z * vec1.x - vec1.x * vec2.z, vec1.x * vec2.y - vec1.y * vec2.x);
    }

    pub fn normalize(self) -> Vector3D{
        return Vector3D::new(self.x, self.y, self.z)/self.magnitude();
    }
}

impl Vector4D {
    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
        Vector4D { x, y, z, w }
    }

    pub fn dot(self, other: Vector4D) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z + self.w * other.w
    }

    pub fn magnitude(self) -> f32 {
        (self.x * self.x + self.y * self.y + self.z * self.z + self.w * self.w).sqrt()
    }

    pub fn normalize(self) -> Self {
        let mag = self.magnitude();
        if mag > 0.0 {
            Self {
                x: self.x / mag,
                y: self.y / mag,
                z: self.z / mag,
                w: self.w / mag,
            }
        } else {
            self
        }
    }
}


// some trait implementations for vector operations 

impl Add for Vector2D {
    type Output = Vector2D;

    fn add(self, rhs: Self) -> Self::Output {
        let x = self.x + rhs.x;
        let y = self.y + rhs.y;
        return Vector2D::new(x, y);
    }
}

impl Sub for Vector2D {
    type Output = Vector2D;

    fn sub(self, rhs: Self) -> Self::Output {
        let x = self.x - rhs.x;
        let y = self.y - rhs.y;
        return Vector2D::new(x, y);
    }
}

impl Mul<f32> for Vector2D {
    type Output = Vector2D;

    fn mul(self, rhs: f32) -> Self::Output {
        return Vector2D::new(self.x*rhs, self.y*rhs);
    }
}

impl Mul for Vector2D {
    type Output = Vector2D;

    fn mul(self, rhs: Self) -> Self::Output {
        return Vector2D::new(self.x*rhs.x, self.y*rhs.y);
    }
}

impl Div<f32> for Vector2D {
    type Output = Vector2D;

    fn div(self, rhs: f32) -> Self::Output {
        return Vector2D::new(self.x/rhs, self.y/rhs);
    }
}


// trait implementations for Vector3D
impl Add for Vector3D {
    type Output = Vector3D;

    fn add(self, rhs: Self) -> Self::Output {
        let x = self.x + rhs.x;
        let y = self.y + rhs.y;
        let z = self.z + rhs.z;
        return Vector3D::new(x, y, z);
    }
}

impl Sub for Vector3D {
    type Output = Vector3D;

    fn sub(self, rhs: Self) -> Self::Output {
        let x = self.x - rhs.x;
        let y = self.y - rhs.y;
        let z = self.z - rhs.z;
        return Vector3D::new(x, y, z);
    }
}

impl Mul<f32> for Vector3D {
    type Output = Vector3D;

    fn mul(self, rhs: f32) -> Self::Output {
        return Vector3D::new(self.x*rhs, self.y*rhs, self.z*rhs);
    }
}

impl Mul for Vector3D {
    type Output = Vector3D;

    fn mul(self, rhs: Self) -> Self::Output {
        return Vector3D::new(self.x*rhs.x, self.y*rhs.y, self.z*rhs.z);
    }
}

impl Div<f32> for Vector3D {
    type Output = Vector3D;

    fn div(self, rhs: f32) -> Self::Output {
        return Vector3D::new(self.x/rhs, self.y/rhs, self.z/rhs);
    }
}

// Trait implementations for vector4d

impl Add for Vector4D {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
            w: self.w + other.w,
        }
    }
}

impl Sub for Vector4D {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
            w: self.w - other.w,
        }
    }
}

impl Mul<f32> for Vector4D {
    type Output = Self;

    fn mul(self, scalar: f32) -> Self {
        Self {
            x: self.x * scalar,
            y: self.y * scalar,
            z: self.z * scalar,
            w: self.w * scalar,
        }
    }
}

impl Div<f32> for Vector4D {
    type Output = Self;

    fn div(self, scalar: f32) -> Self {
        Self {
            x: self.x / scalar,
            y: self.y / scalar,
            z: self.z / scalar,
            w: self.w / scalar,
        }
    }
}
