use bevy::prelude::*;

mod systems;
mod tables;
mod utils;

pub struct MarchingCubesTerrain;

impl Plugin for MarchingCubesTerrain {
    fn build(&self, app: &mut App) {
        app.init_resource::<TerrainGeneratorConfig>()
            .add_event::<GenerateNewTerrainEvent>()
            .add_event::<RegenerateTerrainEvent>()
            .add_systems(Update, systems::create_chunks)
            .add_systems(Update, systems::generate_new_chunks)
            .add_systems(Update, systems::regenerate_chunks);
    }
}

// TODO: split config when it becomes too big
// also split ui into sections to make modifications to parts of generation algorithm possible
// without modifying everything
#[derive(Resource, Debug, Clone, Copy, PartialEq)]
pub struct TerrainGeneratorConfig {
    pub chunks_amount: UVec3,
    pub chunk_size: UVec3,
    pub cube_edge_length: f32,
    pub isolevel: f32,
    pub show_debug_points: bool,
}

impl Default for TerrainGeneratorConfig {
    fn default() -> Self {
        Self {
            cube_edge_length: 1f32,
            chunks_amount: UVec3::ONE,
            chunk_size: UVec3::ONE,
            isolevel: 0f32,
            show_debug_points: false,
        }
    }
}

#[derive(Event, Debug)]
pub struct GenerateNewTerrainEvent;

#[derive(Event, Debug)]
pub struct RegenerateTerrainEvent;

#[derive(Debug, Clone, Copy)]
struct Point {
    /// Absolute position in the world
    position: Vec3,
    // NOTE: bool for now, f32 later
    /// Is this point empty
    value: f32,
}

#[derive(Component, Debug)]
struct TerrainChunk {
    /// Chunk's position in the world,
    /// the same as the position of its first point
    position: Vec3,
    /// Chunk's size measuring in cubes
    size: UVec3,
    /// Chunk's size measuring in points, each direction is bigger by one
    point_size: UVec3,
    /// 1D array of points
    points: Vec<Point>,
}

impl TerrainChunk {
    fn new(position: Vec3, size: UVec3, cube_edge_size: f32) -> Self {
        // Add one to each dimension because we specify chunk size in cubes but we need last points
        let point_size_x = 1 + size.x as usize;
        let point_size_y = 1 + size.y as usize;
        let point_size_z = 1 + size.z as usize;

        let mut points = Vec::with_capacity(point_size_x * point_size_y * point_size_z);
        for z in 0..point_size_z {
            for y in 0..point_size_y {
                for x in 0..point_size_x {
                    points.push(Point {
                        position: Vec3::new(
                            x as f32 * cube_edge_size,
                            y as f32 * cube_edge_size,
                            z as f32 * cube_edge_size,
                        ) + position,
                        value: 0f32,
                    });
                }
            }
        }

        Self {
            position,
            size,
            point_size: UVec3::new(
                point_size_x as u32,
                point_size_y as u32,
                point_size_z as u32,
            ),
            points,
        }
    }
}
