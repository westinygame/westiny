
use amethyst::input::InputHandler;
use amethyst::derive::SystemDesc;
use amethyst::ecs::{Read, System, SystemData, ReadStorage, ReadExpect, Entities, WriteStorage};
use amethyst::core::{Transform, Time, math::{Vector3, Vector2}};
use amethyst::ecs::prelude::LazyUpdate;
use amethyst::ecs::prelude::Join;

use crate::systems::player_movement::{ActionBinding, MovementBindingTypes};
use crate::components::{Player, Velocity, Weapon};
use crate::entities::spawn_bullet;
use crate::resources::{SpriteResource, SpriteId};

#[derive(SystemDesc)]
pub struct PlayerShooterSystem;

impl<'s> System<'s> for PlayerShooterSystem {
    type SystemData = (
        Entities<'s>,
        ReadStorage<'s, Transform>,
        ReadStorage<'s, Player>,
        WriteStorage<'s, Weapon>,
        Read<'s, InputHandler<MovementBindingTypes>>,
        Read<'s, Time>,
        ReadExpect<'s, SpriteResource>,
        ReadExpect<'s, LazyUpdate>
    );

    fn run(&mut self, (entities, transforms, players, mut weapons, input, time, sprites, lazy_update): Self::SystemData) {
        for (_player, player_transform, mut weapon) in (&players, &transforms, &mut weapons).join() {
            if input.action_is_down(&ActionBinding::Shoot).unwrap_or(false)
            {
                if weapon.is_allowed_to_shoot(time.absolute_time_seconds())
                {
                    let mut bullet_transform = Transform::default();
                    bullet_transform.set_translation(*player_transform.translation());
                    bullet_transform.set_rotation(*player_transform.rotation());

                    let direction3d = (bullet_transform.rotation() * Vector3::y()).normalize();
                    let direction2d = Vector2::new(-direction3d.x, -direction3d.y);

                    spawn_bullet(bullet_transform, direction2d, &weapon.details, sprites.sprite_render_for(SpriteId::Bullet), &entities, &lazy_update);
                    weapon.last_shot_time = time.absolute_time_seconds();
                    weapon.input_lifted = false;
                }
            }
            else
            {
                weapon.input_lifted = true;
            }
        }
    }
}
