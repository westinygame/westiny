use amethyst::ecs::{System, Read, ReadExpect, WriteExpect};

use crate::resources::{Sounds, SoundPlayer, SoundId};
use amethyst::assets::AssetStorage;
use amethyst::audio::{Source, output::Output};

pub struct SoundPlayerSystem;

impl<'s> System<'s> for SoundPlayerSystem {
    type SystemData = (
        WriteExpect<'s, SoundPlayer>,
        Read<'s, AssetStorage<Source>>,
        ReadExpect<'s, Sounds>,
        Read<'s, Output>
        );

    fn run(&mut self, (mut player, audio_storage, sounds, output): Self::SystemData)
    {
        if let Some(sound_id) = player.sound
        {
            let sound_handle = match (sound_id) {
                SoundId::SingleShot => &sounds.single_shot,
                SoundId::DirtHit => &sounds.dirt_hit,
                SoundId::Ricochet => &sounds.ricochet,
            };

            if let Some(sound) = audio_storage.get(sound_handle) {
                (*output).play_once(sound, 1.0);
            }

        }
        player.sound = None;

    }
}
