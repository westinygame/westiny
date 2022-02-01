use crate::components;
use crate::resources::{ClientID, NetworkCommand};
use bevy::log::debug;
use bevy::prelude::{EventReader, Query};

pub fn transform_commands(
    mut network_commands: EventReader<NetworkCommand>,
    mut query: Query<(&components::Client, &mut components::Input)>,
) {
    for net_command in network_commands.iter() {
        match net_command {
            NetworkCommand::Input { id, input } => apply_client_input(id, input, &mut query),
        }
    }
}

fn apply_client_input(
    id: &ClientID,
    new_input: &components::Input,
    query: &mut Query<(&components::Client, &mut components::Input)>,
) {
    for (client, mut input) in query.iter_mut() {
        if &client.id == id {
            debug!(
                "Assigning new input to client id={:?}, new input={:?}",
                &id, &new_input
            );
            *input = *new_input;
        }
    }
}
