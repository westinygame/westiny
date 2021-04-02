use amethyst::
{
    ecs::{System, ReadExpect, WriteStorage, ReadStorage, Join},
    ui::UiText,
};
use crate::resources::{Hud, format_health, format_ammo};
use westiny_common::components::{Player, Health};
use crate::components::WeaponInfo;

pub struct HudUpdateSystem;

impl<'s> System<'s> for HudUpdateSystem {
    type SystemData = (
        ReadExpect<'s, Hud>,
        WriteStorage<'s, UiText>,
        ReadStorage<'s, Player>,
        ReadStorage<'s, Health>,
        ReadStorage<'s, WeaponInfo>,
        );

    fn run(&mut self, (hud, mut ui_texts, players, healths, weapons): Self::SystemData)
    {
        for (_player, health, weapon_info) in (&players, &healths, &weapons).join()
        {
            if let Some(text) = ui_texts.get_mut(hud.health) {
                text.text = format_health(health.0);
            }

            if let Some(text) = ui_texts.get_mut(hud.ammo) {
                text.text = format_ammo(weapon_info.bullets_in_magazine, weapon_info.magazine_size);
            }
        }
    }
}
