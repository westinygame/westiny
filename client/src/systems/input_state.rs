use amethyst::ecs::{System, Read, Write, WriteStorage};
use amethyst::ecs::prelude::Join;
use amethyst::input::InputHandler;
use amethyst::network::simulation::{TransportResource, DeliveryRequirement, UrgencyRequirement};


use crate::bindings::{MovementBindingTypes, ActionBinding};
use crate::resources::StreamId;

use westiny_common::components::{InputFlags, Input};
use westiny_common::resources::{ServerAddress, CursorPosition};
use westiny_common::{network, serialize};

const INPUT_FLAG_MAPPING : [(InputFlags, ActionBinding); 13] = [
    (InputFlags::FORWARD,  ActionBinding::Forward),
    (InputFlags::BACKWARD, ActionBinding::Backward),
    (InputFlags::LEFT,     ActionBinding::StrafeLeft),
    (InputFlags::RIGHT,    ActionBinding::StrafeRight),
    (InputFlags::SHOOT,    ActionBinding::Shoot),
    (InputFlags::USE,      ActionBinding::Use),
    (InputFlags::RUN,      ActionBinding::Run),
    (InputFlags::RELOAD,   ActionBinding::Reload),
    (InputFlags::SELECT1,  ActionBinding::Select1),
    (InputFlags::SELECT2,  ActionBinding::Select2),
    (InputFlags::SELECT3,  ActionBinding::Select3),
    (InputFlags::SELECT4,  ActionBinding::Select4),
    (InputFlags::SELECT5,  ActionBinding::Select5),
];

fn update_input_keys(input: &mut Input, handler: &InputHandler<MovementBindingTypes>) {
    for (flag, binding) in INPUT_FLAG_MAPPING.iter() {
        input.flags.set(*flag, handler.action_is_down(binding).unwrap_or(false));
    }
}

fn update_input_cursor(input: &mut Input, cursor: &CursorPosition) {
    input.cursor = cursor.pos;
}

pub struct InputStateSystem;

// This system is responsible to send input data to the server.
// TODO This should be placed in the `client` subcrate.
impl<'s> System<'s> for InputStateSystem {
    type SystemData = (
       Read<'s, InputHandler<MovementBindingTypes>>,
       Read<'s, CursorPosition>,
       WriteStorage<'s, Input>,
       Read<'s, ServerAddress>,
       Write<'s, TransportResource>,
        );

    fn run(&mut self, (input_handler, cursor, mut inputs, server, mut net): Self::SystemData) {
        // NOTE: There is only one Input component exists on the client
        for mut input in (&mut inputs).join()
        {
            update_input_keys(&mut input, &input_handler);
            update_input_cursor(&mut input, &cursor);

            send_to_server(&mut net, &server, &input);
        }
    }
}

fn send_to_server(net: &mut TransportResource, server: &ServerAddress, input: &Input)
{
    let message = serialize(&network::PacketType::InputState{input: *input})
        .expect("InputState could not be serialized");

    net.send_with_requirements(server.address, &message, DeliveryRequirement::UnreliableSequenced(StreamId::InputState.into()), UrgencyRequirement::OnTick);
}



