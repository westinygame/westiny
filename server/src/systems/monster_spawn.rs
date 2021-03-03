use amethyst::shred::System;
use amethyst::core::ecs::{Component, NullStorage, ReadStorage, LazyUpdate, ReadExpect, Entities, Builder, WriteExpect, Join, WriteStorage};
use crate::components::{Damage, BoundingCircle, NetworkId, EntityType, Input, Player, InputFlags, Velocity, Health};
use amethyst::core::Transform;
use crate::resources::NetworkIdSupplier;
use std::ops::Sub;
use amethyst::core::math::{Vector2, Point2};
use amethyst::core::num::real::Real;
use std::cmp::Ordering;
use crate::components::weapon::Weapon;

pub struct MonsterSystem;

impl<'s> System<'s> for MonsterSystem {
    type SystemData = (
        ReadStorage<'s, Monster>,
        Entities<'s>,
        ReadExpect<'s, LazyUpdate>,
        WriteExpect<'s, NetworkIdSupplier>,
        ReadStorage<'s, Transform>,
        ReadStorage<'s, Player>,
        WriteStorage<'s, Input>,
        ReadStorage<'s, Weapon>,
    );

    fn run(&mut self, (monsters, entities, lazy, mut net_id_supplier, transforms, players, mut inputs, weapons): Self::SystemData) {
        if monsters.count() == 0 {
            MonsterSystem::spawn_monster(&entities, &lazy, net_id_supplier.next(EntityType::Monster));
        }

        let mut player_transforms = Vec::new();
        (&transforms, &players).join().for_each(|(transform, _)| player_transforms.push(transform));

        for (_monster, input, monster_transform, weapon) in (&monsters, &mut inputs, &transforms, &weapons).join() {
           if let Some((player_distance, player_translation)) =  player_transforms.iter()
               .map(|player_transform| {
                   let diff = player_transform.translation() - monster_transform.translation();
                   (diff, player_transform)
               })
               .min_by(|(one_diff, one_transform), (other_diff, other_transform)|
                   if one_diff.norm() > other_diff.norm() {Ordering::Greater} else {Ordering::Less})
               .map(|(diff, transform)|(diff.norm(), transform.translation())){

               if player_distance < 256.0 {
                   input.cursor = Point2::<f32>::new(player_translation.x, player_translation.y);
                   if player_distance <= weapon.details.distance {
                       // input.flags.insert(InputFlags::SHOOT);
                   } else {
                       input.flags.insert(InputFlags::FORWARD);
                   }
               }
           } else {
               *input = Input::default();
           }
        }
    }
}

impl MonsterSystem {
    fn spawn_monster(
        entities: &Entities,
        lazy: &LazyUpdate,
        net_id: NetworkId,
    ) {
        use westiny_common::components::weapon;
        // TODO define these values in RON resource files. PREFAB?
        let monster_revolver = weapon::WeaponDetails {
            damage: 5,
            distance: 120.0,
            fire_rate: 1.4,
            magazine_size: 6,
            reload_time: 1.0,
            spread: 2.0,
            shot: weapon::Shot::Auto,
            bullet_speed: 200.0,
        };


        lazy.create_entity(entities)
            .with(Monster)
            .with(BoundingCircle{radius: 8.0})
            .with(Transform::default())
            .with(Velocity::default())
            .with(net_id)
            .with(Input::default())
            .with(weapon::Weapon::new(monster_revolver))
            .with(Health(50))
            .build();
    }
}

#[derive(Copy, Clone, Default)]
pub struct Monster;

impl Component for Monster {
    type Storage = NullStorage<Self>;
}