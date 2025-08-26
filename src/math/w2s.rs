use crate::models::{fvector::FVector,vector2::Vector2, cameramodels::FMinimalViewInfo};
pub fn world2screen(position: FVector, pov: FMinimalViewInfo, window_size:[u32;2]) -> Vector2 {
    let view_matrix = pov.rotation.to_matrix();

    let v_axis_x = FVector {
        x: view_matrix[0][0],
        y: view_matrix[0][1],
        z: view_matrix[0][2],
    };
    let v_axis_y = FVector {
        x: view_matrix[1][0],
        y: view_matrix[1][1],
        z: view_matrix[1][2],
    };
    let v_axis_z = FVector {
        x: view_matrix[2][0],
        y: view_matrix[2][1],
        z: view_matrix[2][2],
    };

    let v_delta = position - pov.location;
    let v_transformed = FVector {
        x: v_delta.dot(&v_axis_y),
        y: v_delta.dot(&v_axis_z),
        z: v_delta.dot(&v_axis_x),
    };

    if v_transformed.z < 1f64 {
        // return
        return Vector2 { x: 0.0, y: 0.0 };
    }

    let fov = pov.fov;
    let screen_center_x = window_size[0] as f64 / 2.0;
    let screen_center_y = window_size[1] as f64 / 2.0;
    let fov_rad = (fov as f32) * std::f32::consts::PI / 360.0;
    let tan_fov = fov_rad.tan();
    let screenpos = Vector2 {
        x: screen_center_x as f32
            + (v_transformed.x as f32) * (screen_center_x as f32 / tan_fov)
                / (v_transformed.z as f32),
        y: screen_center_y as f32
            - (v_transformed.y as f32) * (screen_center_x as f32 / tan_fov)
                / (v_transformed.z as f32),
    };

    return screenpos;
}