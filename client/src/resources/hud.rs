use amethyst::ui::{Anchor, LineMode, TtfFormat, UiText, UiTransform};
use amethyst::ecs::prelude::Entity;
use amethyst::ecs::prelude::*;
use amethyst::assets::{Loader};

pub struct Hud {
    pub health: Entity,
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

    world.insert(Hud{
        health: health,
    });

}

pub fn format_health(health: u16) -> String
{
    format!("HP {}", health)
}
