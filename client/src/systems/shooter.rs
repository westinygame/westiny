use westiny_common::resources::{AudioQueue, SoundId};
use westiny_common::entities::BulletBundle;
use westiny_common::network::ShotEvent;
use westiny_common::components::SpriteId;
use bevy::prelude::*;

pub fn spawn_bullets(
    mut commands: Commands,
    mut shot_events: EventReader<ShotEvent>,
    time: Res<Time>,
    mut audio: ResMut<AudioQueue>
) {
    let event_cnt = shot_events.iter()
        .inspect(|shot| {
            commands.spawn_bundle(
                BulletBundle::new(
                    shot.position,
                    shot.velocity,
                    shot.bullet_time_limit_secs,
                    time.time_since_startup())
                )
            .insert(SpriteId::Bullet);
        })
        .count();

        if event_cnt > 0 {
            audio.play(SoundId::SingleShot, 1.0);
        }
}

