use bevy::{pbr::wireframe::Wireframe, prelude::*, render::mesh::PrimitiveTopology};
use log;

use super::{utils::*, *};

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

pub(super) fn despawn_terrain_mesh(
    mut commands: Commands,
    generate_event: EventReader<GenerateNewTerrainEvent>,
    regenerate_event: EventReader<RegenerateTerrainEvent>,
    meshes: Query<Entity, With<Handle<Mesh>>>,
) {
    if !generate_event.is_empty() || !regenerate_event.is_empty() {
        log::info!("Despawning terrain");
        for mesh in meshes.iter() {
            commands.entity(mesh).despawn();
        }
    }
}

pub(super) fn generate_new_chunks(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut chunks_query: Query<(Entity, &mut TerrainChunk), Added<TerrainChunk>>,
    config: Res<TerrainGeneratorConfig>,
) {
    for (entity, mut chunk) in chunks_query.iter_mut() {
        log::info!("Generating new chunk '{entity:?}' at {}", chunk.position);
        generate_chunk(
            &mut commands,
            &mut meshes,
            &mut materials,
            &mut chunk,
            &config,
        );
    }
}

pub(super) fn regenerate_chunks(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut chunks_query: Query<(Entity, &mut TerrainChunk), With<TerrainChunk>>,
    mut regenerate_event: EventReader<RegenerateTerrainEvent>,
    config: Res<TerrainGeneratorConfig>,
) {
    if !regenerate_event.is_empty() {
        for (entity, mut chunk) in chunks_query.iter_mut() {
            log::info!("Regenerating chunk '{entity:?}' at {}", chunk.position);
            generate_chunk(
                &mut commands,
                &mut meshes,
                &mut materials,
                &mut chunk,
                &config,
            );
        }
        regenerate_event.clear();
    }
}

fn generate_chunk(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    chunk: &mut TerrainChunk,
    config: &Res<TerrainGeneratorConfig>,
) {
    let ground_function = |pos: Vec3| -> f32 {
        (pos.x.powi(2) + pos.y.powi(2) + pos.z.powi(2)) - config.isolevel.powi(2)
    };

    for point in chunk.points.iter_mut() {
        point.value = ground_function(point.position);
    }

    // Go throught all of the points except for the final in each dimension
    // This way we get only 0th point of every cube in chunk
    // TODO: show logs
    for z in 0..chunk.size.z {
        for y in 0..chunk.size.y {
            for x in 0..chunk.size.x {
                let points = &chunk.points;
                let point_size = chunk.point_size;
                let cube = [
                    // Bottom
                    points[from_3D_to_1D_index(UVec3::new(x, y, z), point_size) as usize],
                    points[from_3D_to_1D_index(UVec3::new(x + 1, y, z), point_size) as usize],
                    points[from_3D_to_1D_index(UVec3::new(x + 1, y, z + 1), point_size) as usize],
                    points[from_3D_to_1D_index(UVec3::new(x, y, z + 1), point_size) as usize],
                    // Top
                    points[from_3D_to_1D_index(UVec3::new(x, y + 1, z), point_size) as usize],
                    points[from_3D_to_1D_index(UVec3::new(x + 1, y + 1, z), point_size) as usize],
                    points
                        [from_3D_to_1D_index(UVec3::new(x + 1, y + 1, z + 1), point_size) as usize],
                    points[from_3D_to_1D_index(UVec3::new(x, y + 1, z + 1), point_size) as usize],
                ];

                // Compute cube configuration index by setting bits of the points that are below
                // the isosurface to 1
                let mut cube_index = 0;
                for (i, point) in cube.iter().enumerate() {
                    if point.value < config.isolevel {
                        cube_index |= 1 << i;
                    }
                }

                // Get intersecred edges for the cube configuration,
                // calculate points along them and add to `vertices`
                let intersected_edges = tables::INTERSECTED_EDGES[cube_index];
                let mut vertices = Vec::new();
                for edge in intersected_edges {
                    if edge == -1 {
                        break;
                    }
                    let (p1_idx, p2_idx) = tables::EDGE_VERTICES[edge as usize];
                    vertices.push(utils::vertex_lerp(
                        config.isolevel,
                        cube[p1_idx as usize],
                        cube[p2_idx as usize],
                    ));
                }

                // Create mesh
                let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);

                mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, vertices);
                mesh.compute_flat_normals();

                commands.spawn((
                    PbrBundle {
                        mesh: meshes.add(mesh),
                        material: materials.add(StandardMaterial {
                            base_color: Color::rgb(0.3, 0.5, 0.3),
                            double_sided: true,
                            cull_mode: None,
                            ..Default::default()
                        }),
                        ..Default::default()
                    },
                    Wireframe,
                ));
            }
        }
    }
}

pub(super) fn gismos(mut gizmos: Gizmos, config: Res<TerrainGeneratorConfig>) {
    if config.show_gizmo {
        for chx in 0..config.chunks_amount.x {
            for chy in 0..config.chunks_amount.y {
                for chz in 0..config.chunks_amount.z {
                    for x in 0..=config.chunk_size.x {
                        for y in 0..=config.chunk_size.y {
                            for z in 0..=config.chunk_size.z {
                                gizmos.sphere(
                                    Vec3::new(
                                        (chx * config.chunk_size.x + x) as f32,
                                        (chy * config.chunk_size.y + y) as f32,
                                        (chz * config.chunk_size.z + z) as f32,
                                    ) * config.cube_edge_length,
                                    Quat::IDENTITY,
                                    0.02,
                                    Color::BLACK,
                                );
                            }
                        }
                    }
                }
            }
        }
    }
}
