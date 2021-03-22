use amethyst::
{
    ecs::{System, ReadExpect, WriteStorage, ReadStorage, Join},
    ui::UiText,
};
use crate::resources::{Hud, format_health, format_ammo};
use westiny_common::components::{Player, Health};
use westiny_common::components::weapon::Weapon;

pub struct HudUpdateSystem;

impl<'s> System<'s> for HudUpdateSystem {
    type SystemData = (
        ReadExpect<'s, Hud>,
        WriteStorage<'s, UiText>,
        ReadStorage<'s, Player>,
        ReadStorage<'s, Health>,
        ReadStorage<'s, Weapon>,
        );

    fn run(&mut self, (hud, mut ui_texts, players, healths, weapons): Self::SystemData)
    {
        for (_player, health, weapon) in (&players, &healths, &weapons).join()
        {
            if let Some(text) = ui_texts.get_mut(hud.health) {
                text.text = format_health(health.0);
            }

            if let Some(text) = ui_texts.get_mut(hud.ammo) {
                text.text = format_ammo(weapon.bullets_left_in_magazine, weapon.details.magazine_size);
            }
        }
    }
}
