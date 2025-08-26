use crate::models::{frotator::FRotator,fvector::FVector};
pub fn calculate_aim(camera_position: &FVector, target_position: &FVector, z_offset:f64) -> FRotator {
    let mut newtarget = target_position.clone();
    newtarget.z += z_offset;
    let vec_delta = camera_position.sub(&newtarget);
    let hyp = (vec_delta.x * vec_delta.x + vec_delta.y * vec_delta.y).sqrt();

    let mut view_angles = FRotator {
        pitch: -(vec_delta.z / hyp).atan() * (180.0 / 3.14159),
        yaw: (vec_delta.y / vec_delta.x).atan() * (180.0 / 3.14159),
        roll: 0.0,
    };

    if vec_delta.x >= 0.0 {
        view_angles.yaw += 180.0;
    }

    view_angles
}

pub fn smooth_angle(current: f64, target: f64, max_delta: f64) -> f64 {
    let mut delta = target - current;

    if delta > 180.0 {
        delta -= 360.0;
    } else if delta < -180.0 {
        delta += 360.0;
    }

    if delta.abs() > max_delta {
        current + max_delta * delta.signum()
    } else {
        current + delta
    }
}
pub fn normalize_angle(angle: f64) -> f64 {
    let normalized = angle % 360.0;
    if normalized < 0.0 {
        normalized + 360.0
    } else {
        normalized
    }
}