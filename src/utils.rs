use bevy::prelude::UVec3;

#[allow(non_snake_case)]
pub fn from_1D_to_3D_index(idx: u32, dimensions: UVec3) -> UVec3 {
    let x = idx % dimensions.x;
    let y = (idx / dimensions.x) % dimensions.y;
    let z = idx / (dimensions.x * dimensions.y);
    UVec3::new(x, y, z)
}

#[allow(non_snake_case)]
pub fn from_3D_to_1D_index(idx: UVec3, dimensions: UVec3) -> u32 {
    idx.x + idx.y * dimensions.x + idx.z * dimensions.x * dimensions.y
}
