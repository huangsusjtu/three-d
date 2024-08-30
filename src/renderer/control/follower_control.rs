use crate::renderer::*;
use three_d_asset::ProjectionType;

///
/// A control that makes the camera follow a target.
///
pub struct FollowerControl {}

impl FollowerControl {
    /// Creates a new first person control with the given speed of movements.
    pub fn new() -> Self {
        Self {}
    }

    /// Handles the events. Must be called each frame.
    pub fn handle_events(&mut self, camera: &mut Camera, events: &mut [Event]) -> bool {
        let c_type = camera.projection_type().clone();
        let physic_height = match c_type {
            // 相机视图里代表的物理真实高度
            ProjectionType::Orthographic { .. } => return false,
            ProjectionType::Perspective { .. } => {
                return false;
            }
        };

        return true;
    }
}
