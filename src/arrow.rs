use bevy::prelude::*;

use crate::soldier::Soldier;
use crate::team::Team;

#[derive(Component)]
pub struct Arrow {
    pub velocity: Vec3,
}

impl Arrow {
    pub fn new(origin: Vec3, target: Vec3) -> Self {
        Self {
            velocity: (target - origin).normalize(),
        }
    }
}

#[derive(Resource)]
pub struct ShootArrowsTimer(Timer);

pub struct ArrowPlugin;

impl Plugin for ArrowPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(ShootArrowsTimer(Timer::from_seconds(
                2.0,
                TimerMode::Repeating,
            )))
            .add_systems(Update, shoot_arrows)
            .add_systems(Update, move_arrows);
    }
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
