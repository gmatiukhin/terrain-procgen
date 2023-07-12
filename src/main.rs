use bevy::{app::AppExit, prelude::*};
use bevy_egui::EguiPlugin;
use terrain_procgen::generation::*;

mod ui;

fn main() {
    env_logger::init();
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(EguiPlugin)
        .add_plugins(MarchingCubesTerrain)
        .add_event::<AppExit>()
        .add_systems(Update, bevy::window::close_on_esc)
        .add_systems(Update, ui::ui_system)
        .run();
}
