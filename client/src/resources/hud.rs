use amethyst::ui::{Anchor, LineMode, TtfFormat, UiText, UiTransform};
use amethyst::ecs::prelude::Entity;
use amethyst::ecs::prelude::*;
use amethyst::assets::{Loader};

pub struct Hud {
    pub health: Entity,
    pub messages: Entity
}

pub fn initialize_hud(world: &mut World)
{
    let font = {
        let loader = world.read_resource::<Loader>();
        loader.load("fonts/square.ttf", TtfFormat, (), &world.read_resource())
    };
    let hud_transform = UiTransform::new(
        "HUD".to_string(),
        Anchor::TopRight,
        Anchor::TopMiddle,
        -100., -50., 1., // x,y,z
        150., 50., // width, height
        );

    let health = world.create_entity()
            .with(hud_transform)
            .with(UiText::new(
                    font.clone(),
                    format_health(0), // text
                    [1., 1., 1., 1.], // color
                    40., // font size
                    LineMode::Single,
                    Anchor::Middle)
            ).build();

    let message_panel_transform = UiTransform::new(
        "MessagePanel".to_string(),
        Anchor::TopLeft,
        Anchor::TopLeft,
        10., -10., 1.,
        250., 50.,
        );

    let message_panel = world.create_entity()
        .with(message_panel_transform)
        .with(UiText::new(
                font.clone(),
                "".to_string(),
                [1., 1., 1., 0.8], // color
                18., // font size
                LineMode::Wrap,
                Anchor::TopLeft)
            ).build();

    world.insert(Hud{
        health: health,
        messages: message_panel
    });

}

pub fn format_health(health: u16) -> String
{
    format!("HP {}", health)
}
