use westiny_common::resources::AudioQueue;

use bevy::prelude::*;
use crate::resources::Sounds;

pub fn play_audio(
    mut queue: ResMut<AudioQueue>,
    audio: Res<Audio>,
    sounds: Res<Sounds>)
{

    if let Some(sound_id) = queue.sound
    {
        let handle = &sounds.handles[sound_id as usize];
        audio.play(handle.clone());
    }

    queue.sound = None
}

