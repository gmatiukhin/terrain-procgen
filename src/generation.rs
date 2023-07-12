use bevy::prelude::*;

mod systems;

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
    pub show_debug_points: bool,
}

#[derive(Event, Debug)]
pub struct GenerateNewTerrainEvent;

#[derive(Event, Debug)]
pub struct RegenerateTerrainEvent;

#[derive(Component, Debug)]
struct TerrainChunk {
    position: Vec3,
    size: UVec3,
    points: Vec<Point>,
}

#[derive(Debug)]
struct Point {
    /// Local position inside a chunk
    position: Vec3,
    // NOTE: bool for now, f32 later
    value: bool,
}

impl TerrainChunk {
    fn new(position: Vec3, size: UVec3, cube_edge_size: f32) -> Self {
        // Add one to each dimension because we specify chunk size in cubes but we need last points
        let mut points = Vec::with_capacity(((size.x + 1) * (size.y + 1) * (size.z + 1)) as usize);
        for z in 0..=size.z {
            for y in 0..=size.y {
                for x in 0..=size.x {
                    points.push(Point {
                        position: Vec3::new(
                            x as f32 * cube_edge_size,
                            y as f32 * cube_edge_size,
                            z as f32 * cube_edge_size,
                        ),
                        value: false,
                    });
                }
            }
        }

        Self {
            position,
            size,
            points,
        }
    }
}

impl Default for TerrainGeneratorConfig {
    fn default() -> Self {
        Self {
            cube_edge_length: 1f32,
            chunks_amount: UVec3::ONE,
            chunk_size: UVec3::ONE,
            show_debug_points: false,
        }
    }
}
