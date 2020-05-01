use amethyst::input::{InputHandler, StringBindings};
use amethyst::derive::SystemDesc;
use amethyst::ecs::{Read, System, SystemData, World};

#[derive(SystemDesc)]
pub struct MouseDebugSystem;

const ACTIONS: &'static [&'static str] = &[
    "forward",
    "backward",
    "strafe_left",
    "strafe_right",
    "fire",
    "use",
];

impl<'s> System<'s> for MouseDebugSystem {
    type SystemData = Read<'s, InputHandler<StringBindings>>;

    fn run(&mut self, input: Self::SystemData) {
        if let Some((x, y)) = input.mouse_position() {
            println!("Mouse: ({}, {})", x, y);
        }

        for &action_name in ACTIONS {
            if let Some(action) = input.action_is_down(&action_name.to_string()) {
                if action {
                    println!("ACTION: {}", action_name);
                }
            }
        }
    }
}

