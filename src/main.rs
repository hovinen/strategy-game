use strategy::arrow::ArrowPlugin;
use strategy::soldier::SoldierPlugin;

use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(SoldierPlugin)
        .add_plugins(ArrowPlugin)
        .add_systems(Startup, add_ui)
        .run();
}

fn add_ui(mut commands: Commands) {
    commands.spawn(Camera2d);
}
