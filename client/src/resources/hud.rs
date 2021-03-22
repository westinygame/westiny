use amethyst::ui::{Anchor, LineMode, TtfFormat, UiText, UiTransform};
use amethyst::ecs::prelude::Entity;
use amethyst::ecs::prelude::*;
use amethyst::assets::{Loader};
use amethyst::core::Parent;

pub struct Hud {
    pub health: Entity,
    pub ammo: Entity,
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

    let ui_text = UiText::new(
        font.clone(),
        format_health(0), // text
        [1., 1., 1., 1.], // color
        40., // font size
        LineMode::Single,
        Anchor::Middle);
    let health = world.create_entity()
            .with(hud_transform)
            .with(ui_text.clone())
        .build();

    let ammo_transform = UiTransform::new(
            "ammo".to_string(),
            Anchor::TopRight,
            Anchor::TopMiddle,
            -100., -50., 1.,
            150., 50.,
        );
    let ammo = world.create_entity()
        .with(ammo_transform)
        .with(Parent { entity: health })
        .with(ui_text)
        .build();


    world.insert(Hud{
        health,
        ammo
    });

}

pub fn format_health(health: u16) -> String
{
    format!("HP {}", health)
}

pub fn format_ammo(ammo_in_magazine: u32, magazine_size: u32) -> String {
    format!("{} / {}", ammo_in_magazine, magazine_size)
}
