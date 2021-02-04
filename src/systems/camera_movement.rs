use amethyst::input::InputHandler;
use amethyst::derive::SystemDesc;
use amethyst::ecs::{Read, System, SystemData, ReadStorage, WriteStorage};
use amethyst::core::Transform;
use amethyst::core::math::Vector3;

use amethyst::ecs::prelude::Join;
use amethyst::renderer::Camera;
use crate::components::Player;
use crate::systems::MovementBindingTypes;
use crate::systems::AxisBinding;


#[derive(SystemDesc)]
pub struct CameraMovementSystem;

const MIN_ZOOM: f32 = 1.0; // 1x (1:1 screenpixel:spritepixel)
const MAX_ZOOM: f32 = 8.0; // N:1 (N screen pixel will show 1 spritepixel)
const STEP: f32 = 1.0;

impl<'s> System<'s> for CameraMovementSystem {
    type SystemData = (
        WriteStorage<'s, Transform>,
        ReadStorage<'s, Camera>,
        Read<'s, InputHandler<MovementBindingTypes>>,
        ReadStorage<'s, Player>,
    );

    fn run(&mut self, (mut transforms, cameras, input, players): Self::SystemData) {

        let player_pos = (&players, &transforms).join().next()
            .map(|(_, transform)| transform)
            .map(Transform::translation)
            .map(&Vector3::xy);
        if let Some((_camera, transform)) = (&cameras, &mut transforms).join().next() {
            // Emulated axis value is from [-1, 1].
            // -1 when "neg" is pressed
            //  1 when "pos" is pressed
            //  0 when neither "neg", neither "pos" is pressed
            if let Some(zoom_direction) = input.axis_value(&AxisBinding::Zoom) {
                // Camera's transform scale is:
                // - 1.0  when screen-sprite pixel ratio is 1:1
                // - 0.25 when screen-sprite pixel ratio is 4:1
                let current_scale = transform.scale().x;
                let current_zoom = 1.0 / current_scale; // 0.25 -> 4x

                // 4x -> 5x and clamp:
                let raw_new_zoom = current_zoom - zoom_direction * STEP;
                let new_zoom = raw_new_zoom.min(MAX_ZOOM).max(MIN_ZOOM);
                let new_scale = 1.0 / new_zoom; // 5x -> 0.2

                transform.scale_mut().x = new_scale;
                transform.scale_mut().y = new_scale;
            }

            if let Some(player_coord) = player_pos {
                transform.set_translation_x(player_coord.x);
                transform.set_translation_y(player_coord.y);
            }
        }
    }
}
