use bevy::{math::Vec3A, prelude::*, render::camera::CameraProjection};

#[derive(Component, Debug, Clone, Reflect)]
#[reflect(Component, Default)]
pub struct WebXrProjection {
    pub projection_matrix: Mat4,
}

fn transform_matrix(matrix: &mut Mat4) {
    matrix.y_axis.y = -matrix.y_axis.y;
}

impl From<Vec<f32>> for WebXrProjection {
    fn from(value: Vec<f32>) -> Self {
        let mut projection_matrix = Mat4::from_cols_slice(&value);
        transform_matrix(&mut projection_matrix);
        WebXrProjection { projection_matrix }
    }
}

impl Default for WebXrProjection {
    fn default() -> Self {
        Self {
            //default perspective matrix
            projection_matrix: Mat4 {
                x_axis: Vec4::new(2.4142134, 0.0, 0.0, 0.0),
                y_axis: Vec4::new(0.0, 2.4142134, 0.0, 0.0),
                z_axis: Vec4::new(0.0, 0.0, 0.0, -1.0),
                w_axis: Vec4::new(0.0, 0.0, 0.1, 0.0),
            },
        }
    }
}

impl WebXrProjection {
    pub fn update_matrix(&mut self, value: Vec<f32>) {
        let mut matrix = Mat4::from_cols_slice(&value);
        transform_matrix(&mut matrix);
        self.projection_matrix = matrix;
    }
}

impl CameraProjection for WebXrProjection {
    fn get_projection_matrix(&self) -> Mat4 {
        self.projection_matrix
    }

    /// This projection gets updated by the update_xr_cameras system.
    fn update(&mut self, width: f32, height: f32) {}

    fn far(&self) -> f32 {
        let far = self.projection_matrix.to_cols_array()[14]
            / (self.projection_matrix.to_cols_array()[10] + 1.0);

        //let c = self.projection_matrix.y_axis.y;

        //1. / c

        //far

        1000.0
    }

    // fn get_frustum_corners(&self, z_near: f32, z_far: f32) -> [bevy::math::Vec3A; 8] {
    //     let ndc_corners = [
    //         Vec3A::new(1.0, -1.0, 1.0),   // Bottom-right far
    //         Vec3A::new(1.0, 1.0, 1.0),    // Top-right far
    //         Vec3A::new(-1.0, 1.0, 1.0),   // Top-left far
    //         Vec3A::new(-1.0, -1.0, 1.0),  // Bottom-left far
    //         Vec3A::new(1.0, -1.0, -1.0),  // Bottom-right near
    //         Vec3A::new(1.0, 1.0, -1.0),   // Top-right near
    //         Vec3A::new(-1.0, 1.0, -1.0),  // Top-left near
    //         Vec3A::new(-1.0, -1.0, -1.0), // Bottom-left near
    //     ];

    //     let mut view_space_corners = [Vec3A::ZERO; 8];
    //     let inverse_matrix = self.projection_matrix.inverse();
    //     for (i, corner) in ndc_corners.into_iter().enumerate() {
    //         view_space_corners[i] = inverse_matrix.transform_point3a(corner);
    //     }

    //     view_space_corners
    // }
}
