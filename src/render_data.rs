//! Data structures for use by a renderer.

use std::ops::Mul;

pub const DEFAULT_COLOR: [f32; 3] = [1.0, 0.0, 0.0];

// --------------------
// Vertex, mesh, scene.

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct GpuVertex {
    pub position: [f32; 3],
    pub color: [f32; 3],
    pub normal: [f32; 3],
    pub tex_coords: [f32; 2],
}

impl Default for GpuVertex {
    fn default() -> Self {
        Self {
            position: [0.0, 0.0, 0.0],
            color: [1.0, 0.0, 1.0],
            normal: [0.0, 0.0, 0.0],
            tex_coords: [0.0, 0.0],
        }
    }
}

#[derive(Default)]
pub struct RenderMesh {
    pub vertices: Vec<GpuVertex>,
    pub indices: Vec<u32>,
    pub matrix: MatrixUniform,
}

#[derive(Default)]
pub struct RenderScene {
    pub meshes: Vec<RenderMesh>,
}

// -------
// Matrix.

pub const X_AXIS: cgmath::Vector3<f32> = cgmath::Vector3::new(1.0, 0.0, 0.0);
pub const Y_AXIS: cgmath::Vector3<f32> = cgmath::Vector3::new(0.0, 1.0, 0.0);

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct MatrixUniform {
    matrix: [[f32; 4]; 4],
}

impl From<[[f32; 4]; 4]> for MatrixUniform {
    fn from(value: [[f32; 4]; 4]) -> Self {
        Self { matrix: value }
    }
}

impl Mul for MatrixUniform {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        let cg_self: cgmath::Matrix4<_> = self.matrix.into();
        let cg_other: cgmath::Matrix4<_> = rhs.matrix.into();
        Self {
            matrix: (cg_self * cg_other).into(),
        }
    }
}

impl MatrixUniform {
    #[allow(unused)]
    pub fn identity() -> Self {
        use cgmath::SquareMatrix;
        Self {
            matrix: cgmath::Matrix4::identity().into(),
        }
    }

    pub fn from(matrix: cgmath::Matrix4<f32>) -> Self {
        Self {
            matrix: matrix.into(),
        }
    }

    pub fn translation(coords: &[f32]) -> Self {
        Self {
            matrix: cgmath::Matrix4::from_translation(cgmath::Vector3 {
                x: coords[0],
                y: coords[1],
                z: coords[2],
            })
            .into(),
        }
    }

    pub fn x_rotation(degrees: f32) -> Self {
        Self {
            matrix: cgmath::Matrix4::from_axis_angle(X_AXIS, cgmath::Deg(degrees)).into(),
        }
    }

    pub fn update(&mut self, matrix: cgmath::Matrix4<f32>) {
        self.matrix = matrix.into();
    }
}

impl Default for MatrixUniform {
    fn default() -> Self {
        Self::identity()
    }
}
