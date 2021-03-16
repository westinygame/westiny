use derive_new::new;
use westiny_common::network::ShotEvent;
use amethyst::core::math::{Point2, Vector2};
use amethyst::prelude::{Builder, World};
use amethyst::core::{Transform, Time};
use std::time::Duration;
use amethyst::renderer::SpriteRender;
use crate::resources::SpriteResource;
use westiny_common::resources::{SpriteId, AudioQueue, SoundId};
use westiny_common::entities::spawn_bullet;
use amethyst::core::ecs::{System, ReaderId, Read, LazyUpdate, Entities, SystemData, WriteExpect};
use amethyst::core::ecs::shrev::EventChannel;

#[derive(Default)]
pub struct ShooterSystemDesc;

impl<'a, 'b> ::amethyst::core::SystemDesc<'a, 'b, ShooterSystem> for ShooterSystemDesc {
    fn build(self, world: &mut World) -> ShooterSystem {
        <ShooterSystem as System<'_>>::SystemData::setup(world);

        let reader_id = world
            .fetch_mut::<EventChannel<ShotEvent>>()
            .register_reader();

        let bullet_sprite = world.fetch::<SpriteResource>().sprite_render_for(SpriteId::Bullet);
        ShooterSystem::new(reader_id, bullet_sprite)
    }
}

#[derive(new)]
pub struct ShooterSystem {
    shot_reader: ReaderId<ShotEvent>,
    bullet_sprite: SpriteRender,
}

impl<'s> System<'s> for ShooterSystem {
    type SystemData = (
        Read<'s, EventChannel<ShotEvent>>,
        Read<'s, Time>,
        Read<'s, LazyUpdate>,
        Entities<'s>,
        WriteExpect<'s, AudioQueue>,
    );

    fn run(&mut self, (shot_event_channel, time, lazy, entities, mut audio): Self::SystemData) {
        let events = shot_event_channel.read(&mut self.shot_reader);
        if events.len() == 0 {
            return;
        }
        let current_time = time.absolute_time();
        audio.play(SoundId::SingleShot, 1.0);
        events.for_each(|shot_event|
                self.spawn_bullet(
                &shot_event.position,
                &shot_event.velocity,
                shot_event.bullet_time_limit_secs,
                current_time,
                lazy.create_entity(&entities)));


    }
}

impl ShooterSystem {
    fn spawn_bullet<B: Builder>(&self,
                                pos: &Point2<f32>,
                                velocity: &Vector2<f32>,
                                time_limit_sec: f32,
                                current_time: Duration,
                                entity_builder: B) {
        let transform = {
            let mut transform = Transform::default();
            transform.set_translation_xyz(pos.x, pos.y, 0.0);
            westiny_common::utilities::set_rotation_toward_vector(&mut transform, velocity);
            transform
        };

        let prepared_builder = entity_builder
            .with(self.bullet_sprite.clone());

        spawn_bullet(transform, *velocity, current_time, time_limit_sec, prepared_builder);
    }
}