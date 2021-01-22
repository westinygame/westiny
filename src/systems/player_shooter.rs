
use amethyst::input::InputHandler;
use amethyst::derive::SystemDesc;
use amethyst::ecs::{Read, System, SystemData, ReadStorage, ReadExpect, Entities};
use amethyst::core::{Transform, math::{Vector3, Vector2}};
use amethyst::ecs::prelude::LazyUpdate;
use amethyst::ecs::prelude::Join;

use crate::systems::player_movement::{ActionBinding, MovementBindingTypes};
use crate::components::{Player, Velocity, DistanceLimit};
use crate::entities::spawn_bullet;
use crate::resources::{SpriteResource, SpriteId};

#[derive(SystemDesc)]
pub struct PlayerShooterSystem;

impl<'s> System<'s> for PlayerShooterSystem {
    type SystemData = (
        Entities<'s>,
        ReadStorage<'s, Transform>,
        ReadStorage<'s, Player>,
        Read<'s, InputHandler<MovementBindingTypes>>,
        ReadExpect<'s, SpriteResource>,
        ReadExpect<'s, LazyUpdate>
    );

    fn run(&mut self, (entities, transforms, players, input, sprites, lazy_update): Self::SystemData) {
        for (_player, player_transform) in (&players, &transforms).join() {
            if input.action_is_down(&ActionBinding::Shoot).unwrap_or(false)
            {
                let mut bullet_transform = Transform::default();
                bullet_transform.set_translation(*player_transform.translation());
                bullet_transform.set_rotation(*player_transform.rotation());

                let direction3d = (bullet_transform.rotation() * Vector3::y()).normalize();
                let speed = 100.0;
                let velocity = Velocity(Vector2::new(-direction3d.x * speed, -direction3d.y * speed));
                let distance_limit = DistanceLimit::new(100.0);
                spawn_bullet(bullet_transform, velocity, sprites.sprite_render_for(SpriteId::Bullet), distance_limit, &entities, &lazy_update);
            }
        }
    }
}
