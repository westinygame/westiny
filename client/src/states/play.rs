
use crate::states::AppState;
use crate::systems;
use bevy::prelude::SystemSet;

pub fn setup_system_set() -> SystemSet {
    SystemSet::on_enter(AppState::Play)
        .with_system(spawn_ball)
}

pub fn system_set() -> SystemSet {
    SystemSet::on_update(AppState::Play)
}

use bevy::prelude::{Sprite, Color, OrthographicCameraBundle, Vec3, Transform, SpriteBundle, Commands};

pub fn spawn_ball(mut commands: bevy::prelude::Commands) {
    println!("Spawning ball");
    // ball
    commands
        .spawn_bundle(SpriteBundle {
            transform: Transform {
                scale: Vec3::new(30.0, 30.0, 0.0),
                translation: Vec3::new(0.0, -50.0, 1.0),
                ..Default::default()
            },
            sprite: Sprite {
                color: Color::rgb(1.0, 0.5, 0.5),
                ..Default::default()
            },
            ..Default::default()
        });
}
