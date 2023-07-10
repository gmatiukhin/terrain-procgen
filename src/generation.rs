use bevy::prelude::*;

#[derive(Resource)]
pub struct TerrainGeneratorConfig {
    cube_size: f32,
    chunk_size: IVec3,
    chunks: IVec3,
    show_debug_points: bool,
}

#[derive(Event, Debug)]
pub struct GenerateTerrainEvent;

#[derive(Component)]
pub struct TerrainChunk {
    position: Vec3,
}

impl Default for TerrainGeneratorConfig {
    fn default() -> Self {
        Self {
            cube_size: 1f32,
            chunk_size: IVec3 { x: 1, y: 1, z: 1 },
            chunks: IVec3 { x: 1, y: 1, z: 1 },
            show_debug_points: false,
        }
    }
}
