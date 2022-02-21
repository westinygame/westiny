use bevy::ecs::component::Component;

#[derive(Copy, Clone, PartialEq, Component)]
#[repr(usize)]
pub enum SpriteId {
    Grass = 0,
    Barren = 1,
    Player = 2,
    Barrel = 3,
    Corpse = 4,
    Bullet = 5,
    HandWithPistol = 6,
}
