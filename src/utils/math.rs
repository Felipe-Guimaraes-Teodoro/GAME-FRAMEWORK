use std::ops::Add;

pub fn distance(a: f32, b: f32) -> f32{
    return f32::sqrt(a*a + b*b);
}

pub fn lerp(min: f32, max: f32, t: f32) -> f32{
    return min + (max - min) * t;
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