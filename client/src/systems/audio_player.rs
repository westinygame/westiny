use amethyst::ecs::{System, Read, ReadExpect, WriteExpect};
use amethyst::assets::AssetStorage;
use amethyst::audio::{Source, output::Output};
use westiny_common::resources::AudioQueue;
use crate::resources::Sounds;

pub struct AudioPlayerSystem;

impl<'s> System<'s> for AudioPlayerSystem {
    type SystemData = (
        WriteExpect<'s, AudioQueue>,
        Read<'s, AssetStorage<Source>>,
        ReadExpect<'s, Sounds>,
        Read<'s, Output>
        );

    fn run(&mut self, (mut player, audio_storage, sounds, output): Self::SystemData)
    {
        if let Some(sound_id) = player.sound
        {
            let sound_handle = &sounds.handles[sound_id as usize];

            if let Some(sound) = audio_storage.get(sound_handle) {
                (*output).play_once(sound, 1.0);
            }

        }
        player.sound = None;

    }
}

