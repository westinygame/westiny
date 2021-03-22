
use amethyst::ecs::prelude::{Component, DenseVecStorage};
use std::time::Duration;

#[derive(Debug, PartialEq)]
pub enum Shot
{
    /// one shot per click (even when player holds down the button)
    Single,
    /// N shot per click
    #[allow(dead_code)] // Please remove this allow when using Burst
    Burst(u32),
    /// constant shooting, it will shoot while mouse button held down
    #[allow(dead_code)] // Please remove this allow when using Auto
    Auto
}

pub struct WeaponDetails
{
    /// Fire rate per seconds [1/s]
    pub fire_rate: f32,
    /// Number of bullets in a single magazine. 0 mean infinite (e.g. laser pistol)
    pub magazine_size: u32,
    /// When magazine_size > 0, amount of time required to reload [seconds]
    pub reload_time: f32,
    /// Damage caused by single bullet
    pub damage: u16,
    /// Bullet spread [degree]
    /// 0 is the perfect gun, always shooting where pointed
    /// 10 is a dumb shotgun
    pub spread: f32,
    /// Shooting time, bullets disappear after
    pub bullet_distance_limit: f32,
    pub bullet_speed: f32,
    pub shot: Shot
}

pub struct Weapon
{
    /// Time::absolute_time()
    pub last_shot_time: f64,
    /// Content of the weapon magazine
    pub bullets_left_in_magazine: u32,
    /// When reload is started. Defined only while reloading.
    pub reload_started_at: Option<Duration>,
    /// Flag required for single/burst shot weapons
    pub input_lifted: bool,
    /// Static details of the weapon.
    pub details: WeaponDetails,
}

impl Component for Weapon {
    type Storage = DenseVecStorage<Self>;
}

impl Weapon
{
    pub fn new(details: WeaponDetails) -> Self {
        Weapon {
            last_shot_time: 0.0,
            bullets_left_in_magazine: details.magazine_size,
            reload_started_at: None,
            input_lifted: true,
            details,
        }
    }

    pub fn is_allowed_to_shoot(&self, current_absolute_time: f64) -> bool {
        let shoot_interval = 1.0 / self.details.fire_rate as f64;
        let need_input_press = match self.details.shot {
            Shot::Single => true,
            Shot::Burst(_) => true,
            Shot::Auto => false
        };
        let input_ok: bool = !need_input_press || self.input_lifted;

        self.reload_started_at.is_none()
            && input_ok
            && self.bullets_left_in_magazine > 0
            && current_absolute_time > self.last_shot_time + shoot_interval
    }

    pub fn bullet_lifespan_sec(&self) -> f32 {
        self.details.bullet_distance_limit / self.details.bullet_speed
    }

    pub fn is_allowed_to_reload(&self) -> bool {
        self.reload_started_at.is_none()
    }
}
