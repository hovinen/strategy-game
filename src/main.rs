use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, spawn_initial_soliders)
        .add_systems(Startup, add_ui)
        .run();
}

fn spawn_initial_soliders(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    spawn_soldier_group(
        Vec3::new(-200.0, 0.0, 0.0),
        Team(0),
        &mut commands,
        &mut meshes,
        &mut materials,
    );
    spawn_soldier_group(
        Vec3::new(200.0, 0.0, 0.0),
        Team(1),
        &mut commands,
        &mut meshes,
        &mut materials,
    );
}

#[derive(Component, Copy, Clone, PartialEq, Eq)]
struct Team(u32);

impl Team {
    fn create_material(
        &self,
        materials: &mut ResMut<Assets<ColorMaterial>>,
    ) -> Handle<ColorMaterial> {
        let color = match self.0 {
            0 => Color::Srgba(Srgba::rgb(0.0, 0.0, 1.0)),
            1 => Color::Srgba(Srgba::rgb(0.0, 1.0, 0.0)),
            _ => Color::Srgba(Srgba::rgb(1.0, 1.0, 1.0)),
        };
        materials.add(color)
    }
}

fn spawn_soldier_group(
    centred_on: Vec3,
    team: Team,
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
) {
    let mesh = meshes.add(Circle::new(10.0));
    let material = team.create_material(materials);
    commands.spawn((
        Soldier::new(),
        Mesh2d(mesh.clone()),
        MeshMaterial2d(material.clone()),
        Transform::from_translation(centred_on + Vec3::new(30.0, 30.0, 0.0)),
        team,
    ));
    commands.spawn((
        Soldier::new(),
        Mesh2d(mesh.clone()),
        MeshMaterial2d(material.clone()),
        Transform::from_translation(centred_on + Vec3::new(30.0, -30.0, 0.0)),
        team,
    ));
    commands.spawn((
        Soldier::new(),
        Mesh2d(mesh.clone()),
        MeshMaterial2d(material.clone()),
        Transform::from_translation(centred_on + Vec3::new(-30.0, 30.0, 0.0)),
        team,
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
