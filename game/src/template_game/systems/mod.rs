
use std::f32::consts::PI;
use specs::{Join, Read, ReadStorage, WriteStorage, System};

use goblin::engine::components::*;
use goblin::engine::resources::*;
use goblin::engine::input::{Mouse, EventType, KeyBoard};
use goblin::math::Vert3;

// systems
pub struct Orbit;

impl<'a> System<'a> for Orbit {
    type SystemData = (Read<'a, DeltaTime>, WriteStorage<'a, Transform>, ReadStorage<'a, Velocity>);

    fn run(&mut self, (delta, mut pos, vel): Self::SystemData) {

        // Read implements DeRef
        let delta = delta.0;

        for (pos, vel) in (&mut pos, &vel).join() {
            ()
        }
    }
}

pub struct UpdateCamera {
    pub mouse: Mouse,
    pub event: EventType,
}

impl<'a> System<'a> for UpdateCamera {
    type SystemData = (Read<'a, DeltaTime>, WriteStorage<'a, Camera>, ReadStorage<'a, PlayerController>);

    fn run(&mut self, (delta, mut c_storage, pc_storage): Self::SystemData) {

        let delta = delta.0;

        for camera in (&mut c_storage).join() {
            if self.event == EventType::Move && self.mouse.left() {
                camera.yaw -= self.mouse.move_x() * delta;
                camera.pitch += self.mouse.move_y() * delta;

                if camera.pitch > PI / 2.0 - 0.1 {
                    camera.pitch = PI / 2.0 - 0.1;
                } else if camera.pitch < -PI / 2.0 + 0.1 {
                    camera.pitch = -PI / 2.0 + 0.1;
                }
            }
            else if self.event == EventType::Scroll {
                let s = self.mouse.move_s();

                camera.pole_arm += s/s.abs();

                if camera.pole_arm < 0.1 {
                    camera.pole_arm = 0.1;
                }
            }
            camera.update();
        }
    }
}

pub struct RunInput;

impl<'a> System<'a> for RunInput {
    type SystemData = (
        Read<'a, KeyBoard>,
        WriteStorage<'a, Velocity>,
        ReadStorage<'a, Camera>,
        ReadStorage<'a, PlayerController>
    );

    fn run(&mut self, (board, mut v_storage, c_storage, pc_storage): Self::SystemData) {
        let sprint: f32 = if board[16] { 2.5 } else { 1.0 };
        for (vel, camera, _) in (&mut v_storage, &c_storage, &pc_storage).join() {
            let forward : Vert3
                = camera.rotation.normalize();
            let right : Vert3
                = camera.rotation.cross(&Vert3::new(0.0, 1.0, 0.0)).normalize();

            vel.position = Vert3::zero();

            if board[87] {
                vel.position -= forward * 5.0 * sprint;
            }
            if board[83] {
                vel.position += forward * 5.0 * sprint;
            }
            if board[65] {
                vel.position += right * 5.0 * sprint;
            }
            if board[68] {
                vel.position -= right * 5.0 * sprint;
            }

            //vel.position.normalize();

            //pos.rotation = vel.position.normalize();
        }
    }
}