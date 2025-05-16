use bevy::prelude::*;

#[derive(Component, Copy, Clone, PartialEq, Eq)]
pub struct Team(pub u32);

impl Team {
    pub fn create_material(
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
