use bevy_inspector_egui::bevy_egui::EguiPlugin;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use strategy::arrow::ArrowPlugin;
use strategy::soldier::SoldierPlugin;

use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(ArrowPlugin)
        .add_plugins(SoldierPlugin)
        .add_plugins(EguiPlugin {
            enable_multipass_for_primary_context: true,
        })
        .add_plugins(WorldInspectorPlugin::new())
        .add_systems(Startup, add_ui)
        .run();
}

fn add_ui(mut commands: Commands) {
    commands.spawn(Camera2d);
}
