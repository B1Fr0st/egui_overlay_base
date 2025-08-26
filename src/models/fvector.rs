#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
pub struct FVector {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}
impl FVector {
    pub fn sub(&self, other: &FVector) -> FVector {
        FVector {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl FVector {
    pub fn magnitude(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2) + self.z.powi(2)).sqrt()
    }

    pub fn distance(self, other: FVector) -> f64 {
        (self - other).magnitude()
    }

    pub fn distance_meter(self, other: FVector) -> f64 {
        self.distance(other) / 100.0
    }

    pub fn dot(&self, other: &FVector) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn cross(&self, other: &FVector) -> FVector {
        FVector {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }

    pub fn normalize(&self) -> FVector {
        let mag = self.magnitude();
        Self {
            x: self.x / mag,
            y: self.y / mag,
            z: self.z / mag,
        }
    }
    // Add to_matrix
    pub fn to_matrix(&self) -> [[f64; 4]; 4] {
        // Skidded.
        let origin = FVector {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        };

        let rad_pitch = self.x * std::f64::consts::PI / 180.0;
        let rad_yaw = self.y * std::f64::consts::PI / 180.0;
        let rad_roll = self.z * std::f64::consts::PI / 180.0;

        let sp = rad_pitch.sin();
        let cp = rad_pitch.cos();
        let sy = rad_yaw.sin();
        let cy = rad_yaw.cos();
        let sr = rad_roll.sin();
        let cr = rad_roll.cos();

        [
            [cp * cy, cp * sy, sp, 0.0],
            [
                sr * sp * cy - cr * sy,
                sr * sp * sy + cr * cy,
                -sr * cp,
                0.0,
            ],
            [
                -(cr * sp * cy + sr * sy),
                cy * sr - cr * sp * sy,
                cr * cp,
                0.0,
            ],
            [origin.x, origin.y, origin.z, 1.0],
        ]
    }
}

impl std::ops::Sub for FVector {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl std::ops::Div for FVector {
    type Output = Self;
    fn div(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
            z: self.z / rhs.z,
        }
    }
}

impl std::ops::Add for FVector {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl std::ops::Neg for FVector {
    type Output = FVector;

    fn neg(self) -> Self::Output {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}