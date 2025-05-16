use bevy::prelude::*;

#[derive(Component)]
pub struct Health(u32);

impl Health {
    pub fn new(points: u32) -> Self {
        Self(points)
    }

    pub fn hit(&mut self, damage: u32) {
        self.0 = self.0.saturating_sub(damage);
    }

    pub fn is_dead(&self) -> bool {
        self.0 == 0
    }
}
