use bevy::prelude::*;
use log;

pub struct MarchingCubesTerrain;

impl Plugin for MarchingCubesTerrain {
    fn build(&self, app: &mut App) {
        app.init_resource::<TerrainGeneratorConfig>()
            .add_event::<GenerateTerrainEvent>()
            .add_system(generate_chunks)
            .add_system(generate_chunk);
    }
}

// TODO: split config when it becomes too big
// also split ui into sections to make modifications to parts of generation algorithm possible
// without modifying everything
#[derive(Resource, Debug)]
pub struct TerrainGeneratorConfig {
    pub chunks_amount: UVec3,
    pub chunk_size: UVec3,
    pub cube_size: f32,
    pub show_debug_points: bool,
}

#[derive(/*Event,*/ Debug)]
pub struct GenerateTerrainEvent;

#[derive(Component, Debug)]
struct TerrainChunk {
    position: Vec3,
    size: UVec3,
    // NOTE: bool for now, f32 later
    points: Vec<bool>,
}

impl TerrainChunk {
    fn new(position: Vec3, size: UVec3) -> Self {
        // Add one to each dimension because we specify chunk size in cubes but we need last points
        let points = vec![false; ((size.x + 1) * (size.y + 1) * (size.z + 1)) as usize];
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
            cube_size: 1f32,
            chunks_amount: UVec3::ONE,
            chunk_size: UVec3::ONE,
            show_debug_points: false,
        }
    }
}

fn generate_chunks(
    mut commands: Commands,
    mut gen_event_reader: EventReader<GenerateTerrainEvent>,
    config: Res<TerrainGeneratorConfig>,
    existing_chunks_query: Query<Option<Entity>, With<TerrainChunk>>,
) {
    // We only care that there is an event
    if !gen_event_reader.is_empty() {
        log::info!("Generating terrain with config:\n{config:#?}");

        // Despawn all existing chunks
        for existing_chunk_entity in existing_chunks_query.iter().flatten() {
            log::info!("Despawning chunk '{existing_chunk_entity:?}'");
            commands.entity(existing_chunk_entity).despawn();
        }

        // Generate chunks
        let chunks_amount = config.chunks_amount;
        let chunk_size = config.chunk_size;
        let cube_size = config.cube_size;
        let mut chunks = vec![];
        for z in 0..chunks_amount.z {
            for y in 0..chunks_amount.y {
                for x in 0..chunks_amount.x {
                    let chunk = TerrainChunk::new(
                        Vec3 {
                            x: (x * chunk_size.x) as f32 * cube_size,
                            y: (y * chunk_size.y) as f32 * cube_size,
                            z: (z * chunk_size.z) as f32 * cube_size,
                        },
                        config.chunk_size,
                    );
                    chunks.push(chunk);
                }
            }
        }
        commands.spawn_batch(chunks);
        gen_event_reader.clear();
    }
}

fn generate_chunk(chunks_query: Query<(Entity, &TerrainChunk), Added<TerrainChunk>>) {
    for chunk in chunks_query.iter() {
        let (entity, chunk) = chunk;
        log::info!("Generating chunk '{entity:?}' at {}", chunk.position);
    }
}
