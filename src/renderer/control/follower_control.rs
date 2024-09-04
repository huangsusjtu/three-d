use crate::renderer::*;
use three_d_asset::ProjectionType;

///
/// A control that makes the camera follow a target.
///
pub struct FollowerControl {
    distance_to_target: f32,
    yaw : f32,
    pitch : f32,
}

impl FollowerControl {
    /// Creates a new first person control with the given speed of movements.
    pub fn new() -> Self {
        Self {
            distance_to_target: 10.0,
            yaw: 0.0,
            pitch: 0.0,
        }
    }

    /// Handles the events. Must be called each frame.
    pub fn handle_events(&mut self, camera: &mut Camera, events: &mut [Event]) -> bool {
        let c_type = camera.projection_type().clone();
        let field_of_view_y = match c_type {
            // 相机视图里代表的物理真实高度
            ProjectionType::Orthographic { .. } => return false,
            ProjectionType::Perspective { field_of_view_y } => {
                field_of_view_y
            }
        };



        return true;
    }
}
