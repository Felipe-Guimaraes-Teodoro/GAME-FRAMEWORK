use crate::glam::vec3;

pub struct RaycastResult {

}

pub struct Raycaster {
    origin: Vec3,
    dir: (f32, f32), // pitch and yaw
    result: RaycastResult,
}
