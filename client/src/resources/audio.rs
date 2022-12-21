use bevy::prelude::{Resource, AudioSource, AssetServer, Handle, Res, ResMut};

#[derive(Default, Resource)]
pub struct Sounds
{
    pub handles: [Handle<AudioSource>; 3],
}

pub fn initialize_audio(
    server: Res<AssetServer>,
    mut sounds: ResMut<Sounds>)
{
    sounds.handles = [
        server.load("audio/shot.ogg"),
        server.load("audio/handgun_ready.ogg"),
        server.load("audio/ouch.ogg"),
    ];
}

