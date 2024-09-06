use crate::renderer::*;
use std::ops::{Add, Mul, Sub};

///
/// A control that makes the camera fly through the 3D scene.
///
pub struct FlyControl {
    distance_to_ground: f32,
    direction: Vec3,
    // control: CameraControl,
}

impl FlyControl {
    /// Creates a new fly control with the given speed of movements.
    pub fn new(speed: f32) -> Self {
        Self {
            distance_to_ground: 0.0,
            // control: CameraControl {
            //     left_drag_horizontal: CameraAction::Yaw {
            //         speed: std::f32::consts::PI / 1800.0,
            //     },
            //     left_drag_vertical: CameraAction::Pitch {
            //         speed: std::f32::consts::PI / 1800.0,
            //     },
            //     scroll_vertical: CameraAction::Forward { speed },
            //     right_drag_horizontal: CameraAction::Left { speed },
            //     right_drag_vertical: CameraAction::Up { speed },
            //     ..Default::default()
            // },
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
                            MouseButton::Left => {}
                            MouseButton::Right => {
                                // 鼠标右键 移动
                                self.move_(camera, delta.0, delta.1);
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
                Event::KeyPress {
                    kind,
                    modifiers: _modifiers,
                    handled: _handled,
                } => {
                    let speed = 0.1;
                    match kind {
                        Key::ArrowDown => {
                            self.move_(camera, 0.0, -speed * self.distance_to_ground);
                        }
                        Key::ArrowLeft => {
                            self.move_(camera, speed * self.distance_to_ground, 0.0);
                        }
                        Key::ArrowRight => {
                            self.move_(camera, -speed * self.distance_to_ground, 0.0);
                        }
                        Key::ArrowUp => {
                            self.move_(camera, 0.0, speed * self.distance_to_ground);
                        }

                        Key::A => {
                            camera.yaw(degrees(5.0)); // camera围绕 自己的y轴自旋转, 从用户视角上就是左右转
                        }
                        Key::D => {
                            camera.yaw(degrees(-5.0)); // camera围绕 自己的x轴自旋转, 从用户视角上就是上下转
                        }
                        Key::S => {
                            self.zoom_(camera, 1.0 / 0.9);
                        }
                        Key::W => {
                            self.zoom_(camera, 0.9);
                        }
                        _ => {}
                    }
                }
                _ => {}
            }
        }
        return false;
        // self.control.handle_events(camera, events)
    }

    fn update_self(&mut self, camera: &mut Camera) {
        let r = camera.target().sub(camera.position().clone());
        self.direction = r.normalize();
        self.distance_to_ground = (camera.position().z / self.direction.z).abs();
    }


    /// 缩放  camera到target的距离
    fn zoom_(&mut self, camera: &mut Camera, scale: f32) {
        let new_distance_to_target = (self.distance_to_ground * scale)
            .max(camera.z_near())
            .min(camera.z_far());

        let new_position = camera
            .target()
            .sub(self.direction.mul(new_distance_to_target));
        let target = camera
            .position()
            .add(self.direction.mul(self.distance_to_ground)); // target 是camera方向射向地面上的点
        let up = camera.up().clone();
        camera.set_view(new_position, target, up);
    }

    /// 从相机的z轴 垂直面上 上下左右移动相机
    fn move_(&mut self, camera: &mut Camera, move_x: f32, move_y: f32) {
        let up = camera.up().clone();
        let camera_right = up.cross(self.direction.clone()).normalize();

        let delta = camera_right.mul(move_x).add(up.mul(move_y));
        let new_position = camera.position().add(delta);
        let new_target = camera.target().add(delta);

        camera.set_view(new_position, new_target, up);
    }
}
