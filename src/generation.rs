use bevy::prelude::*;

#[derive(Resource, Debug)]
pub struct TerrainGeneratorConfig {
    pub cube_size: f32,
    pub chunk_size: UVec3,
    pub chunks: UVec3,
    pub show_debug_points: bool,
}

#[derive(Debug)]
pub struct GenerateTerrainEvent;

#[derive(Component)]
pub struct TerrainChunk {
    position: Vec3,
}

impl Default for TerrainGeneratorConfig {
    fn default() -> Self {
        Self {
            cube_size: 1f32,
            chunk_size: UVec3::ONE,
            chunks: UVec3::ONE,
            show_debug_points: false,
        }
    }
}
