use crate::renderer::*;
use cgmath::Array;
use std::ops::{Add, Mul, Sub};
use three_d_asset::{PixelPoint, ProjectionType};

///
/// A control that makes the camera follow a target.
///
pub struct FollowerControl {
    distance_to_target: f32,
    direction: Vec3,
}

impl FollowerControl {
    /// Creates a new first person control with the given speed of movements.
    pub fn new() -> Self {
        Self {
            distance_to_target: 0.0,
            direction: Vector3::new(0.0, 0.0, -1.0),
        }
    }

    /// Handles the events. Must be called each frame.
    pub fn handle_events(&mut self, camera: &mut Camera, events: &mut [Event]) -> bool {
        for event in events {
            self.update_self(camera);
            match event {
                Event::MouseMotion {
                    button,
                    delta,
                    position: _position,
                    modifiers: _modifiers,
                    handled: _handled,
                } => {
                    if let Some(button) = button {
                        match button {
                            MouseButton::Left => {
                                // 鼠标右键 旋转视角, 保持target不变
                                self.yaw_and_pitch(camera, delta.0, delta.1);
                            }
                            _ => {}
                        }
                    }
                }
                Event::MouseWheel {
                    delta,
                    position: _position,
                    modifiers: _modifiers,
                    handled: _handled,
                } => {
                    // 缩放后.
                    let scale = if delta.1 > 0.0 { 0.8 } else { 1.2 };
                    self.zoom_(camera, scale);
                }
                Event::KeyPress { .. } => {}
                _ => {}
            }
        }

        return true;
    }

    fn update_self(&mut self, camera: &mut Camera) {
        let r = camera.target().sub(camera.position().clone());
        self.direction = r.normalize();
        self.distance_to_target = r.magnitude();
    }

    /// 改变 从camera到target的方向
    fn yaw_and_pitch(&mut self, camera: &mut Camera, yaw: f32, pitch: f32) {
        camera.yaw(degrees(yaw));
        camera.pitch(degrees(pitch));
    }

    /// 缩放  camera到target的距离
    fn zoom_(&mut self, camera: &mut Camera, scale: f32) {
        // 经过缩放后. camera到target的距离
        let new_distance_to_target = (self.distance_to_target * scale)
            .max(camera.z_near())
            .min(camera.z_far());

        let new_position = camera
            .target()
            .sub(self.direction.mul(new_distance_to_target));
        let target = camera.target().clone();
        let up = camera.up().clone();
        camera.set_view(new_position, target, up);
    }
}
