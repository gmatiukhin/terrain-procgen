use bevy::{
    prelude::*,
    render::mesh::{Indices, PrimitiveTopology},
};

use super::{tables::*, utils::*, *};

pub(super) fn light(mut commands: Commands) {
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            illuminance: 15000f32,
            shadows_enabled: true,
            ..Default::default()
        },
        transform: Transform {
            translation: Vec3::new(100f32, 100f32, 100f32),
            rotation: Quat::IDENTITY,
            scale: Vec3::ONE,
        },
        visibility: Visibility::Visible,
        ..Default::default()
    });
}

pub(super) fn create_chunks(
    mut commands: Commands,
    existing_chunks: Query<Option<Entity>, With<TerrainChunk>>,
    config: Res<TerrainGeneratorConfig>,
) {
    info!("Despawning chunks");
    for existing_chunk_entity in existing_chunks.iter().flatten() {
        debug!("Despawning chunk '{existing_chunk_entity:?}'");
        commands.entity(existing_chunk_entity).despawn();
    }

    info!("Generating chunks with config:\n{config:#?}");
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
}

pub(super) fn appply_ground_function(
    mut new_chunks: Query<(Entity, &mut TerrainChunk), Added<TerrainChunk>>,
) {
    info!("Applying ground function");
    for (entity, mut chunk) in new_chunks.iter_mut() {
        debug!(
            "Applying ground function to chunk '{entity:?}' at {}",
            chunk.position
        );
        let ground_function = |pos: Vec3| -> f32 { pos.y };

        for point in chunk.points.iter_mut() {
            point.value = ground_function(point.position);
        }
    }
}

pub(super) fn generate_chunks(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    changed_chunks: Query<(Entity, &TerrainChunk), Changed<TerrainChunk>>,
    config: Res<TerrainGeneratorConfig>,
) {
    info!("Generating meshes");
    for (entity, chunk) in changed_chunks.iter() {
        debug!(
            "Generating mesh for chunk '{entity:?}' at {}",
            chunk.position
        );

        // Go throught all of the points except for the final in each dimension
        // This way we get only 0th point of every cube in chunk
        let mut vertices = vec![];
        let mut indices = vec![];
        for z in 0..chunk.size.z {
            for y in 0..chunk.size.y {
                for x in 0..chunk.size.x {
                    let points = &chunk.points;
                    let point_size = chunk.point_size;
                    #[rustfmt::skip]
                    let cube = [
                        // Bottom
                        points[from_3D_to_1D_index(UVec3::new(x, y, z), point_size) as usize],
                        points[from_3D_to_1D_index(UVec3::new(x + 1, y, z), point_size) as usize],
                        points[from_3D_to_1D_index(UVec3::new(x + 1, y, z + 1), point_size) as usize],
                        points[from_3D_to_1D_index(UVec3::new(x, y, z + 1), point_size) as usize],
                        // Top
                        points[from_3D_to_1D_index(UVec3::new(x, y + 1, z), point_size) as usize],
                        points[from_3D_to_1D_index(UVec3::new(x + 1, y + 1, z), point_size) as usize],
                        points[from_3D_to_1D_index(UVec3::new(x + 1, y + 1, z + 1), point_size) as usize],
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
                    // calculate points along them
                    let intersected_edges = INTERSECTED_EDGES[cube_index];
                    for edge in intersected_edges {
                        if edge == -1 {
                            break;
                        }
                        let (p1_idx, p2_idx) = EDGE_VERTICES[edge as usize];
                        let vertex = utils::vertex_lerp(
                            config.isolevel,
                            cube[p1_idx as usize],
                            cube[p2_idx as usize],
                        );
                        if let Some(idx) = vertices.iter().position(|el| *el == vertex) {
                            indices.push(idx as u16);
                        } else {
                            vertices.push(vertex);
                            indices.push((vertices.len() - 1) as u16);
                        }
                    }
                }
            }
        }

        // Compute normals
        let mut normals = vec![Vec3::ZERO; vertices.len()];
        for chunk in indices.chunks_exact(3) {
            let idx_a = chunk[0] as usize;
            let idx_b = chunk[1] as usize;
            let idx_c = chunk[2] as usize;

            let vertex_a = vertices[idx_a];
            let vertex_b = vertices[idx_b];
            let vertex_c = vertices[idx_c];

            let edge_ab = vertex_b - vertex_a;
            let edge_ac = vertex_c - vertex_a;

            let wheighted_normal = edge_ab.cross(edge_ac);

            normals[idx_a] += wheighted_normal;
            normals[idx_b] += wheighted_normal;
            normals[idx_c] += wheighted_normal;
        }

        for n in normals.iter_mut() {
            *n = n.normalize();
        }

        let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, vertices);
        mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
        mesh.set_indices(Some(Indices::U16(indices)));

        debug!("Inserting mesh into `{entity:?}`");
        commands.entity(entity).insert(PbrBundle {
            mesh: meshes.add(mesh),
            material: materials.add(StandardMaterial {
                base_color: Color::rgb(0.3, 0.5, 0.3),
                double_sided: true,
                // cull_mode: None,
                perceptual_roughness: 1f32,
                metallic: 0f32,
                reflectance: 0f32,
                ..Default::default()
            }),
            ..Default::default()
        });
    }
}

