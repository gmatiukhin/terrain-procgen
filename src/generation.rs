use bevy::prelude::*;

mod systems;
mod tables;
mod utils;

pub struct MarchingCubesTerrain;

impl Plugin for MarchingCubesTerrain {
    fn build(&self, app: &mut App) {
        use systems::*;
        app.init_resource::<TerrainGeneratorConfig>()
            .insert_resource(Msaa::Sample4)
            .add_event::<GenerateTerrainEvent>()
            .add_systems(Startup, light)
            .add_systems(
                Update,
                (
                    create_chunks.run_if(on_event::<GenerateTerrainEvent>()),
                    appply_ground_function
                        .run_if(IntoSystem::into_system(
                            |new_chunks: Query<Entity, Added<TerrainChunk>>| !new_chunks.is_empty(),
                        ))
                        .after(create_chunks),
                    generate_chunks
                        .run_if(IntoSystem::into_system(
                            |changed_chunks: Query<Entity, Changed<TerrainChunk>>| {
                                !changed_chunks.is_empty()
                            },
                        ))
                        .after(appply_ground_function),
                ),
            )
            .add_systems(Update, (draw_bounding_box, draw_mesh_normals));
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
    pub show_gizmos: bool,
}

impl Default for TerrainGeneratorConfig {
    fn default() -> Self {
        Self {
            cube_edge_length: 1f32,
            chunks_amount: UVec3::new(4, 4, 4),
            chunk_size: UVec3::new(4, 4, 4),
            isolevel: 0f32,
            show_gizmos: false,
        }
    }
}

#[derive(Event, Debug)]
pub struct GenerateTerrainEvent;

#[derive(Debug, Clone, Copy)]
struct Point {
    /// Absolute position in the world
    position: Vec3,
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
