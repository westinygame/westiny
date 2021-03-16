use westiny_common::components;
use std::collections::HashMap;
use crate::components::{EntityType, NetworkId};

#[derive(Default)]
pub struct NetworkIdSupplier {
    next_ids: HashMap<EntityType, u32>,
}

impl NetworkIdSupplier {
    pub fn new() -> Self {
        NetworkIdSupplier { next_ids: HashMap::new() }
    }

    pub fn next(&mut self, entity_type: EntityType) -> components::NetworkId {
        let next = self.next_ids.entry(entity_type).or_insert(0);
        let network_id = NetworkId::new(entity_type, next.clone());
        *next += 1;
        network_id
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn network_id_supplier_incremental() {
        let mut supplier = NetworkIdSupplier::new();
        for i in 0..1000 {
            let actual = supplier.next(EntityType::Player);
            let expected = NetworkId { entity_type: EntityType::Player, id: i};
            assert_eq!(expected, actual, "With Player entity")
        }

        // TODO place here another network EntitiyType when we have other than the Player

        for i in 1000..1100 {
            let actual = supplier.next(EntityType::Player);
            let expected = NetworkId { entity_type: EntityType::Player, id: i};
            assert_eq!(expected, actual, "With Player entity")
        }
    }
}