use std::{ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign}, process::Output};

pub fn distance(a: f32, b: f32) -> f32{
    return f32::sqrt(a*a + b*b);
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Vector2D {
    pub x: f32,
    pub y: f32,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Vector3D {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Vector4D {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

impl Vector2D {
    pub const ZERO: Vector2D = Vector2D { x: 0.0, y: 0.0 };

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
    pub const ZERO: Vector4D = Vector4D { x: 0.0, y: 0.0, z: 0.0, w: 0.0};

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

// trait implementations for Vector2D
impl Add for Vector2D {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Add<f32> for Vector2D {
    type Output = Self;

    fn add(self, other: f32) -> Self {
        Self {
            x: self.x + other,
            y: self.y + other,
        }
    }
}

impl Add<i32> for Vector2D {
    type Output = Self;

    fn add(self, other: i32) -> Self {
        let other = other as f32;
        Self {
            x: self.x + other,
            y: self.y + other,
        }
    }
}

impl Sub for Vector2D {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl Sub<f32> for Vector2D {
    type Output = Self;

    fn sub(self, other: f32) -> Self {
        Self {
            x: self.x + other,
            y: self.y + other,
        }
    }
}

impl Sub<i32> for Vector2D {
    type Output = Self;

    fn sub(self, other: i32) -> Self {
        let other = other as f32;
        Self {
            x: self.x + other,
            y: self.y + other,
        }
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

impl Div for Vector2D {
    type Output = Vector2D;

    fn div(self, rhs: Vector2D) -> Self::Output {
        return Vector2D::new(self.x/rhs.x, self.y/rhs.y);
    }
}

impl Div<f32> for Vector2D {
    type Output = Vector2D;

    fn div(self, rhs: f32) -> Self::Output {
        return Vector2D::new(self.x/rhs, self.y/rhs);
    }
}

impl AddAssign for Vector2D{
    fn add_assign(&mut self, rhs: Vector2D) -> () {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl AddAssign<f32> for Vector2D{
    fn add_assign(&mut self, rhs: f32) -> () {
        self.x += rhs;
        self.y += rhs;
    }
}

impl SubAssign for Vector2D{
    fn sub_assign(&mut self, rhs: Vector2D) -> () {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl SubAssign<f32> for Vector2D{
    fn sub_assign(&mut self, rhs: f32) -> () {
        self.x -= rhs;
        self.y -= rhs;
    }
}

impl MulAssign for Vector2D{
    fn mul_assign(&mut self, rhs: Vector2D) -> () {
        self.x *= rhs.x;
        self.y *= rhs.y;
    }
}

impl MulAssign<f32> for Vector2D{
    fn mul_assign(&mut self, rhs: f32) -> () {
        self.x *= rhs;
        self.y *= rhs;
    }
}

impl DivAssign for Vector2D{
    fn div_assign(&mut self, rhs: Vector2D) -> () {
        self.x /= rhs.x;
        self.y /= rhs.y;
    }
}

impl DivAssign<f32> for Vector2D{
    fn div_assign(&mut self, rhs: f32) -> () {
        self.x /= rhs;
        self.y /= rhs;
    }
}

// trait implementations for Vector3D
impl Add for Vector3D {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Add<f32> for Vector3D {
    type Output = Self;

    fn add(self, other: f32) -> Self {
        Self {
            x: self.x + other,
            y: self.y + other,
            z: self.z + other,
        }
    }
}

impl Add<i32> for Vector3D {
    type Output = Self;

    fn add(self, other: i32) -> Self {
        let other = other as f32;
        Self {
            x: self.x + other,
            y: self.y + other,
            z: self.z + other,
        }
    }
}

impl Sub for Vector3D {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl Sub<f32> for Vector3D {
    type Output = Self;

    fn sub(self, other: f32) -> Self {
        Self {
            x: self.x + other,
            y: self.y + other,
            z: self.z + other,
        }
    }
}

impl Sub<i32> for Vector3D {
    type Output = Self;

    fn sub(self, other: i32) -> Self {
        let other = other as f32;
        Self {
            x: self.x + other,
            y: self.y + other,
            z: self.z + other,
        }
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

impl Div for Vector3D {
    type Output = Vector3D;

    fn div(self, rhs: Vector3D) -> Self::Output {
        return Vector3D::new(self.x/rhs.x, self.y/rhs.y, self.z/rhs.z);
    }
}

impl Div<f32> for Vector3D {
    type Output = Vector3D;

    fn div(self, rhs: f32) -> Self::Output {
        return Vector3D::new(self.x/rhs, self.y/rhs, self.z/rhs);
    }
}

impl AddAssign for Vector3D{
    fn add_assign(&mut self, rhs: Vector3D) -> () {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl AddAssign<f32> for Vector3D{
    fn add_assign(&mut self, rhs: f32) -> () {
        self.x += rhs;
        self.y += rhs;
        self.z += rhs;
    }
}

impl SubAssign for Vector3D{
    fn sub_assign(&mut self, rhs: Vector3D) -> () {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}

impl SubAssign<f32> for Vector3D{
    fn sub_assign(&mut self, rhs: f32) -> () {
        self.x -= rhs;
        self.y -= rhs;
        self.z -= rhs;
    }
}

impl MulAssign for Vector3D{
    fn mul_assign(&mut self, rhs: Vector3D) -> () {
        self.x *= rhs.x;
        self.y *= rhs.y;
        self.z *= rhs.z;
    }
}

impl MulAssign<f32> for Vector3D{
    fn mul_assign(&mut self, rhs: f32) -> () {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
    }
}

impl DivAssign for Vector3D{
    fn div_assign(&mut self, rhs: Vector3D) -> () {
        self.x /= rhs.x;
        self.y /= rhs.y;
        self.z /= rhs.z;
    }
}

impl DivAssign<f32> for Vector3D{
    fn div_assign(&mut self, rhs: f32) -> () {
        self.x /= rhs;
        self.y /= rhs;
        self.z /= rhs;
    }
}

// Trait implementations for Vector4D

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

impl Add<f32> for Vector4D {
    type Output = Self;

    fn add(self, other: f32) -> Self {
        Self {
            x: self.x + other,
            y: self.y + other,
            z: self.z + other,
            w: self.w + other,
        }
    }
}

impl Add<i32> for Vector4D {
    type Output = Self;

    fn add(self, other: i32) -> Self {
        let other = other as f32;
        Self {
            x: self.x + other,
            y: self.y + other,
            z: self.z + other,
            w: self.w + other,
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

impl Sub<f32> for Vector4D {
    type Output = Self;

    fn sub(self, other: f32) -> Self {
        Self {
            x: self.x + other,
            y: self.y + other,
            z: self.z + other,
            w: self.w + other,
        }
    }
}

impl Sub<i32> for Vector4D {
    type Output = Self;

    fn sub(self, other: i32) -> Self {
        let other = other as f32;
        Self {
            x: self.x + other,
            y: self.y + other,
            z: self.z + other,
            w: self.w + other,
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

impl AddAssign for Vector4D{
    fn add_assign(&mut self, rhs: Vector4D) -> () {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
        self.w += rhs.w;
    }
}

impl AddAssign<f32> for Vector4D{
    fn add_assign(&mut self, rhs: f32) -> () {
        self.x += rhs;
        self.y += rhs;
        self.z += rhs;
        self.w += rhs;
    }
}

impl SubAssign for Vector4D{
    fn sub_assign(&mut self, rhs: Vector4D) -> () {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
        self.w -= rhs.w;
    }
}

impl SubAssign<f32> for Vector4D{
    fn sub_assign(&mut self, rhs: f32) -> () {
        self.x -= rhs;
        self.y -= rhs;
        self.z -= rhs;
        self.w -= rhs;
    }
}

impl MulAssign for Vector4D{
    fn mul_assign(&mut self, rhs: Vector4D) -> () {
        self.x *= rhs.x;
        self.y *= rhs.y;
        self.z *= rhs.z;
        self.w *= rhs.w;
    }
}

impl MulAssign<f32> for Vector4D{
    fn mul_assign(&mut self, rhs: f32) -> () {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
        self.w *= rhs;
    }
}

impl DivAssign for Vector4D{
    fn div_assign(&mut self, rhs: Vector4D) -> () {
        self.x /= rhs.x;
        self.y /= rhs.y;
        self.z /= rhs.z;
        self.w /= rhs.w;
    }
}

impl DivAssign<f32> for Vector4D{
    fn div_assign(&mut self, rhs: f32) -> () {
        self.x /= rhs;
        self.y /= rhs;
        self.z /= rhs;
        self.w /= rhs;
    }
}
