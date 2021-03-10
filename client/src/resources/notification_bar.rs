use amethyst::ui::{Anchor, LineMode, TtfFormat, UiText, UiTransform};
use amethyst::ecs::prelude::Entity;
use amethyst::ecs::prelude::*;
use amethyst::assets::{Loader};

pub struct NotificationBar {
    pub messages: Entity,
    pub visible_until_sec: f64,

}

impl NotificationBar
{
    pub fn show_message(&mut self, message: &String, ui_texts: &mut WriteStorage<UiText>, visible_until: f64)
    {
        if let Some(messages) = ui_texts.get_mut(self.messages) {
            if !messages.text.is_empty()
            {
                messages.text += "\n";
            }
            messages.text += message;
            self.visible_until_sec = visible_until;
        }
    }

    pub fn hide(&self, ui_texts: &mut WriteStorage<UiText>)
    {
        if let Some(messages) = ui_texts.get_mut(self.messages) {
            messages.text = "".to_string();
        }
    }

    pub fn initialize(world: &mut World)
    {
        let font = {
            let loader = world.read_resource::<Loader>();
            loader.load("fonts/square.ttf", TtfFormat, (), &world.read_resource())
        };

        let notification_bar_transform = UiTransform::new(
            "NotificationBar".to_string(),
            Anchor::BottomLeft,
            Anchor::BottomLeft,
            10., 10., 1.,
            400., 150.,
            );


        let notification_bar = world.create_entity()
            .with(notification_bar_transform)
            .with(UiText::new(
                    font.clone(),
                    "".to_string(),
                    [1., 1., 1., 0.8], // color
                    18., // font size
                    LineMode::Wrap,
                    Anchor::BottomLeft)
                ).build();

        world.insert(NotificationBar{messages: notification_bar, visible_until_sec: 0.0});
    }
}

