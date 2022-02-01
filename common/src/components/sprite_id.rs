use bevy::ecs::component::Component;

#[derive(Component)]
pub struct SpriteComponent {
    pub id: SpriteId,
}

#[derive(Copy, Clone)]
#[repr(usize)]
pub enum SpriteId {
    Player = 2,
    Barrel = 3,
    Corpse = 4,
    Bullet = 5,
    HandWithPistol = 6,
}
