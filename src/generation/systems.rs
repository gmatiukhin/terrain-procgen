use bevy::{
    prelude::*,
    render::mesh::{self, PrimitiveTopology},
};
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
    mut chunks_query: Query<(Entity, &mut TerrainChunk), Added<TerrainChunk>>,
    config: Res<TerrainGeneratorConfig>,
) {
    for (entity, mut chunk) in chunks_query.iter_mut() {
        log::info!("Generating new chunk '{entity:?}' at {}", chunk.position);
        generate_chunk(&mut chunk, &config);
    }
}

pub(super) fn regenerate_chunks(
    mut chunks_query: Query<(Entity, &mut TerrainChunk), With<TerrainChunk>>,
    mut regenerate_event: EventReader<RegenerateTerrainEvent>,
    config: Res<TerrainGeneratorConfig>,
) {
    if !regenerate_event.is_empty() {
        for (entity, mut chunk) in chunks_query.iter_mut() {
            log::info!("Regenerating chunk '{entity:?}' at {}", chunk.position);
            generate_chunk(&mut chunk, &config);
        }
        regenerate_event.clear();
    }
}

fn generate_chunk(chunk: &mut TerrainChunk, config: &Res<TerrainGeneratorConfig>) {
    let ground_function = |pos: Vec3| -> f32 { config.isolevel - pos.y };

    for point in chunk.points.iter_mut() {
        point.value = ground_function(point.position);
    }

    // Go throught all of the points except for the final in each dimension
    // This way we get only 0th point of every cube in chunk
    // TODO: show logs
    for z in 0..chunk.size.z {
        for y in 0..chunk.size.y {
            for x in 0..chunk.size.x {
                let cube_zero_corner_idx =
                    utils::from_3D_to_1D_index((x, y, z).into(), chunk.size) as usize;
                let size_x = chunk.point_size.x as usize;
                let size_y = chunk.point_size.y as usize;

                // Get all points of a cube
                let cube = [
                    // Bottom
                    chunk.points[cube_zero_corner_idx],
                    chunk.points[cube_zero_corner_idx + 1],
                    chunk.points[cube_zero_corner_idx + size_x * size_y + 1],
                    chunk.points[cube_zero_corner_idx + size_x * size_y],
                    // Top
                    chunk.points[size_x + cube_zero_corner_idx],
                    chunk.points[size_x + cube_zero_corner_idx + 1],
                    chunk.points[size_x + cube_zero_corner_idx + size_x * size_y + 1],
                    chunk.points[size_x + cube_zero_corner_idx + size_x * size_y],
                ];

                // Compute cube configuration index by setting bits of the points that are below
                // the isosurface to 1
                let mut cube_index = 0;
                for (i, point) in cube.iter().enumerate() {
                    if point.value < config.isolevel {
                        cube_index |= usize::pow(2, i as u32);
                    }
                }

                let edge_configuration = tables::EDGE_CONFIGURATION[cube_index];
                if edge_configuration == 0 {
                    // This cube is either fully below or fully above the isosurface
                    // no triangles need to be made
                    break;
                }

                // Calculate triangle vertex position for each edge of the cube with at least one
                // vertex below the isosurface
                let mut vertlist = [None; 12];
                for edge in 0..12 {
                    if edge_configuration & u16::pow(2, edge) != 0 {
                        let (p1_idx, p2_idx) = tables::EDGE_VERTICES[edge as usize];
                        vertlist[edge as usize] = Some(utils::vertex_lerp(
                            config.isolevel,
                            cube[p1_idx as usize],
                            cube[p2_idx as usize],
                        ));
                    }
                }

                // Get triangulation order from the table and
                let trianglulation_order = tables::TRIANGULATION_SEQUENCE[cube_index];
                let mut i = 0;
                struct Triangle {
                    points: [Vec3; 3],
                }
                let mut triangles = Vec::new();
                while trianglulation_order[i] != -1 {
                    triangles.push(Triangle {
                        points: [
                            vertlist[trianglulation_order[i] as usize].unwrap(),
                            vertlist[trianglulation_order[i + 1] as usize].unwrap(),
                            vertlist[trianglulation_order[i + 2] as usize].unwrap(),
                        ],
                    });
                    i += 3;
                }
            }
        }
    }
}
