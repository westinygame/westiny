use bevy::prelude::*;
use bevy::utils::{Instant, Duration};

use westiny_common::network::{PlayerNotification};

const NOTIFICATION_VISIBILITY: Duration = Duration::from_secs(10);

pub fn update_notification_bar(
    mut notifications: EventReader<PlayerNotification>,
    mut notification_display: Query<(&mut Text, &mut NotificationBarVisibility)>,
    time: Res<Time>
    )
{
    let current_time = time.startup() + time.time_since_startup();

    let (mut text, mut bar) = notification_display.single_mut();

    for notification in notifications.iter() {
        log::info!("PlayerNotification: {}", notification.message);
        show_message(&mut text.sections[0].value, &mut bar, &notification.message, current_time + NOTIFICATION_VISIBILITY);
    }

    if current_time > bar.visible_until {
        text.sections[0].value = "".to_string();
    }
}


// TODO Each message could be a separate entity with its own Lifespan component...

#[derive(Component)]
pub struct NotificationBarVisibility {
    pub visible_until: Instant
}

pub fn show_message(text: &mut String,  bar: &mut NotificationBarVisibility, message: &String, visible_until: Instant) {
    if !text.is_empty() {
        text.push_str("\n");
    }
    text.push_str(message);
    bar.visible_until = visible_until;
}

pub fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>
) {
    commands.spawn_bundle(
        TextBundle {
            style: Style {
                align_self: AlignSelf::FlexEnd,
                position_type: PositionType::Absolute,
                position: UiRect {
                    top: Val::Px(5.0),
                    right: Val::Px(5.0),
                    ..Default::default()
                },
                ..Default::default()
            },
            text: Text::from_section(
                "",
                TextStyle {
                    font: asset_server.load("fonts/carnevalee_freakshow.ttf"),
                    font_size: 15.0,
                    color: Color::WHITE,
                }
            ),
            ..Default::default()
        }
    ).insert(NotificationBarVisibility{visible_until: Instant::now()});
}
