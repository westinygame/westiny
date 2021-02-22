use derive_new::new;
use amethyst::ecs::{System, SystemData, Entities};
use amethyst::derive::SystemDesc;
use amethyst::shrev::{ReaderId, EventChannel};

use westiny_common::resources::EntityDelete;
use westiny_common::network::NetworkEntityDelete;
use amethyst::core::ecs::{ReadStorage, Join, Entity, Read};
use westiny_common::components::NetworkId;

#[derive(SystemDesc, new)]
#[system_desc(name(NetworkEntityDeleteSystemDesc))]
pub struct NetworkEntityDeleteSystem {
    #[system_desc(event_channel_reader)]
    reader_delete: ReaderId<EntityDelete>,

    #[system_desc(event_channel_reader)]
    reader_network_delete: ReaderId<NetworkEntityDelete>,
}

impl<'s> System<'s> for NetworkEntityDeleteSystem {
    type SystemData = (
        Read<'s, EventChannel<EntityDelete>>,
        Read<'s, EventChannel<NetworkEntityDelete>>,
        ReadStorage<'s, NetworkId>,
        Entities<'s>);

    fn run(&mut self, (entity_deletions, net_entity_deletions, network_ids, entities): Self::SystemData) {
        let net_deletions: Vec<NetworkId> = net_entity_deletions.read(&mut self.reader_network_delete)
            .map(|del| del.network_id)
            .collect();

        let mut deletions: Vec<Entity> = entity_deletions.read(&mut self.reader_delete)
            .map(|del| del.entity_id)
            .collect();

        for (net_id, entity) in (&network_ids, &entities).join() {
            if net_deletions.contains(net_id) {
                deletions.push(entity);
            }
        }

        deletions.iter()
            // delete
            .map(|entity| entities.delete(*entity))
            // log errors
            .filter_map(|result| result.err())
            .for_each(|err| log::error!("Entity could not be deleted {}", err));
    }
}
