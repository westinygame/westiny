use crate::resources::StreamId;

use westiny_common::components::{self, InputFlags};
use westiny_common::resources::ServerAddress;
use westiny_common::{metric_dimension::length::MeterVec2, network, serialization::serialize};
use crate::systems::camera::PlayCamera;

use bevy::input::{keyboard::KeyCode, mouse::MouseButton};
use bevy::prelude::*;
use bevy::render::camera::RenderTarget;
use blaminar::prelude::*;

#[derive(Copy, Clone)]
enum Control {
    Keyboard(KeyCode),
    Mouse(MouseButton)
}

use Control::*;

const INPUT_FLAG_MAPPING : [(InputFlags, Control); 13] = [
    (InputFlags::FORWARD,  Keyboard(KeyCode::W)),
    (InputFlags::BACKWARD, Keyboard(KeyCode::S)),
    (InputFlags::LEFT,     Keyboard(KeyCode::A)),
    (InputFlags::RIGHT,    Keyboard(KeyCode::D)),
    (InputFlags::SHOOT,    Mouse(MouseButton::Left)),
    (InputFlags::USE,      Mouse(MouseButton::Right)),
    (InputFlags::RUN,      Keyboard(KeyCode::LShift)),
    (InputFlags::RELOAD,   Keyboard(KeyCode::R)),
    (InputFlags::SELECT1,  Keyboard(KeyCode::Key1)),
    (InputFlags::SELECT2,  Keyboard(KeyCode::Key2)),
    (InputFlags::SELECT3,  Keyboard(KeyCode::Key3)),
    (InputFlags::SELECT4,  Keyboard(KeyCode::Key4)),
    (InputFlags::SELECT5,  Keyboard(KeyCode::Key5)),
];

fn update_input_keys(
    input: &mut components::Input,
    keyboard_input: &Input<KeyCode>,
    mouse_btn_input: &Input<MouseButton>,
) {
    INPUT_FLAG_MAPPING.iter()
        .map(|(flag, control)| (flag, is_held_down(*control, keyboard_input, mouse_btn_input)))
        .for_each(|(flag, pressed)| input.flags.set(*flag, pressed));
}

fn is_held_down(
    control: Control,
    keyboard_input: &Input<KeyCode>,
    mouse_input: &Input<MouseButton>
) -> bool {
    match control {
        Keyboard(key_code) => keyboard_input.pressed(key_code),
        Mouse(button) => mouse_input.pressed(button),
    }
}

fn update_cursor_position(
    input: &mut components::Input,
    windows: &Windows,
    camera: &Camera,
    camera_transform: &GlobalTransform,
) {
    if let RenderTarget::Window(window_id) = camera.target {
        let wnd = windows.get(window_id).unwrap();

        if let Some(screen_pos) = wnd.cursor_position() {
            let window_size = Vec2::new(wnd.width() as f32, wnd.height() as f32);
            let ndc = (screen_pos / window_size) * 2.0 - Vec2::ONE;

            // matrix for undoing the projection and camera transform
            let ndc_to_world = camera_transform.compute_matrix() * camera.projection_matrix().inverse();

            // use it to convert ndc to world-space coordinates
            let world_pos = ndc_to_world.project_point3(ndc.extend(-1.0)).truncate();

            input.cursor = MeterVec2::from_pixel_vec(world_pos);
        }
    }
}

pub fn handle_user_inputs(
    mut input_qry: Query<&mut components::Input>,
    camera_query: Query<(&Camera, &GlobalTransform), With<PlayCamera>>,
    keyboard_input: Res<Input<KeyCode>>,
    mouse_button_input: Res<Input<MouseButton>>,
    windows: Res<Windows>,
    server_address: Res<ServerAddress>,
    mut net: ResMut<TransportResource>,
) {
    // NOTE: Only one Input component exists on the client
    if let Some(mut input) = input_qry.iter_mut().next() {
        update_input_keys(&mut input, &keyboard_input, &mouse_button_input);

        let (camera, camera_transform) = camera_query.single();
        update_cursor_position(&mut input, &windows, camera, camera_transform);

        send_to_server(&mut net, &server_address, &input);
    }
}

fn send_to_server(net: &mut TransportResource, server: &ServerAddress, input: &components::Input) {
    let message = serialize(&network::PacketType::InputState { input: *input })
        .expect("InputState could not be serialized");

    net.send_with_requirements(
        server.address,
        &message,
        DeliveryRequirement::UnreliableSequenced(StreamId::InputState.into()),
        UrgencyRequirement::OnTick,
    );
}
