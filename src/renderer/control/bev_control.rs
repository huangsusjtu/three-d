use crate::renderer::*;
use three_d_asset::{PixelPoint, ProjectionType};

///
/// MapControls is intended for bird's eye perspective.
/// 1. 鼠标右键+移动 = 场景移动
/// 2. 鼠标滚轮 = 缩放
///
pub struct BevControl {
    min_distance: f32,
    max_distance: f32,
    ratio: f32, // 一个像素代表的物理单位 的比例
}

impl BevControl {
    /// Creates a new orbit control with the given target and minimum and maximum distance to the target.
    pub fn new(min_distance: f32, max_distance: f32) -> Self {
        Self {
            min_distance,
            max_distance,
            ratio: 0.0,
        }
    }

    /// Handles the events. Must be called each frame.
    pub fn handle_events(&mut self, camera: &mut Camera, events: &mut [Event]) -> bool {
        {
            let c_type = camera.projection_type().clone();
            let physic_height = match c_type {
                // 相机视图里代表的物理真实高度
                ProjectionType::Orthographic { height } => height,
                ProjectionType::Perspective { .. } => {
                    return false;
                }
            };
            self.ratio = physic_height / camera.viewport().height as f32; // 像素单位对应的真实物理单位
        }

        for event in events {
            match event {
                Event::MousePress { .. } => {}
                Event::MouseRelease { .. } => {}
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
                                // todo
                            }
                            MouseButton::Right => {
                                // 鼠标右键 滑动视窗
                                self.move_(camera, -delta.0 * self.ratio, delta.1 * self.ratio);
                            }
                            MouseButton::Middle => {}
                        }
                    }
                }
                Event::MouseWheel {
                    delta,
                    position, // 左下角是原点
                    modifiers: _modifiers,
                    handled: _handled,
                } => {
                    // println!("MouseWheel {:?}", delta);
                    // 经过缩放后. 单个像素表示物理单位,  scale变小 表示在显示上要放大元素
                    let scale = if delta.1 > 0.0 { 0.8 } else { 1.2 };
                    self.zoom_(camera, position, scale);
                }
                Event::MouseEnter => {}
                Event::MouseLeave => {}
                Event::KeyPress {
                    kind,
                    modifiers: _modifiers,
                    handled: _handled,
                } => match kind {
                    Key::ArrowDown => {}
                    Key::ArrowLeft => {}
                    Key::ArrowRight => {}
                    Key::ArrowUp => {}
                    Key::A => {}
                    Key::D => {}
                    Key::S => {}
                    Key::W => {}
                    _ => {}
                },
                Event::KeyRelease { .. } => {}
                Event::ModifiersChange { .. } => {}
                Event::Text(_) => {}
            }
        }

        return true;
    }

    /// 从bev视角上移动相机
    fn move_(&mut self, camera: &mut Camera, move_x: f32, move_y: f32) {
        // 鼠标右键 滑动视窗
        let new_position = Vec3::new(
            camera.position().x + move_x,
            camera.position().y + move_y,
            camera.position().z,
        );
        let new_target = Vec3::new(
            camera.target().x + move_x,
            camera.target().y + move_y,
            camera.target().z,
        );
        let up = camera.up().clone();
        camera.set_view(new_position, new_target, up);
    }

    /// 从bev视角上缩放 相机, 以当前鼠标点为中心缩放
    fn zoom_(&mut self, camera: &mut Camera, position: &mut PixelPoint, scale: f32) {
        // 经过缩放后. 单个像素表示物理单位,  scale变小 表示在显示上要放大元素
        let new_ratio = self.ratio * scale;
        let (delta_screen_x, delta_screen_y) = (
            camera.viewport().width as f32 / 2.0 - position.x,
            camera.viewport().height as f32 / 2.0 - position.y,
        ); // UI中心点的y和鼠标点的y的差值
        let (delta_x, delta_y) = (
            delta_screen_x * (new_ratio - self.ratio),
            delta_screen_y * (new_ratio - self.ratio),
        ); // 对应到物理世界的 x, y的差值

        let new_position = Vec3::new(
            camera.position().x + delta_x,
            camera.position().y + delta_y,
            camera.position().z,
        );
        let new_target = Vec3::new(
            camera.target().x + delta_x,
            camera.target().y + delta_y,
            camera.target().z,
        );
        let up = camera.up().clone();

        camera.set_view(new_position, new_target, up);
        let physic_height = self.ratio * camera.viewport().height as f32;
        camera.set_orthographic_projection(
            physic_height * scale,
            self.min_distance,
            self.max_distance,
        );
    }
}
