use amethyst::
{
    core::Time,
    ecs::{System, SystemData, ReadExpect, WriteExpect, WriteStorage},
    ecs::shrev::{ReaderId, EventChannel},
    ui::UiText,
};
use amethyst::derive::SystemDesc;
use derive_new::new;

use crate::resources::{NotificationBar};
use westiny_common::network::{PlayerNotification};

#[derive(SystemDesc, new)]
#[system_desc(name(NotificationBarSystemDesc))]
pub struct NotificationBarSystem {
    #[system_desc(event_channel_reader)]
    notification_reader: ReaderId<PlayerNotification>,
}

const NOTIFICATION_VISIBILITY_SECONDS: f64 = 10.0;

impl<'s> System<'s> for NotificationBarSystem {
    type SystemData = (
        WriteExpect<'s, NotificationBar>,
        WriteStorage<'s, UiText>,
        ReadExpect<'s, EventChannel<PlayerNotification>>,
        ReadExpect<'s, Time>
        );

    fn run(&mut self, (mut bar, mut ui_texts, notifications, time): Self::SystemData)
    {
        let current_time = time.absolute_time_seconds();
        for notification in notifications.read(&mut self.notification_reader) {
            bar.show_message(&notification.message, &mut ui_texts, current_time + NOTIFICATION_VISIBILITY_SECONDS);
        }

        if current_time > bar.visible_until_sec {
            bar.hide(&mut ui_texts);
        }
    }
}
