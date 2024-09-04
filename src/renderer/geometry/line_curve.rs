use crate::renderer::*;

///
/// A rectangle 2D geometry which can be rendered using a camera created by [Camera::new_2d].
///
pub struct LineCurve {
    mesh: Mesh,
    thickness: f32,
}

impl LineCurve {
    ///
    /// Constructs a new line geometry.
    ///
    pub fn new(context: &Context, points: &[Vec3], thickness: f32) -> Self {
        let mut mesh = CpuMesh::line_curve(points, thickness);
        let mut line_curve = Self {
            mesh: Mesh::new(context, &mesh),
            thickness,
        };
        line_curve
    }
}

impl<'a> IntoIterator for &'a LineCurve {
    type Item = &'a dyn Geometry;
    type IntoIter = std::iter::Once<&'a dyn Geometry>;

    fn into_iter(self) -> Self::IntoIter {
        std::iter::once(self)
    }
}

use std::ops::Deref;
impl Deref for LineCurve {
    type Target = Mesh;
    fn deref(&self) -> &Self::Target {
        &self.mesh
    }
}

impl std::ops::DerefMut for LineCurve {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.mesh
    }
}

impl Geometry for LineCurve {
    impl_geometry_body!(deref);

    fn animate(&mut self, time: f32) {
        self.mesh.animate(time)
    }
}
