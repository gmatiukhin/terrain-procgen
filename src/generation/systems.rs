use bevy::prelude::*;
use log;

use super::*;

pub(super) fn create_chunks(
    mut commands: Commands,
    mut gen_event_reader: EventReader<GenerateNewTerrainEvent>,
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
        let cube_size = config.cube_edge_length;
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
                        config.cube_edge_length,
                    );
                    chunks.push(chunk);
                }
            }
        }
        commands.spawn_batch(chunks);
        gen_event_reader.clear();
    }
}

pub(super) fn generate_new_chunks(
    chunks_query: Query<(Entity, &TerrainChunk), Added<TerrainChunk>>,
) {
    for chunk in chunks_query.iter() {
        let (entity, chunk) = chunk;
        log::info!("Generating new chunk '{entity:?}' at {}", chunk.position);
        generate_chunk(chunk);
    }
}

pub(super) fn regenerate_chunks(
    chunks_query: Query<(Entity, &TerrainChunk), With<TerrainChunk>>,
    mut regenerate_event: EventReader<RegenerateTerrainEvent>,
) {
    if !regenerate_event.is_empty() {
        for chunk in chunks_query.iter() {
            let (entity, chunk) = chunk;
            log::info!("Reenerating chunk '{entity:?}' at {}", chunk.position);
            generate_chunk(chunk);
        }
        regenerate_event.clear();
    }
}

fn generate_chunk(chunk: &TerrainChunk) {
    fn ground_function(pos: Vec3) -> bool {
        pos.y > 2f32
    }
    log::debug!("Generating...");
}
