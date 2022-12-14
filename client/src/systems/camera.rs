use crate::components::Player;
use bevy::prelude::*;
use bevy::input::mouse::{MouseScrollUnit, MouseWheel};

#[derive(Component)]
pub struct PlayCamera;


pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn_bundle(
            Camera2dBundle::default())
            //Camera2dBundle{
            //    projection: OrthographicProjection{
            //        scale: 0.8,
            //        ..default()
            //    },
            //    ..default()
            //})
        .insert(PlayCamera);

    // commands
    //     .spawn_bundle(UiCameraBundle::default());

        /*
        .with_children(|parent| {
            // left vertical fill (border)
            parent
                .spawn_bundle(NodeBundle {
                    style: Style {
                        size: Size::new(Val::Px(200.0), Val::Percent(100.0)),
                        border: Rect::all(Val::Px(2.0)),
                        ..Default::default()
                    },
                    color: Color::rgba(0.65, 0.65, 0.65, 0.4).into(),
                    ..Default::default()
                })
                .with_children(|parent| {
                    // left vertical fill (content)
                    parent
                        .spawn_bundle(NodeBundle {
                            style: Style {
                                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                                align_items: AlignItems::FlexEnd,
                                ..Default::default()
                            },
                            color: Color::rgba(0.15, 0.15, 0.15, 0.4).into(),
                            ..Default::default()
                        })
                        .with_children(|parent| {
                            // text
                            parent.spawn_bundle(TextBundle {
                                style: Style {
                                    margin: Rect::all(Val::Px(5.0)),
                                    ..Default::default()
                                },
                                text: Text::with_section(
                                    "Text Example",
                                    TextStyle {
                                        font: asset_server.load("fonts/carnevalee_freakshow.ttf"),
                                        font_size: 30.0,
                                        color: Color::WHITE,
                                    },
                                    Default::default(),
                                ),
                                ..Default::default()
                            });
                        });
                });
            // right vertical fill
            parent
                .spawn_bundle(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::ColumnReverse,
                        justify_content: JustifyContent::Center,
                        size: Size::new(Val::Px(200.0), Val::Percent(100.0)),
                        ..Default::default()
                    },
                    color: Color::rgb(0.15, 0.15, 0.15).into(),
                    ..Default::default()
                })
                .with_children(|parent| {
                    // Title
                    parent.spawn_bundle(TextBundle {
                        style: Style {
                            size: Size::new(Val::Undefined, Val::Px(25.)),
                            margin: Rect {
                                left: Val::Auto,
                                right: Val::Auto,
                                ..Default::default()
                            },
                            ..Default::default()
                        },
                        text: Text::with_section(
                            "Scrolling list",
                            TextStyle {
                                font: asset_server.load("fonts/carnevalee_freakshow.ttf"),
                                font_size: 25.,
                                color: Color::WHITE,
                            },
                            Default::default(),
                        ),
                        ..Default::default()
                    });
                    // List with hidden overflow
                    parent
                        .spawn_bundle(NodeBundle {
                            style: Style {
                                flex_direction: FlexDirection::ColumnReverse,
                                align_self: AlignSelf::Center,
                                size: Size::new(Val::Percent(100.0), Val::Percent(50.0)),
                                overflow: Overflow::Hidden,
                                ..Default::default()
                            },
                            color: Color::rgb(0.10, 0.10, 0.10).into(),
                            ..Default::default()
                        })
                        .with_children(|parent| {
                            // Moving panel
                            parent
                                .spawn_bundle(NodeBundle {
                                    style: Style {
                                        flex_direction: FlexDirection::ColumnReverse,
                                        flex_grow: 1.0,
                                        max_size: Size::new(Val::Undefined, Val::Undefined),
                                        ..Default::default()
                                    },
                                    color: Color::NONE.into(),
                                    ..Default::default()
                                })
                                .insert(ScrollingList::default())
                                .with_children(|parent| {
                                    // List items
                                    for i in 0..30 {
                                        parent.spawn_bundle(TextBundle {
                                            style: Style {
                                                flex_shrink: 0.,
                                                size: Size::new(Val::Undefined, Val::Px(20.)),
                                                margin: Rect {
                                                    left: Val::Auto,
                                                    right: Val::Auto,
                                                    ..Default::default()
                                                },
                                                ..Default::default()
                                            },
                                            text: Text::with_section(
                                                format!("Item {}", i),
                                                TextStyle {
                                                    font: asset_server.load("fonts/carnevalee_freakshow.ttf"),
                                                    font_size: 20.,
                                                    color: Color::WHITE,
                                                },
                                                Default::default(),
                                            ),
                                            ..Default::default()
                                        });
                                    }
                                });
                        });
                });
            // absolute positioning blue rectangles
            parent
                .spawn_bundle(NodeBundle {
                    style: Style {
                        size: Size::new(Val::Px(200.0), Val::Px(200.0)),
                        position_type: PositionType::Absolute,
                        position: Rect {
                            left: Val::Px(210.0),
                            bottom: Val::Px(10.0),
                            ..Default::default()
                        },
                        border: Rect::all(Val::Px(20.0)),
                        ..Default::default()
                    },
                    color: Color::rgb(0.4, 0.4, 1.0).into(),
                    ..Default::default()
                })
                .with_children(|parent| {
                    parent.spawn_bundle(NodeBundle {
                        style: Style {
                            size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                            ..Default::default()
                        },
                        color: Color::rgb(0.8, 0.8, 1.0).into(),
                        ..Default::default()
                    });
                });
            // render order test: reddest in the back, whitest in the front (flex center)
            parent
                .spawn_bundle(NodeBundle {
                    style: Style {
                        size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                        position_type: PositionType::Absolute,
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        ..Default::default()
                    },
                    color: Color::NONE.into(),
                    ..Default::default()
                })
                .with_children(|parent| {
                    parent
                        .spawn_bundle(NodeBundle {
                            style: Style {
                                size: Size::new(Val::Px(100.0), Val::Px(100.0)),
                                ..Default::default()
                            },
                            color: Color::rgb(1.0, 0.0, 0.0).into(),
                            ..Default::default()
                        })
                        .with_children(|parent| {
                            parent.spawn_bundle(NodeBundle {
                                style: Style {
                                    size: Size::new(Val::Px(100.0), Val::Px(100.0)),
                                    position_type: PositionType::Absolute,
                                    position: Rect {
                                        left: Val::Px(20.0),
                                        bottom: Val::Px(20.0),
                                        ..Default::default()
                                    },
                                    ..Default::default()
                                },
                                color: Color::rgb(1.0, 0.3, 0.3).into(),
                                ..Default::default()
                            });
                            parent.spawn_bundle(NodeBundle {
                                style: Style {
                                    size: Size::new(Val::Px(100.0), Val::Px(100.0)),
                                    position_type: PositionType::Absolute,
                                    position: Rect {
                                        left: Val::Px(40.0),
                                        bottom: Val::Px(40.0),
                                        ..Default::default()
                                    },
                                    ..Default::default()
                                },
                                color: Color::rgb(1.0, 0.5, 0.5).into(),
                                ..Default::default()
                            });
                            parent.spawn_bundle(NodeBundle {
                                style: Style {
                                    size: Size::new(Val::Px(100.0), Val::Px(100.0)),
                                    position_type: PositionType::Absolute,
                                    position: Rect {
                                        left: Val::Px(60.0),
                                        bottom: Val::Px(60.0),
                                        ..Default::default()
                                    },
                                    ..Default::default()
                                },
                                color: Color::rgb(1.0, 0.7, 0.7).into(),
                                ..Default::default()
                            });
                            // alpha test
                            parent.spawn_bundle(NodeBundle {
                                style: Style {
                                    size: Size::new(Val::Px(100.0), Val::Px(100.0)),
                                    position_type: PositionType::Absolute,
                                    position: Rect {
                                        left: Val::Px(80.0),
                                        bottom: Val::Px(80.0),
                                        ..Default::default()
                                    },
                                    ..Default::default()
                                },
                                color: Color::rgba(1.0, 0.9, 0.9, 0.4).into(),
                                ..Default::default()
                            });
                        });
                });
                parent.spawn_bundle(NodeBundle {
                    style: Style {
                        size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                        position_type: PositionType::Absolute,
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::FlexEnd,
                        ..Default::default()
                    },
                    color: Color::NONE.into(),
                    ..Default::default()
                })
                .with_children(|parent| {
                    // bevy logo (image)
                    parent.spawn_bundle(ImageBundle {
                        style: Style {
                            size: Size::new(Val::Px(500.0), Val::Auto),
                            ..Default::default()
                        },
                        image: asset_server.load("spritesheet.png").into(),
                        ..Default::default()
                    });
                });
        });
*/
}

