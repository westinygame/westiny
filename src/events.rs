use std::vec::Vec;
use piston::input::{Event, Button, PressEvent};

pub struct EventHandler {
}

impl EventHandler {
    pub fn handle_event(&self, ev: &Event) {
        match ev.press_args() {
            Some(Button::Mouse(button)) => println!("Mouse button {:?}", button),
            _ => {}
        }
    }
}