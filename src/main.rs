use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, spawn_initial_soldiers)
        .add_systems(Startup, add_ui)
        .run();
}

fn spawn_initial_soldiers(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let mesh = meshes.add(Circle::new(10.0));
    let material = materials.add(Color::WHITE);
    commands.spawn((
        Soldier::new(),
        Mesh2d(mesh.clone()),
        MeshMaterial2d(material.clone()),
        Transform::from_translation(Vec3::new(30.0, 30.0, 0.0)),
    ));
    commands.spawn((
        Soldier::new(),
        Mesh2d(mesh.clone()),
        MeshMaterial2d(material.clone()),
        Transform::from_translation(Vec3::new(30.0, -30.0, 0.0)),
    ));
    commands.spawn((
        Soldier::new(),
        Mesh2d(mesh.clone()),
        MeshMaterial2d(material.clone()),
        Transform::from_translation(Vec3::new(-30.0, 30.0, 0.0)),
    ));
}

fn add_ui(mut commands: Commands) {
    commands.spawn(Camera2d);
}

#[derive(Component)]
struct Soldier {
    hit_points: u32,
}

impl Soldier {
    fn new() -> Self {
        Self { hit_points: 100 }
    }
}

#[derive(Component)]
struct Arrow {
    velocity: Vec2,
}
