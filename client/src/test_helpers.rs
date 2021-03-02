use amethyst::winit::{WindowEvent, Event, WindowId, DeviceId, ModifiersState, dpi::LogicalPosition};
use amethyst::prelude::World;
use amethyst::input::InputHandler;
use amethyst::shrev::EventChannel;
use crate::bindings::MovementBindingTypes;

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

/// Call this from `with_effect` block
pub fn send_input_event(event: Event, world: &World) {
    let mut input_handler = world.fetch_mut::<InputHandler<MovementBindingTypes>>();
    let mut dummy_event_channel = EventChannel::<>::new();
    input_handler.send_event(&event, &mut dummy_event_channel, 1.0);
}
