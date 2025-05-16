use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, spawn_initial_soliders)
        .add_systems(Startup, add_ui)
        .add_systems(Update, shoot_arrows)
        .add_systems(Update, move_arrows)
        .add_systems(Update, injure_soldiers)
        .add_systems(Update, kill_soldiers)
        .insert_resource(ShootArrowsTimer(Timer::from_seconds(
            2.0,
            TimerMode::Repeating,
        )))
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

fn add_ui(mut commands: Commands) {
    commands.spawn(Camera2d);
}

fn shoot_arrows(
    mut commands: Commands,
    soldiers: Query<(&Team, &Transform), With<Soldier>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    time: Res<Time>,
    mut timer: ResMut<ShootArrowsTimer>,
) {
    if !timer.0.tick(time.delta()).just_finished() {
        return;
    }
    let material = materials.add(ColorMaterial::from_color(Color::WHITE));
    for (team, origin) in soldiers {
        if let Some((_, target)) = soldiers.iter().find(|(t, _)| *t != team) {
            let mesh = meshes.add(Triangle2d::new(
                vec2(-5.0, -5.0),
                vec2(5.0, -5.0),
                vec2(0.0, 5.0),
            ));
            let arrow_origin =
                origin.translation + (target.translation - origin.translation).normalize() * 5.0;
            commands.spawn((
                Arrow::new(origin.translation, target.translation),
                Transform::from_translation(arrow_origin),
                Mesh2d(mesh.clone()),
                MeshMaterial2d(material.clone()),
            ));
        }
    }
}

fn move_arrows(arrows: Query<(&Arrow, &mut Transform)>) {
    for (arrow, mut transform) in arrows {
        transform.translation += arrow.velocity;
    }
}

fn injure_soldiers(
    mut commands: Commands,
    soldiers: Query<(&mut Soldier, &Transform)>,
    arrows: Query<(Entity, &Transform), With<Arrow>>,
) {
    for (mut soldier, soldier_transform) in soldiers {
        for (entity, arrow_transform) in arrows {
            if (arrow_transform.translation - soldier_transform.translation).length() < 5.0 {
                commands.entity(entity).despawn();
                soldier.hit_points = soldier.hit_points.saturating_sub(10);
            }
        }
    }
}

fn kill_soldiers(mut commands: Commands, soldiers: Query<(Entity, &Soldier)>) {
    for (entity, soldier) in soldiers {
        if soldier.hit_points == 0 {
            commands.entity(entity).despawn();
        }
    }
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
    velocity: Vec3,
}

impl Arrow {
    fn new(origin: Vec3, target: Vec3) -> Self {
        Self {
            velocity: (target - origin).normalize(),
        }
    }
}

#[derive(Resource)]
struct ShootArrowsTimer(Timer);
