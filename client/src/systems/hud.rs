use bevy::prelude::*;
use westiny_common::components::{Player, Health};
use crate::components::WeaponInfo;

pub fn update_hud(
    player_health: Query<&Health, With<Player>>,
    mut hud_health: Query<&mut Text, With<HudHealth>>
) {
    hud_health.single_mut().sections[0].value = format_health(player_health.single().0);
}
/*
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
*/

#[derive(Component)]
pub struct HudHealth;

pub fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    commands.spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                justify_content: JustifyContent::SpaceBetween,
                flex_direction: FlexDirection::RowReverse,
                ..Default::default()
            },
            color: Color::NONE.into(),
            ..Default::default()
        })
        .with_children(|parent| {
            parent
                .spawn_bundle(NodeBundle {
                    style: Style {
                        size: Size::new(Val::Percent(100.0), Val::Px(40.0)),
                        border: Rect::all(Val::Px(2.0)),
                        ..Default::default()
                    },
                    color: Color::rgba(0.23, 0.08, 0.05, 0.7).into(),
                    ..Default::default()
                })
                .with_children(|parent| {
                    parent.spawn_bundle(TextBundle {
                        style: Style {
                            margin:Rect::all(Val::Px(5.0)),
                            ..Default::default()
                        },
                        text: Text::with_section(
                            "",
                            TextStyle {
                                font: asset_server.load("fonts/carnevalee_freakshow.ttf"),
                                font_size: 30.0,
                                color: Color::WHITE,
                            },
                            Default::default(),
                        ),
                        ..Default::default()
                    })
                    .insert(HudHealth);
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
