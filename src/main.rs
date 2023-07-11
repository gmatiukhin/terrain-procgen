use bevy::{app::AppExit, prelude::*};
use bevy_egui::EguiPlugin;
use terrain_procgen::generation::*;

mod ui;

fn main() {
    env_logger::init();
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(EguiPlugin)
        .add_plugin(MarchingCubesTerrain)
        .add_event::<AppExit>()
        .add_system(bevy::window::close_on_esc)
        .add_system(ui::ui_system)
        .run();
}
