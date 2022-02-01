use crate::network::PacketType;
use anyhow::Result;
use thiserror::Error;

#[derive(Error, Debug)]
#[error("Could not encode packet: {0:?}")]
pub struct EncodeError(rmp_serde::encode::Error);

#[derive(Error, Debug)]
#[error("Could not decode packet: {0:?}")]
pub struct DecodeError(rmp_serde::decode::Error);

pub fn serialize(packet: &PacketType) -> Result<Vec<u8>, EncodeError> {
    rmp_serde::to_vec(packet).map_err(|e| EncodeError(e))
}

pub fn deserialize(buf: &[u8]) -> Result<PacketType, DecodeError> {
    rmp_serde::from_read_ref(buf).map_err(|e| DecodeError(e))
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::components::{EntityType, Input, InputFlags, NetworkId};
    use crate::metric_dimension::length::{Meter, MeterVec2};
    use crate::network::EntityState;
    use proptest::prelude::*;

    fn packet_enum_strategy() -> impl Strategy<Value = PacketType> {
        prop_oneof![
            any::<String>().prop_map(|name| PacketType::ConnectionRequest { player_name: name }),
            input_state_gen(),
            entity_state_update_gen()
        ]
    }

    prop_compose! {
        fn input_state_gen()(p in arb_point2(),
                             flags in 0..=InputFlags::all().bits()) -> PacketType {
            PacketType::InputState {
                input: Input {
                    flags: InputFlags::from_bits(flags).unwrap(),
                    cursor: p
                }
            }
        }
    }

    prop_compose! {
        fn entity_state_update_gen()(id in network_id_gen(),
                                     pos in arb_point2(),
                                     rot in any::<f32>()) -> PacketType {
            PacketType::EntityStateUpdate(
                vec![EntityState {
                        network_id: id,
                        position: pos,
                        rotation: rot,
                    }]
            )
        }
    }

    prop_compose! {
        fn network_id_gen()(entity_type in entity_type_strategy(), id in any::<u32>()) -> NetworkId {
            NetworkId::new(entity_type, id)
        }
    }

    fn entity_type_strategy() -> impl Strategy<Value = EntityType> {
        prop_oneof![Just(EntityType::Player),]
    }

    prop_compose! {
        fn arb_point2()(x in any::<f32>(), y in any::<f32>()) -> MeterVec2 {
            MeterVec2::
        }
    }

    proptest! {
        #[test]
        fn encode_decode(packet in packet_enum_strategy()) {
            println!("{:?}", &packet);
            assert_eq!(packet, deserialize(&serialize(&packet).unwrap()).unwrap());
        }
    }
}
