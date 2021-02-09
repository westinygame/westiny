use amethyst::prelude::*;
use amethyst::ecs::World;
use amethyst::assets::Loader;
use amethyst::audio::{SourceHandle, OggFormat};

pub struct Sounds
{
    pub handles: [SourceHandle; 1],

}

pub fn initialize_audio(world: &mut World)
{
    let sounds = {
        let loader = world.read_resource::<Loader>();
        Sounds {
            handles: [
                loader.load("audio/shot.ogg", OggFormat, (), &world.read_resource())
            ]
        }
    };

    world.insert(sounds);
}

