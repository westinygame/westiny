use amethyst::{
    derive::SystemDesc,
    ecs::{Join, Read, ReadStorage, System, SystemData, WriteStorage},
    shrev::{EventChannel, ReaderId},
};

use crate::components;
use crate::resources::{ClientID, NetworkCommand};
use derive_new::new;

#[derive(SystemDesc, new)]
#[system_desc(name(CommandTransformerSystemDesc))]
pub struct CommandTransformerSystem {
    #[system_desc(event_channel_reader)]
    reader: ReaderId<NetworkCommand>,
}

impl<'s> System<'s> for CommandTransformerSystem {
    type SystemData = (
        Read<'s, EventChannel<NetworkCommand>>,
        WriteStorage<'s, components::Input>,
        ReadStorage<'s, components::Client>,
    );

    fn run(&mut self, (command_channel, mut inputs, clients): Self::SystemData) {
        for command in command_channel.read(&mut self.reader) {
            match command {
                NetworkCommand::Input { id, input } => self.apply_client_input(id, &input, &clients, &mut inputs),
            }
        }
    }
}

impl CommandTransformerSystem {
    fn apply_client_input<'s>(
        &self,
        id: &ClientID,
        new_input: &components::Input,
        clients: &ReadStorage<'s, components::Client>,
        inputs: &mut WriteStorage<'s, components::Input>,
    ) {
        for (client, input) in (clients, inputs).join() {
            if &client.id == id {
                log::debug!("Assigning new input to client id={:?}, new input={:?}", &id, &new_input);
                *input = *new_input;
            }
        }
    }
}
