use crate::models::fvector::FVector;
#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
pub struct FQuat {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: f64,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
pub struct FMatrix {
    pub m: [[f64; 4]; 4],
}

impl FMatrix {
    pub fn multiply(&self, other: &FMatrix) -> FMatrix {
        let mut result = FMatrix { m: [[0.0; 4]; 4] };

        for row in 0..4 {
            for col in 0..4 {
                result.m[row][col] = self.m[row][0] * other.m[0][col]
                    + self.m[row][1] * other.m[1][col]
                    + self.m[row][2] * other.m[2][col]
                    + self.m[row][3] * other.m[3][col];
            }
        }

        result
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
pub struct FTransform {
    pub rotation: FQuat,
    pub translation: FVector,
    _pad0: [u8; 0x8],
    pub scale3d: FVector,
    _pad1: [u8; 0x8],
}

impl FTransform {
    pub fn to_matrix_with_scale(self) -> FMatrix {
        let mut matrix = FMatrix { m: [[0.0; 4]; 4] };

        matrix.m[3][0] = self.translation.x as f64;
        matrix.m[3][1] = self.translation.y as f64;
        matrix.m[3][2] = self.translation.z as f64;

        let x2 = self.rotation.x + self.rotation.x;
        let y2 = self.rotation.y + self.rotation.y;
        let z2 = self.rotation.z + self.rotation.z;

        let xx2 = self.rotation.x * x2;
        let yy2 = self.rotation.y * y2;
        let zz2 = self.rotation.z * z2;

        matrix.m[0][0] = (1.0 - (yy2 + zz2)) * self.scale3d.x as f64;
        matrix.m[1][1] = (1.0 - (xx2 + zz2)) * self.scale3d.y as f64;
        matrix.m[2][2] = (1.0 - (xx2 + yy2)) * self.scale3d.z as f64;

        let yz2 = self.rotation.y * z2;
        let wx2 = self.rotation.w * x2;
        matrix.m[2][1] = (yz2 - wx2) * self.scale3d.z as f64;
        matrix.m[1][2] = (yz2 + wx2) * self.scale3d.y as f64;

        let xy2 = self.rotation.x * y2;
        let wz2 = self.rotation.w * z2;
        matrix.m[1][0] = (xy2 - wz2) * self.scale3d.x as f64;
        matrix.m[0][1] = (xy2 + wz2) * self.scale3d.y as f64;

        let xz2 = self.rotation.x * z2;
        let wy2 = self.rotation.w * y2;
        matrix.m[2][0] = (xz2 + wy2) * self.scale3d.x as f64;
        matrix.m[0][2] = (xz2 - wy2) * self.scale3d.z as f64;

        matrix.m[0][3] = 0.0;
        matrix.m[1][3] = 0.0;
        matrix.m[2][3] = 0.0;
        matrix.m[3][3] = 1.0;

        matrix
    }
}