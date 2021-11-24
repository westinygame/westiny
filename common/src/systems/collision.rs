use crate::collision::{
    Collider,
    check_body_collision,
    check_projectile_collision
};
use crate::components::{
    Velocity,
    BoundingCircle,
    Projectile,
    Damage,
    Health
};
use crate::resources::collision::{
    Collision,
    Collisions,
    ProjectileCollision,
    ProjectileCollisions
};
/*
use crate::events::{
    EntityDelete,
    DamageEvent
};
*/
use bevy::prelude::*;

pub struct CollisionPlugin;

impl bevy::app::Plugin for CollisionPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.init_resource::<Collisions>()
           .init_resource::<ProjectileCollisions>();

        app.add_system_set(
                collision_system_set()
                    .after("physics"));
            /*
        app.add_system_set(
                projectile_collision_system_set()
                    .after("physics")
                    */
    }
}

fn collision_system_set() -> SystemSet {
    SystemSet::new()
        .label("collision")
        .with_system(collect_collisions.system())
        .with_system(handle_obstacle_collisions.system())
}

/*
fn projectile_collision_system_set() -> SystemSet {
    SystemSet::new()
        .label("projectile_collision")
        .with_system(collect_projectile_collisions.system())
        .with_system(handle_projectile_collisions.system()))
}
*/

fn collect_collisions(moving_query: Query<(Entity, &Transform, &BoundingCircle), With<Velocity>>,
                      standing_query: Query<(Entity, &Transform, &BoundingCircle)>,
                      mut collision_res: ResMut<Collisions>) {
    collision_res.0.clear();
    for (moving_id, &moving_transform, moving_bounds) in moving_query.iter() {
        // NOTE: this is not necessarily standing
        for (standing_id, &standing_transform, standing_bounds) in standing_query.iter() {
            // Do not collide with itself
            if moving_id == standing_id
            {
                continue;
            }

            if let Some(collision) = check_body_collision(
                Collider{transform: &moving_transform, bound: &moving_bounds},
                Collider{transform: &standing_transform, bound: &standing_bounds})
            {
                bevy::log::info!("COLLISION!!! vec: {:?}", collision);
                collision_res.0.push(Collision{collider: moving_id, collidee: standing_id, vector: collision});
            }
        }
    }
}

fn handle_obstacle_collisions(collision_res: Res<Collisions>,
                              mut transform_query: Query<&mut Transform>) {
    for collision in &collision_res.0 {
        if let Ok(mut transform) = transform_query.get_mut(collision.collider)
        {
            transform.translation.x -= collision.vector.x.into_pixel();
            transform.translation.y -= collision.vector.y.into_pixel();
        }
    }
}

/*
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
*/
