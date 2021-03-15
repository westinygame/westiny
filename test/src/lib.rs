use amethyst::core::ecs::{Component, Entity};
use amethyst::prelude::{World, WorldExt};
use amethyst_test::EffectReturn;

/// Call this from `with_assertion` block
pub fn get_component<T: Component + Clone>(world: &World) -> T {
    let entity = world.read_resource::<EffectReturn<Entity>>().0.clone();
    let component_storage = world.read_storage::<T>();
    component_storage.get(entity).expect("Entity does not have the required component").clone()
}

pub fn f32_eq(f1: f32, f2: f32) -> bool {
    const F32_ALLOWED_DIFF: f32 = 0.00001;
    (f1 - f2).abs() < F32_ALLOWED_DIFF
}
