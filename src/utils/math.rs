use std::ops::Add;
use rand::prelude::*;

use crate::utils::{Vector2D, Vector3D, Vector4D};

pub fn distance(a: f32, b: f32) -> f32{
    return f32::sqrt(a*a + b*b);
}

pub fn lerp(min: f32, max: f32, t: f32) -> f32{
    return min + (max - min) * t;
}

// generates a random value T between n1: T and n2: T
pub fn rand_betw
<
    T: std::cmp::PartialOrd +
    rand::distributions::uniform::SampleUniform,
>
(
    n1: T, 
    n2: T
) -> T {
    let mut r = thread_rng();
    r.gen_range(n1..n2)
}

pub fn rand_vec2() -> Vector2D {
    Vector2D::new(rand_betw(0.0, 1.0), rand_betw(0.0, 1.0))
}

pub fn rand_vec3() -> Vector3D {
    Vector3D::new(rand_betw(0.0, 1.0), rand_betw(0.0, 1.0), rand_betw(0.0, 1.0))
}

pub fn rand_vec4() -> Vector4D {
    Vector4D::new(rand_betw(0.0, 1.0), rand_betw(0.0, 1.0), rand_betw(0.0, 1.0), rand_betw(0.0, 1.0))
}

struct SecondOrderDynamics {

}
/* 
impl Add<f32> for i32 {
    type Output = i32;

    fn add(self, rhs: f32) -> Self::Output {
        return self as f32 + rhs;
    }
}
*/