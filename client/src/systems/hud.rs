use bevy::prelude::*;
use westiny_common::components::{Player, Health};
use crate::components::WeaponInfo;

pub fn update_hud(
    query: Query<&Health, With<Player>>,
    mut hud_health: Query<&mut Text, With<HudHealth>>
) {
    hud_health.single_mut().sections[0].value = format_health(query.single().0);
}

pub fn update_hud_w(
    query: Query<&WeaponInfo, With<Player>>,
    mut hud_weapon_info: Query<&mut Text, With<HudWeaponInfo>>
) {
    hud_weapon_info.single_mut().sections[0].value = format_ammo(query.single().bullets_in_magazine, query.single().magazine_size);
}

#[derive(Component)]
pub struct HudHealth;

#[derive(Component)]
pub struct HudWeaponInfo;

pub fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {

    let hud_style = TextStyle  {
        font: asset_server.load("fonts/carnevalee_freakshow.ttf"),
        font_size: 30.0,
        color: Color::WHITE,
    };

    commands.spawn(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                justify_content: JustifyContent::SpaceBetween,
                flex_direction: FlexDirection::RowReverse,
                ..Default::default()
            },
            background_color: Color::NONE.into(),
            ..Default::default()
        })
        .with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    style: Style {
                        size: Size::new(Val::Percent(100.0), Val::Px(40.0)),
                        border: UiRect::all(Val::Px(2.0)),
                        ..Default::default()
                    },
                    background_color: Color::rgba(0.23, 0.08, 0.05, 0.7).into(),
                    ..Default::default()
                })
                .with_children(|parent| {
                    parent.spawn(TextBundle {
                        style: Style {
                            margin: UiRect::all(Val::Px(5.0)),
                            ..Default::default()
                        },
                        text: Text::from_section(
                            "".to_string(),
                            hud_style.clone()
                            ),
                        ..Default::default()
                    })
                    .insert(HudHealth);


                });
        });

    commands.spawn(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                justify_content: JustifyContent::SpaceBetween,
                flex_direction: FlexDirection::RowReverse,
                ..Default::default()
            },
            background_color: Color::NONE.into(),
            ..Default::default()
        })
        .with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    style: Style {
                        size: Size::new(Val::Percent(100.0), Val::Px(40.0)),
                        border: UiRect::all(Val::Px(2.0)),
                        ..Default::default()
                    },
                    background_color: Color::rgba(0.23, 0.08, 0.05, 0.7).into(),
                    ..Default::default()
                })
                .with_children(|parent| {
                    parent.spawn(TextBundle {
                        style: Style {
                            margin: UiRect::all(Val::Px(5.0)),
                            ..Default::default()
                        },
                        text: Text::from_section(
                            "",
                            hud_style
                        ),
                        ..Default::default()
                    })
                    .insert(HudWeaponInfo);
                });
        });
}

fn format_health(health: u16) -> String
{
    format!("HP {}", health)
}

fn format_ammo(ammo_in_magazine: u32, magazine_size: u32) -> String {
    format!("{} / {}", ammo_in_magazine, magazine_size)
}
