use std::ops::{Mul};

use crate::utils::Vector4D;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Matrix4x4 {
    // no simd yet :(
    pub m: [[f32; 4]; 4],
}

impl Matrix4x4 {
    pub fn identity() -> Self {
        Self {
            m: [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ],
        }
    }

    pub fn perspective(fov: f32, aspect_ratio: f32, near: f32, far: f32) -> Self {
        let f = 1.0 / (fov / 2.0).tan();
        let nf = 1.0 / (near - far);

        Self {
            m: [
                [f / aspect_ratio, 0.0, 0.0, 0.0],
                [0.0, f, 0.0, 0.0],
                [0.0, 0.0, (far + near) * nf, -1.0],
                [0.0, 0.0, (2.0 * far * near) * nf, 0.0],
            ],
        }
    }

    pub fn mul(self, other: Self) -> Self {
        let mut result = Self { m: [[0.0; 4]; 4] };

        for i in 0..4 {
            for j in 0..4 {
                result.m[i][j] = self.m[i][0] * other.m[0][j]
                    + self.m[i][1] * other.m[1][j]
                    + self.m[i][2] * other.m[2][j]
                    + self.m[i][3] * other.m[3][j];
            }
        }

        result
    }

    pub fn mul_vector(self, v: Vector4D) -> Vector4D {
        Vector4D {
            x: self.m[0][0] * v.x + self.m[0][1] * v.y + self.m[0][2] * v.z + self.m[0][3] * v.w,
            y: self.m[1][0] * v.x + self.m[1][1] * v.y + self.m[1][2] * v.z + self.m[1][3] * v.w,
            z: self.m[2][0] * v.x + self.m[2][1] * v.y + self.m[2][2] * v.z + self.m[2][3] * v.w,
            w: self.m[3][0] * v.x + self.m[3][1] * v.y + self.m[3][2] * v.z + self.m[3][3] * v.w,
        }
    }
}

impl Mul for Matrix4x4 {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        self.mul(other)
    }
}

impl Mul<Vector4D> for Matrix4x4 {
    type Output = Vector4D;

    fn mul(self, v: Vector4D) -> Vector4D {
        self.mul_vector(v)
    }
}
