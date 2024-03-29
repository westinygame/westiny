#[derive(Copy, Clone)]
#[repr(usize)]
pub enum SoundId {
    SingleShot = 0,
    WeaponReady = 1,
    Ouch = 2,
}

#[derive(Default, bevy::prelude::Resource)]
pub struct AudioQueue {
    pub sound: Option<SoundId>,
}

impl AudioQueue {
    pub fn play(&mut self, id: SoundId, _volume: f32) {
        self.sound = Some(id);
    }
}
