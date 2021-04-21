use amethyst::ecs::{System, ReadStorage, WriteStorage, Entities, Write, WriteExpect, ReadExpect};
use amethyst::core::{Transform, SystemBundle};
use amethyst::ecs::prelude::Join;
use amethyst::shrev::EventChannel;

use crate::collision::{Collider, check_body_collision, check_projectile_collision};
use crate::components::{Velocity, BoundingCircle, Projectile, Damage, Health};
use crate::resources::collision::{Collision, Collisions, ProjectileCollision, ProjectileCollisions};
use crate::events::{EntityDelete, DamageEvent};
use amethyst::core::ecs::{World, DispatcherBuilder};

pub struct CollisionBundle;

impl<'s, 'b> SystemBundle<'s, 'b> for CollisionBundle {
    fn build(self, world: &mut World, dispatcher: &mut DispatcherBuilder<'s, 'b>) -> amethyst::Result<()> {
        world.insert(Collisions::default());
        world.insert(ProjectileCollisions::default());

        dispatcher.add(CollisionSystem, "collision", &["physics"]);
        dispatcher.add(ProjectileCollisionSystem, "projectile_collision", &["physics"]);
        dispatcher.add(ProjectileCollisionHandler, "projectile_collision_handler", &["projectile_collision"]);
        dispatcher.add(CollisionHandlerForObstacles, "collision_handler", &["collision"]);
        Ok(())
    }
}

pub struct CollisionSystem;

impl<'s> System<'s> for CollisionSystem {
    type SystemData = (
        ReadStorage<'s, Transform>,
        ReadStorage<'s, Velocity>,
        ReadStorage<'s, BoundingCircle>,
        Entities<'s>,
        WriteExpect<'s, Collisions>
        );

    fn run(&mut self, (transforms, velocities, bounding_circles, entities, mut collision_resource): Self::SystemData) {
        collision_resource.0.clear();
        for (moving_id, moving_transform, moving_bounds, _) in (&entities, &transforms, &bounding_circles, &velocities).join()
        {
            // NOTE: this is not necessarily standing
            for (standing_id, standing_transform, standing_bounds) in (&entities, &transforms, &bounding_circles).join()
            {
                // Do not collide with itself
                if moving_id == standing_id
                {
                    continue;
                }

                if let Some(collision) = check_body_collision(
                    Collider{transform: moving_transform, bound: moving_bounds},
                    Collider{transform: standing_transform, bound: standing_bounds})
                {
                    collision_resource.0.push(Collision{collider: moving_id, collidee: standing_id, vector: collision});
                }
            }
        }
    }
}

pub struct CollisionHandlerForObstacles;

impl<'s> System<'s> for CollisionHandlerForObstacles {
    type SystemData = (
        ReadExpect<'s, Collisions>,
        WriteStorage<'s, Transform>
        );

    fn run(&mut self, (collisions, mut transforms): Self::SystemData) {
        for collision in &collisions.0 {
            if let Some(transform) = transforms.get_mut(collision.collider)
            {
                transform.prepend_translation_x(-collision.vector.x.into_pixel());
                transform.prepend_translation_y(-collision.vector.y.into_pixel());
            }
        }

    }
}

pub struct ProjectileCollisionSystem;

impl<'s> System<'s> for ProjectileCollisionSystem {
    type SystemData = (
        WriteStorage<'s, Transform>,
        ReadStorage<'s, Projectile>,
        ReadStorage<'s, BoundingCircle>,
        Entities<'s>,
        WriteExpect<'s, ProjectileCollisions>
        );
    fn run(&mut self, (transforms, projectiles, bounding_circles, entities, mut collision_resource): Self::SystemData) {
        collision_resource.0.clear();
        for (projectile_transform, _, projectile_id) in (&transforms, &projectiles, &entities).join()
        {
            for (object_transform, object_bounds, object_id) in (&transforms, &bounding_circles, &entities).join()
            {
                // unlikely
                if projectile_id == object_id
                {
                    continue;
                }

                if let Some(collision) = check_projectile_collision(
                    projectile_transform,
                    Collider{transform: object_transform, bound: object_bounds})
                {
                    collision_resource.0.push(ProjectileCollision{
                        projectile: projectile_id,
                        target: object_id,
                        vector: collision});
                }
            }
        }
    }
}



pub struct ProjectileCollisionHandler;

impl<'s> System<'s> for ProjectileCollisionHandler {
    type SystemData = (
        ReadExpect<'s, ProjectileCollisions>,
        Write<'s, EventChannel<EntityDelete>>,
        Write<'s, EventChannel<DamageEvent>>,
        ReadStorage<'s, Health>,
        ReadStorage<'s, Damage>,
        );

    // Here Projectile components are not explicitly filtered. ProjectCollisionSystem is expected
    // to put proper entities in `collision.projectile`
    fn run(&mut self, (collisions, mut entity_delete_channel, mut damage_event, healths, damages): Self::SystemData) {

        for collision in &collisions.0 {
            if healths.contains(collision.target) {
                if let Some(damage) = damages.get(collision.projectile) {
                    damage_event.single_write(DamageEvent { damage: *damage, target: collision.target })
            }}

            entity_delete_channel.single_write(EntityDelete{entity_id: collision.projectile})
        }

    }
}

