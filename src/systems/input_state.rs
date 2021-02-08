use amethyst::ecs::{System, Read, Write, WriteStorage};
use amethyst::ecs::prelude::Join;
use amethyst::input::InputHandler;
use amethyst::network::simulation::{TransportResource, DeliveryRequirement, UrgencyRequirement};
use bincode::{serialize};

use westiny_common::components::Input;

use crate::resources::CursorPosition;
use crate::systems::player_movement::{MovementBindingTypes, ActionBinding};
use westiny_common::resources::ServerAddress;
use westiny_common::network;

fn update_input_keys(input: &mut Input, handler: &InputHandler<MovementBindingTypes>) {
    input.forward = handler.action_is_down(&ActionBinding::Forward).unwrap_or(false);
    input.backward = handler.action_is_down(&ActionBinding::Backward).unwrap_or(false);
    input.left = handler.action_is_down(&ActionBinding::StrafeLeft).unwrap_or(false);
    input.right = handler.action_is_down(&ActionBinding::StrafeRight).unwrap_or(false);
    input.shoot = handler.action_is_down(&ActionBinding::Shoot).unwrap_or(false);
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

    log::info!("Sending inputs...");
    net.send_with_requirements(server.address, &message, DeliveryRequirement::Reliable, UrgencyRequirement::OnTick);
}



