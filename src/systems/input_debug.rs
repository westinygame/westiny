use amethyst::input::{InputHandler, StringBindings};
use amethyst::derive::SystemDesc;
use amethyst::ecs::{Read, System, SystemData};

use log::info;

#[derive(SystemDesc, Default)]
pub struct InputDebugSystem {
    last_pos: (f32, f32),
}

const ACTIONS: &'static [&'static str] = &[
    "forward",
    "backward",
    "strafe_left",
    "strafe_right",
    "fire",
    "use",
];

impl<'s> System<'s> for InputDebugSystem {
    type SystemData = Read<'s, InputHandler<StringBindings>>;

    fn run(&mut self, input: Self::SystemData) {
        if let Some((x, y)) = input.mouse_position() {
            if (x, y) != self.last_pos {
                info!("Mouse: ({}, {})", x, y);
                self.last_pos = (x, y);
            }
        }

        for &action_name in ACTIONS {
            if let Some(action) = input.action_is_down(&action_name.to_string()) {
                if action {
                    info!("Action: {}", action_name);
                }
            }
        }

        if let Some(zoom) = input.axis_value("zoom") {
            info!("Zoom: {}", zoom);
        }
    }
}

