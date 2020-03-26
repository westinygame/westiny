mod events;

use piston::window::WindowSettings;
use glutin_window::{OpenGL, GlutinWindow};
use std::io::{stdin, Read};
use piston::event_loop::{Events, EventSettings, EventLoop};
use piston::input::{RenderEvent, UpdateEvent, PressEvent, Button, MouseScrollEvent, Motion};
use events::EventHandler;

const OPENGL: OpenGL = OpenGL::V4_5;


fn main() {
    let mut window : GlutinWindow = WindowSettings::new("Botanique", [640, 480])
        .graphics_api(OPENGL)
        .exit_on_esc(true)
        .build()
        .unwrap();


    let event_handler = EventHandler{};
    let mut events = Events::new(EventSettings::new());
    while let Some(ev) = events.next(&mut window) {
        event_handler.handle_event(&ev);
    }
}
