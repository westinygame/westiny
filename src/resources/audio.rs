
use amethyst::prelude::*;
use amethyst::ecs::World;
use amethyst::assets::Loader;
use amethyst::audio::{SourceHandle, OggFormat};

pub struct Sounds
{
    // TODO use a vector/array
    pub single_shot: SourceHandle,
    pub dirt_hit : SourceHandle,
    pub ricochet : SourceHandle,
}

#[derive(Copy, Clone)]
#[repr(usize)]
pub enum SoundId {
    SingleShot = 0,
    DirtHit = 1,
    Ricochet = 2,
}

#[derive(Default)]
pub struct SoundPlayer {
    pub sound : Option<SoundId>,
}

impl SoundPlayer {
    pub fn play_sound(&mut self, id : SoundId)
    {
        self.sound = Some(id);
    }
}

pub fn initialize_audio(world: &mut World)
{
    let sounds = {
        let loader = world.read_resource::<Loader>();
        Sounds {
            single_shot: loader.load("audio/shot.ogg", OggFormat, (), &world.read_resource()),
            dirt_hit: loader.load("audio/dirt_hit_02.ogg", OggFormat, (), &world.read_resource()),
            ricochet: loader.load("audio/bullet_impact_solid_surface.ogg", OggFormat, (), &world.read_resource()),
        }
    };

    world.insert(sounds);
}
