use amethyst::core::ecs::{Component, Entity};
use amethyst::prelude::{World, WorldExt};
use amethyst_test::EffectReturn;

/// Call this from `with_assertion` block
pub fn get_component<T: Component + Clone>(world: &World) -> T {
    let entity = world.read_resource::<EffectReturn<Entity>>().0.clone();
    let component_storage = world.read_storage::<T>();
    component_storage.get(entity).expect("Entity does not have the required component").clone()
}
