use crate::renderer::*;
use three_d_asset::ProjectionType;

///
/// MapControls is intended for transforming a camera over a map from bird's eye perspective.
///
pub struct MapControl {
    min_distance: f32,
    max_distance: f32,
}

impl MapControl {
    /// Creates a new orbit control with the given target and minimum and maximum distance to the target.
    pub fn new(min_distance: f32, max_distance: f32) -> Self {
        Self {
            min_distance,
            max_distance,
        }
    }

    /// Handles the events. Must be called each frame.
    pub fn handle_events(&mut self, camera: &mut Camera, events: &mut [Event]) -> bool {
        let c_type = camera.projection_type().clone();
        let physic_height = match c_type {
            // 相机视图里代表的物理真实高度
            ProjectionType::Orthographic { height } => height,
            ProjectionType::Perspective { .. } => {
                return false;
            }
        };
        let zoom = physic_height / camera.viewport().height as f32; // 像素单位对应的真实物理单位

        for event in events {
            match event {
                Event::MousePress { .. } => {}
                Event::MouseRelease { .. } => {}
                Event::MouseMotion {
                    button,
                    delta,
                    position,
                    modifiers,
                    handled,
                } => {
                    if let Some(button) = button {
                        match button {
                            MouseButton::Left => {}
                            MouseButton::Right => {
                                // 鼠标右键 滑动视窗
                                let new_position = Vec3::new(
                                    camera.position().x - delta.0 * zoom,
                                    camera.position().y + delta.1 * zoom,
                                    camera.position().z,
                                );
                                let new_target = Vec3::new(
                                    camera.target().x - delta.0 * zoom,
                                    camera.target().y + delta.1 * zoom,
                                    camera.target().z,
                                );
                                let up = camera.up().clone();
                                camera.set_view(new_position, new_target, up);
                            }
                            MouseButton::Middle => {}
                        }
                    }
                }
                Event::MouseWheel {
                    delta,
                    position, // 左下角是原点
                    modifiers,
                    handled,
                } => {
                    // println!("MouseWheel {:?}", delta);
                    // 经过缩放后. 单个像素表示物理单位,  scale变小 表示在显示上要放大元素
                    let scale = if delta.1 > 0.0 { 0.8 } else { 1.2 };
                    let new_zoom = zoom * scale;
                    let (delta_screen_x, delta_screen_y) = (
                        camera.viewport().width as f32 / 2.0 - position.x,
                        camera.viewport().height as f32 / 2.0 - position.y,
                    ); // UI中心点的y和鼠标点的y的差值
                    let (delta_x, delta_y) = (
                        delta_screen_x * (new_zoom - zoom),
                        delta_screen_y * (new_zoom - zoom),
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
                    camera.set_orthographic_projection(
                        physic_height * scale,
                        self.min_distance,
                        self.max_distance,
                    );
                }
                Event::MouseEnter => {}
                Event::MouseLeave => {}
                Event::KeyPress { .. } => {}
                Event::KeyRelease { .. } => {}
                Event::ModifiersChange { .. } => {}
                Event::Text(_) => {}
            }
        }

        return true;
    }
}
