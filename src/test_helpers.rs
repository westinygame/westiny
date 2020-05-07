
use amethyst::prelude::*;
use amethyst::shrev::EventChannel;
use amethyst::ecs::Entity;
use amethyst_test::prelude::*;
use amethyst::winit::{
    DeviceId,
    Event,
    WindowId,
    WindowEvent,
    ModifiersState,
};
use amethyst::winit::dpi::LogicalPosition;
use amethyst::ecs::Component;
use amethyst::input::{InputHandler, StringBindings};

pub fn make_window_event(win_event: WindowEvent) -> Event {
    Event::WindowEvent {
        window_id: unsafe { WindowId::dummy() },
        event: win_event
    }
}

pub fn make_cursor_moved(x: f64, y: f64) -> Event {
    make_window_event(
        WindowEvent::CursorMoved {
            device_id: unsafe { DeviceId::dummy() },
            position: LogicalPosition { x, y },
            modifiers: ModifiersState::default(),
        }
    )
}

/// Call this from `with_assertion` block
pub fn get_component<T: Component + Clone>(world: &World) -> T {
    let entity = world.read_resource::<EffectReturn<Entity>>().0.clone();
    let component_storage = world.read_storage::<T>();
    let component: T = component_storage.get(entity).expect("Entity does not have Transform???").clone();
    component
}

/// Call this from `with_effect` block
pub fn send_input_event(event: Event, world: &World) {
    let mut input_handler = world.fetch_mut::<InputHandler<StringBindings>>();
    let mut dummy_event_channel = EventChannel::<>::new();
    input_handler.send_event(&event, &mut dummy_event_channel, 1.0);
}
