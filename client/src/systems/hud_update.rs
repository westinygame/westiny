use amethyst::
{
    derive::SystemDesc,
    ecs::{System, ReadExpect, WriteStorage},
    ui::UiText,
};
use crate::resources::{Hud, format_health};

pub struct HudUpdateSystem;

impl<'s> System<'s> for HudUpdateSystem {
    type SystemData = (
        ReadExpect<'s, Hud>,
        WriteStorage<'s, UiText>
        );

    fn run(&mut self, (hud, mut ui_texts): Self::SystemData)
    {
        if let Some(text) = ui_texts.get_mut(hud.health) {
            text.text = format_health(100);
        }
    }
}
