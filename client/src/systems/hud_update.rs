use amethyst::
{
    ecs::{System, ReadExpect, WriteStorage, ReadStorage, Join},
    ui::UiText,
};
use crate::resources::{Hud, format_health};
use westiny_common::components::{Player, Health};

pub struct HudUpdateSystem;

impl<'s> System<'s> for HudUpdateSystem {
    type SystemData = (
        ReadExpect<'s, Hud>,
        WriteStorage<'s, UiText>,
        ReadStorage<'s, Player>,
        ReadStorage<'s, Health>,
        );

    fn run(&mut self, (hud, mut ui_texts, players, healths): Self::SystemData)
    {
        for (_player, health) in (&players, &healths).join()
        {
            if let Some(text) = ui_texts.get_mut(hud.health) {
                text.text = format_health(health.0);
            }
        }
    }
}
