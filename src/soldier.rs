use bevy::prelude::*;

use crate::team::Team;

pub struct SoldierPlugin;

impl Plugin for SoldierPlugin {
    fn build(&self, app: &mut App) {
        println!("Built Soldier Plugin!");
        app.add_systems(Startup, spawn_initial_soldiers);
    }
}

fn spawn_initial_soldiers(
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
        Transform::from_translation(centred_on + vec3(30.0, 30.0, 0.0)),
        team,
    ));
    commands.spawn((
        Soldier::new(),
        Mesh2d(mesh.clone()),
        MeshMaterial2d(material.clone()),
        Transform::from_translation(centred_on + vec3(30.0, -30.0, 0.0)),
        team,
    ));
    commands.spawn((
        Soldier::new(),
        Mesh2d(mesh.clone()),
        MeshMaterial2d(material.clone()),
        Transform::from_translation(centred_on + vec3(-30.0, 30.0, 0.0)),
        team,
    ));
}

#[derive(Component)]
pub struct Soldier {
    pub hit_points: u32,
}

impl Soldier {
    pub fn new() -> Self {
        Self { hit_points: 100 }
    }
}

#[derive(Bundle)]
pub struct SoldierBundle {
    soldier: Soldier,
    mesh: Mesh2d,
    material: MeshMaterial2d<ColorMaterial>,
    transform: Transform,
    team: Team,
}
