use bevy::{app::AppExit, prelude::*};
use bevy_egui::EguiPlugin;
use terrain_procgen::generation::*;

mod ui;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(EguiPlugin)
        .init_resource::<TerrainGeneratorConfig>()
        .init_resource::<ui::UIState>()
        .add_event::<GenerateTerrainEvent>()
        .add_event::<AppExit>()
        .add_system(bevy::window::close_on_esc)
        .add_system(ui::ui_system)
        .add_system(generate_terrain)
        .run();
}