#[rustfmt::skip]
pub(super) fn draw_bounding_box(mut gizmos: Gizmos, config: Res<TerrainGeneratorConfig>) {
    if !config.show_gizmos {
        return;
    }

    let start = Vec3::new(0f32, 0f32, 0f32);
    let size_x = (config.chunks_amount.x * config.chunk_size.x) as f32 * config.cube_edge_length;
    let size_y = (config.chunks_amount.y * config.chunk_size.y) as f32 * config.cube_edge_length;
    let size_z = (config.chunks_amount.z * config.chunk_size.z) as f32 * config.cube_edge_length;

    gizmos.line(start, Vec3::new(size_x, 0f32, 0f32), Color::RED);
    gizmos.line(start, Vec3::new(0f32, size_y, 0f32), Color::GREEN);
    gizmos.line(start, Vec3::new(0f32, 0f32, size_z), Color::BLUE);

    let end = Vec3::new(size_x, size_y, size_z);

    gizmos.line(Vec3::new(0f32, size_y, size_z), end, Color::BLACK);
    gizmos.line(Vec3::new(size_x, 0f32, size_z), end, Color::BLACK);
    gizmos.line(Vec3::new(size_x, size_y, 0f32), end, Color::BLACK);

    gizmos.line(Vec3::new(0f32, size_y, size_z), Vec3::new(0f32, size_y, 0f32), Color::BLACK);
    gizmos.line(Vec3::new(0f32, size_y, size_z), Vec3::new(0f32, 0f32, size_z), Color::BLACK);

    gizmos.line(Vec3::new(size_x, 0f32, size_z), Vec3::new(size_x, 0f32, 0f32), Color::BLACK);
    gizmos.line(Vec3::new(size_x, 0f32, size_z), Vec3::new(0f32, 0f32, size_z), Color::BLACK);

    gizmos.line(Vec3::new(size_x, size_y, 0f32), Vec3::new(size_x, 0f32, 0f32), Color::BLACK);
    gizmos.line(Vec3::new(size_x, size_y, 0f32), Vec3::new(0f32, size_y, 0f32), Color::BLACK);
}

pub(super) fn draw_mesh_normals(
    mut gizmos: Gizmos,
    config: Res<TerrainGeneratorConfig>,
    meshe_handles: Query<&Handle<Mesh>, With<TerrainChunk>>,
    meshes: ResMut<Assets<Mesh>>,
) {
    if !config.show_gizmos {
        return;
    }

    for handle in meshe_handles.iter() {
        if let Some(mesh) = meshes.get(handle) {
            let vertices = mesh
                .attribute(Mesh::ATTRIBUTE_POSITION)
                .unwrap()
                .as_float3()
                .unwrap();
            let normals = mesh
                .attribute(Mesh::ATTRIBUTE_NORMAL)
                .unwrap()
                .as_float3()
                .unwrap();

            for (v, n) in vertices.iter().zip(normals) {
                let v = Vec3::from_slice(v);
                let n = Vec3::from_slice(n);
                gizmos.line(v, v + n, Color::ORANGE);
            }
        }
    }
}
