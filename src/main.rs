use bevy::{app::AppExit, prelude::*};
use bevy_egui::{
    egui::{self, Align2, DragValue, FontId, RichText},
    EguiContexts, EguiPlugin,
};
use terrain_procgen::generation::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(EguiPlugin)
        .init_resource::<TerrainGeneratorConfig>()
        .init_resource::<UIState>()
        .add_event::<GenerateTerrainEvent>()
        .add_event::<AppExit>()
        .add_system(bevy::window::close_on_esc)
        .add_system(ui_system)
        .run();
}

#[derive(Resource, Debug, Default)]
struct UIState {
    is_side_pannel_expanded: bool,
}

fn ui_system(
    mut contexts: EguiContexts,
    mut gen_config: ResMut<TerrainGeneratorConfig>,
    mut ui_state: ResMut<UIState>,
    mut event: EventWriter<GenerateTerrainEvent>,
) {
    if !ui_state.is_side_pannel_expanded {
        egui::Area::new("open_side_panel")
            .anchor(Align2::RIGHT_TOP, (0f32, 0f32))
            .show(contexts.ctx_mut(), |ui| {
                ui.horizontal(|ui| {
                    if ui.button("Unfold").clicked() {
                        ui_state.is_side_pannel_expanded = true;
                    }
                })
            });
        return;
    }
    egui::SidePanel::right("Terrain Generator Settings")
        .resizable(false)
        .show_animated(contexts.ctx_mut(), ui_state.is_side_pannel_expanded, |ui| {
            ui.horizontal(|ui| {
                ui.heading("Terrain Generator Settings");
                if ui.button("Fold").clicked() {
                    ui_state.is_side_pannel_expanded = false;
                }
            });
            ui.vertical_centered_justified(|ui| {
                ui.horizontal(|ui| {
                    ui.label("Cube size: ");
                    ui.add(egui::Slider::new(&mut gen_config.cube_size, 0.1..=10f32));
                });

                ui.label(RichText::new("Chunk size").font(FontId::proportional(20f32)));
                ui.horizontal(|ui| {
                    let mut chunk_size = gen_config.chunk_size.to_array();
                    for (i, l) in chunk_size.iter_mut().zip(["x", "y", "z"]) {
                        ui.label(format!("{l}: "));
                        ui.add(DragValue::new(i).clamp_range(1u32..=u32::MAX));
                    }
                    gen_config.chunk_size = UVec3::from_array(chunk_size);
                });

                ui.label(RichText::new("Chunks").font(FontId::proportional(20f32)));
                ui.horizontal(|ui| {
                    let mut chunks = gen_config.chunks.to_array();
                    for (i, l) in chunks.iter_mut().zip(["x", "y", "z"]) {
                        ui.label(format!("{l}: "));
                        ui.add(DragValue::new(i).clamp_range(1u32..=u32::MAX));
                    }
                    gen_config.chunks = UVec3::from_array(chunks);
                });

                ui.add_space(10f32);
                if ui.button("Generate").clicked() {
                    event.send(GenerateTerrainEvent);
                }
            });
        });
}
