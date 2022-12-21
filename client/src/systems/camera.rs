use crate::components::Player;
use bevy::prelude::*;

#[derive(Component)]
pub struct PlayCamera;


pub fn setup(mut commands: Commands) {
    commands.spawn(
            Camera2dBundle::default())
        .insert(PlayCamera);
}

pub fn follow_player(
    player_query: Query<&GlobalTransform, (With<Player>, Without<Camera>)>,
    mut camera_query: Query<&mut Transform, (With<PlayCamera>, Without<Player>)>,
) {
    if let Some(player_transform) = player_query.iter().next() {
        let mut camera_transform = camera_query.single_mut();
        camera_transform.translation.x = player_transform.translation().x;
        camera_transform.translation.y = player_transform.translation().y;
    }
}
