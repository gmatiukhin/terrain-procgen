use bevy::prelude::{UVec3, Vec3};

#[allow(non_snake_case)]
pub(super) fn _from_1D_to_3D_index(idx: u32, dimensions: UVec3) -> UVec3 {
    let x = idx % dimensions.x;
    let y = (idx / dimensions.x) % dimensions.y;
    let z = idx / (dimensions.x * dimensions.y);
    UVec3::new(x, y, z)
}

#[allow(non_snake_case)]
pub(super) fn from_3D_to_1D_index(idx: UVec3, dimensions: UVec3) -> u32 {
    idx.x + idx.y * dimensions.x + idx.z * dimensions.x * dimensions.y
}

pub(super) fn vertex_lerp(isolevel: f32, p1: super::Point, p2: super::Point) -> Vec3 {
    let t = (isolevel - p1.value) / (p2.value - p1.value);
    p1.position + t * (p2.position - p1.position)
}