pub fn follow_player(
    player_query: Query<&GlobalTransform, (With<Player>, Without<Camera>)>,
    mut camera_query: Query<&mut Transform, (With<PlayCamera>, Without<Player>)>,
) {
    if let Some(player_transform) = player_query.iter().next() {
        let mut camera_transform = camera_query.single_mut();
        camera_transform.translation.x = player_transform.translation().x;
        camera_transform.translation.y = player_transform.translation().y;
    }
}

#[derive(Component, Default)]
struct ScrollingList {
    position: f32,
}

fn mouse_scroll(
    mut mouse_wheel_events: EventReader<MouseWheel>,
    mut query_list: Query<(&mut ScrollingList, &mut Style, &Children, &Node)>,
    query_item: Query<&Node>,
) {
    for mouse_wheel_event in mouse_wheel_events.iter() {
        for (mut scrolling_list, mut style, children, uinode) in query_list.iter_mut() {
            let items_height: f32 = children
                .iter()
                .map(|entity| query_item.get(*entity).unwrap().size.y)
                .sum();
            let panel_height = uinode.size.y;
            let max_scroll = (items_height - panel_height).max(0.);
            let dy = match mouse_wheel_event.unit {
                MouseScrollUnit::Line => mouse_wheel_event.y * 20.,
                MouseScrollUnit::Pixel => mouse_wheel_event.y,
            };
            scrolling_list.position += dy;
            scrolling_list.position = scrolling_list.position.clamp(-max_scroll, 0.);
            style.position.top = Val::Px(scrolling_list.position);
        }
    }
}
