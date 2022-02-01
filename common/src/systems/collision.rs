use crate::collision::{check_body_collision, check_projectile_collision, Collider};
use crate::components::{BoundingCircle, Damage, Health, Projectile, Velocity};
use crate::events::{DamageEvent, EntityDelete};
use crate::resources::collision::{
    Collision, Collisions, ProjectileCollision, ProjectileCollisions,
};
use bevy::prelude::*;

pub struct CollisionPlugin;

impl bevy::app::Plugin for CollisionPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Collisions>()
            .init_resource::<ProjectileCollisions>();

        app.add_system_set(collision_system_set().after("physics"));
        app.add_system_set(projectile_collision_system_set().after("physics"));
    }
}

fn collision_system_set() -> SystemSet {
    SystemSet::new()
        .label("collision")
        .with_system(collect_collisions)
        .with_system(handle_obstacle_collisions)
}

fn projectile_collision_system_set() -> SystemSet {
    SystemSet::new()
        .with_system(collect_projectile_collisions.label("collect_projectile_collisions"))
        .with_system(
            handle_projectile_collisions
                .label("projectile_collision")
                .after("collect_projectile_collisions"),
        )
}

fn collect_collisions(
    moving_query: Query<(Entity, &Transform, &BoundingCircle), With<Velocity>>,
    standing_query: Query<(Entity, &Transform, &BoundingCircle)>,
    mut collision_res: ResMut<Collisions>,
) {
    collision_res.0.clear();
    for (moving_id, &moving_transform, moving_bounds) in moving_query.iter() {
        // NOTE: this is not necessarily standing
        for (standing_id, &standing_transform, standing_bounds) in standing_query.iter() {
            // Do not collide with itself
            if moving_id == standing_id {
                continue;
            }

            if let Some(collision) = check_body_collision(
                Collider {
                    transform: &moving_transform,
                    bound: &moving_bounds,
                },
                Collider {
                    transform: &standing_transform,
                    bound: &standing_bounds,
                },
            ) {
                collision_res.0.push(Collision {
                    collider: moving_id,
                    collidee: standing_id,
                    vector: collision,
                });
            }
        }
    }
}

fn handle_obstacle_collisions(
    collision_res: Res<Collisions>,
    mut transform_query: Query<&mut Transform>,
) {
    for collision in &collision_res.0 {
        if let Ok(mut transform) = transform_query.get_mut(collision.collider) {
            transform.translation.x -= collision.vector.x.into_pixel();
            transform.translation.y -= collision.vector.y.into_pixel();
        }
    }
}

fn collect_projectile_collisions(
    mut collision_res: ResMut<ProjectileCollisions>,
    projectile_query: Query<(Entity, &Transform), With<Projectile>>,
    maybe_collidee_query: Query<(Entity, &Transform, &BoundingCircle), Without<Projectile>>,
) {
    collision_res.0.clear();
    for (projectile_id, projectile_transform) in projectile_query.iter() {
        for (maybe_collidee_id, maybe_collidee_transform, maybe_collidee_bounds) in
            maybe_collidee_query.iter()
        {
            if let Some(collision) = check_projectile_collision(
                projectile_transform,
                Collider {
                    transform: maybe_collidee_transform,
                    bound: maybe_collidee_bounds,
                },
            ) {
                collision_res.0.push(ProjectileCollision {
                    projectile: projectile_id,
                    target: maybe_collidee_id,
                    vector: collision,
                });
            }
        }
    }
}

// Here Projectile components are not explicitly filtered. ProjectCollisionSystem is expected
// to put proper entities in `collision.projectile`
fn handle_projectile_collisions(
    collision_res: Res<ProjectileCollisions>,
    mut entity_delete_ec: EventWriter<EntityDelete>,
    mut damage_ec: EventWriter<DamageEvent>,
    healths: Query<&Health>,
    damages: Query<&Damage>,
) {
    for collision in &collision_res.0 {
        if healths.get(collision.target).is_ok() {
            if let Ok(damage) = damages.get(collision.projectile) {
                damage_ec.send(DamageEvent {
                    damage: *damage,
                    target: collision.target,
                });
            }
        }

        entity_delete_ec.send(EntityDelete {
            entity_id: collision.projectile,
        });
    }
}
