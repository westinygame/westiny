use bevy::prelude::{AssetServer, AssetHandle, Res};

pub struct Sounds
{
    pub handles: [AssetHandle; 3],
}

pub fn initialize_audio(
    server: Res<AssetServer>,
    sounds: Res<Sounds>)
{
    let sounds = {
        let loader = world.read_resource::<Loader>();
        Sounds {
            handles: [
                server.load("audio/shot.ogg"),
                server.load("audio/handgun_ready.ogg"),
                server.load("audio/ouch.ogg"),
            ]
        }
    };
}

