use crate::models::{fvector::FVector,frotator::FRotator};
#[derive(Debug, Clone, Copy, Default)]
#[repr(C)]
pub struct FMinimalViewInfo {
    pub location: FVector,
    pub rotation: FRotator,
    pub fov: f32,
}

#[derive(Debug, Clone, Copy, Default)]
#[repr(C)]
pub struct FCameraCacheEntry {
    pub timestamp: f32,
    pub pov: FMinimalViewInfo,
}