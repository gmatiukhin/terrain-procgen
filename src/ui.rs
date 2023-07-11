use bevy::prelude::{EventWriter, ResMut, Resource, UVec3};
use bevy_egui::{
    egui::{DragValue, Grid, Slider, TopBottomPanel, Window},
    EguiContexts,
};
use terrain_procgen::generation::{GenerateTerrainEvent, TerrainGeneratorConfig};

#[derive(Resource, Debug, Default)]
pub struct UIState {
    is_gen_window_expanded: bool,
}

pub fn ui_system(
    mut contexts: EguiContexts,
    mut gen_config: ResMut<TerrainGeneratorConfig>,
    mut ui_state: ResMut<UIState>,
    mut gen_event_writer: EventWriter<GenerateTerrainEvent>,
) {
    TopBottomPanel::top("top_panel")
        .resizable(false)
        .show(contexts.ctx_mut(), |ui| {
            ui.horizontal(|ui| {
                if ui.button("Generation").clicked() {
                    ui_state.is_gen_window_expanded = !ui_state.is_gen_window_expanded;
                }
            })
        });

    Window::new("Terrain Generation Settings")
        .fixed_size((0f32, 0f32))
        .open(&mut ui_state.is_gen_window_expanded)
        .show(contexts.ctx_mut(), |ui| {
            Grid::new("terrain_generation_settings_grid").show(ui, |ui| {
                ui.heading("Cube edge length");
                ui.horizontal(|ui| {
                    ui.add(Slider::new(&mut gen_config.cube_size, 0.1..=10f32));
                });
                ui.end_row();

                ui.heading("Chunk size");
                ui.horizontal(|ui| {
                    let mut chunk_size = gen_config.chunk_size.to_array();
                    for (i, l) in chunk_size.iter_mut().zip(["x", "y", "z"]) {
                        ui.label(format!("{l}: "));
                        ui.add(DragValue::new(i).clamp_range(1u32..=u32::MAX));
                    }
                    gen_config.chunk_size = UVec3::from_array(chunk_size);
                });
                ui.end_row();

                ui.heading("Number of chunks");
                ui.horizontal(|ui| {
                    let mut chunks = gen_config.chunks.to_array();
                    for (i, l) in chunks.iter_mut().zip(["x", "y", "z"]) {
                        ui.label(format!("{l}: "));
                        ui.add(DragValue::new(i).clamp_range(1u32..=u32::MAX));
                    }
                    gen_config.chunks = UVec3::from_array(chunks);
                });
                ui.end_row();
            });
            ui.add_space(10f32);
            ui.vertical_centered_justified(|ui| {
                if ui.button("Generate").clicked() {
                    gen_event_writer.send(GenerateTerrainEvent);
                }
            });
        });
}