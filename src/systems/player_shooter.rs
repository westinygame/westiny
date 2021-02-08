
use amethyst::derive::SystemDesc;
use amethyst::ecs::{Read, System, SystemData, ReadStorage, ReadExpect, Entities, WriteStorage};
use amethyst::core::{Transform, Time, math::{Vector3, Vector2}};
use amethyst::ecs::prelude::LazyUpdate;
use amethyst::ecs::prelude::Join;
use amethyst::assets::AssetStorage;
use amethyst::audio::{Source, output::Output};

use westiny_common::components::{Input, Player, weapon::Weapon, BoundingCircle};
use crate::entities::spawn_bullet;
use crate::resources::{SpriteResource, SpriteId, Sounds};

#[derive(SystemDesc)]
pub struct PlayerShooterSystem;

impl<'s> System<'s> for PlayerShooterSystem {
    type SystemData = (
        Entities<'s>,
        ReadStorage<'s, Transform>,
        ReadStorage<'s, Player>,
        ReadStorage<'s, Input>,
        ReadStorage<'s, BoundingCircle>,
        WriteStorage<'s, Weapon>,
        Read<'s, Time>,
        ReadExpect<'s, SpriteResource>,
        ReadExpect<'s, LazyUpdate>,
        Read<'s, AssetStorage<Source>>,
        ReadExpect<'s, Sounds>,
        Read<'s, Output>
    );

    fn run(&mut self, (entities, transforms, players, inputs, bounds, mut weapons, time, sprites, lazy_update, audio_storage, sounds, sound_output): Self::SystemData) {
        for (_player, input, player_transform, player_bound, mut weapon) in (&players, &inputs, &transforms, &bounds, &mut weapons).join() {
            if input.shoot
            {
                if weapon.is_allowed_to_shoot(time.absolute_time_seconds())
                {
                    let mut bullet_transform = Transform::default();
                    bullet_transform.set_translation(*player_transform.translation());
                    bullet_transform.set_rotation(*player_transform.rotation());

                    let direction3d = (bullet_transform.rotation() * Vector3::y()).normalize();
                    let direction2d = Vector2::new(-direction3d.x, -direction3d.y);

                    *bullet_transform.translation_mut() -= direction3d * player_bound.radius;

                    spawn_bullet(bullet_transform, direction2d, &weapon.details, sprites.sprite_render_for(SpriteId::Bullet), &entities, &lazy_update);
                    weapon.last_shot_time = time.absolute_time_seconds();
                    weapon.input_lifted = false;

                    if let Some(sound) = audio_storage.get(&sounds.single_shot) {
                        (*sound_output).play_once(sound, 1.0);
                    }
                }
            }
            else
            {
                weapon.input_lifted = true;
            }
        }
    }
}
