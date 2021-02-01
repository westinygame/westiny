
use amethyst::prelude::*;
use amethyst::ecs::World;
use amethyst::assets::Loader;
use amethyst::audio::{SourceHandle, OggFormat};

pub struct Sounds
{
    pub single_shot: SourceHandle,
    pub dirt_hit : SourceHandle
}


pub fn initialize_audio(world: &mut World)
{
    let sounds = {
        let loader = world.read_resource::<Loader>();
        Sounds {
            single_shot: loader.load("audio/shot.ogg", OggFormat, (), &world.read_resource()),
            dirt_hit: loader.load("audio/dirt_hit_02.ogg", OggFormat, (), &world.read_resource())
        }
    };

    world.insert(sounds);
}
