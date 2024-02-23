use std::{f32::consts::PI, ops::Mul};


#[derive(Copy, Clone, Debug)]
pub struct AffineMatrix {
    pub matrix: [[f32; 4]; 4],
}

impl AffineMatrix {
    pub fn new() -> Self {
        Self::ident()
    }

    pub fn ident() -> Self {
        Self {
            matrix: [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ],
        }
    }

    pub fn translate(&mut self, x: f32, y: f32, z: f32) {
        self.matrix[3][0] += x;
        self.matrix[3][1] += y;
        self.matrix[3][2] += z;
    }

    pub fn set_translate(&mut self, x: f32, y: f32, z: f32) {
        self.matrix[3][0] = x;
        self.matrix[3][1] = y;
        self.matrix[3][2] = z;
    }

    pub fn scale(&mut self, x: f32, y: f32, z: f32) {
        self.matrix[0][0] *= x;
        self.matrix[1][1] *= y;
        self.matrix[2][2] *= z;
    }

    pub fn set_scale(&mut self, x: f32, y: f32, z: f32) {
        self.matrix[0][0] = x;
        self.matrix[1][1] = y;
        self.matrix[2][2] = z;
    }

    pub fn rotate_x(&mut self, angle: f32) {
        let sin = angle.sin();
        let cos = angle.cos();
        let temp = self.matrix[1][1];
        self.matrix[1][1] = temp * cos - self.matrix[1][2] * sin;
        self.matrix[1][2] = temp * sin + self.matrix[1][2] * cos;
        let temp = self.matrix[2][1];
        self.matrix[2][1] = temp * cos - self.matrix[2][2] * sin;
        self.matrix[2][2] = temp * sin + self.matrix[2][2] * cos;
    }

    pub fn set_rotate_x(&mut self, angle: f32) {
        let sin = angle.sin();
        let cos = angle.cos();
        self.matrix[1][1] = cos;
        self.matrix[1][2] = -sin;
        self.matrix[2][1] = sin;
        self.matrix[2][2] = cos;
    }

    pub fn rotate_y(&mut self, angle: f32) {
        let sin = angle.sin();
        let cos = angle.cos();
        let temp = self.matrix[0][0];
        self.matrix[0][0] = temp * cos + self.matrix[0][2] * sin;
        self.matrix[0][2] = -temp * sin + self.matrix[0][2] * cos;
        let temp = self.matrix[2][0];
        self.matrix[2][0] = temp * cos + self.matrix[2][2] * sin;
        self.matrix[2][2] = -temp * sin + self.matrix[2][2] * cos;
    }

    pub fn set_rotate_y(&mut self, angle: f32) {
        let sin = angle.sin();
        let cos = angle.cos();
        self.matrix[0][0] = cos;
        self.matrix[0][2] = sin;
        self.matrix[2][0] = -sin;
        self.matrix[2][2] = cos;
    }

    pub fn rotate_z(&mut self, angle: f32) {
        let sin = angle.sin();
        let cos = angle.cos();
        let temp = self.matrix[0][0];
        self.matrix[0][0] = temp * cos - self.matrix[0][1] * sin;
        self.matrix[0][1] = temp * sin + self.matrix[0][1] * cos;
        let temp = self.matrix[1][0];
        self.matrix[1][0] = temp * cos - self.matrix[1][1] * sin;
        self.matrix[1][1] = temp * sin + self.matrix[1][1] * cos;
    }

    pub fn set_rotate_z(&mut self, angle: f32) {
        let sin = angle.sin();
        let cos = angle.cos();
        self.matrix[0][0] = cos;
        self.matrix[0][1] = -sin;
        self.matrix[1][0] = sin;
        self.matrix[1][1] = cos;
    }

    pub fn set_rotate(&mut self, x: f32, y: f32, z: f32) {
        let sin_x = x.sin();
        let cos_x = x.cos();
        let sin_y = y.sin();
        let cos_y = y.cos();
        let sin_z = z.sin();
        let cos_z = z.cos();
        self.matrix[0][0] = cos_y * cos_z;
        self.matrix[0][1] = cos_y * sin_z;
        self.matrix[0][2] = -sin_y;
        self.matrix[1][0] = sin_x * sin_y * cos_z - cos_x * sin_z;
        self.matrix[1][1] = sin_x * sin_y * sin_z + cos_x * cos_z;
        self.matrix[1][2] = sin_x * cos_y;
        self.matrix[2][0] = cos_x * sin_y * cos_z + sin_x * sin_z;
        self.matrix[2][1] = cos_x * sin_y * sin_z - sin_x * cos_z;
        self.matrix[2][2] = cos_x * cos_y;
    }

    pub fn multiply(&mut self, other: &Self) {
        let mut result = [[0.0; 4]; 4];
        for i in 0..4 {
            for j in 0..4 {
                for k in 0..4 {
                    result[i][j] += self.matrix[i][k] * other.matrix[k][j];
                }
            }
        }
        self.matrix = result;
    }

    pub fn to_uniform(&self) -> &[f32] {
        bytemuck::cast_slice(&self.matrix)
    }

    pub fn rotate_towards(&mut self, origin: [f32; 3], target: [f32; 3]) {
        // self is the identity matrix
        // we want to create a rotation matrix that rotates a unit vector in the y direction to the direction from origin to target
        let direction = [target[0] - origin[0], target[1] - origin[1], target[2] - origin[2]];
        let direction_magnitude = (direction[0] * direction[0] + direction[1] * direction[1] + direction[2] * direction[2]).sqrt();
        
        let phi = (direction[2] / direction_magnitude).acos() - PI / 2.0;
        let theta = direction[1].atan2(direction[0]) - PI / 2.0;

        let mut theta_rotation = AffineMatrix::new();
        theta_rotation.set_rotate_x(theta);

        let mut phi_rotation = AffineMatrix::new();
        phi_rotation.set_rotate_z(phi);

        let mut y_rotation = AffineMatrix::new();
        y_rotation.set_rotate_y(-PI / 2.0);
        *self = phi_rotation * theta_rotation * y_rotation;
    }

    pub fn combine(&mut self, other: Self, alpha: f32) {
        for i in 0..4 {
            for j in 0..4 {
                self.matrix[i][j] = alpha * other.matrix[i][j] + (1.0 - alpha) * self.matrix[i][j];
            }
        }
    }

    fn face_y_towards(mut self, direction: [f32; 3]) -> Self {
        let direction_magnitude = (direction[0] * direction[0] + direction[1] * direction[1] + direction[2] * direction[2]).sqrt();
        let normalized_direction = [direction[0] / direction_magnitude, direction[1] / direction_magnitude, direction[2] / direction_magnitude];
        self = Self::ident();
        let phi = (-normalized_direction[0]).asin();
        let theta = (direction[1] / (1.0 - (direction[0] * direction[0])).sqrt()).acos();
        let mut rotate_phi = Self::ident();
        rotate_phi.set_rotate_z(phi);
        let mut rotate_theta = Self::ident();
        rotate_theta.set_rotate_x(theta);
        rotate_theta * rotate_phi
    }

    pub fn multiply_3d(&self, point: [f32; 3]) -> [f32; 3] {
        let mut result = [0.0; 3];
        for i in 0..3 {
            for j in 0..3 {
                result[i] += self.matrix[i][j] * point[j];
            }
        }
        result
    }

    pub fn multiply_4d(&self, point: [f32; 4]) -> [f32; 4] {
        let mut result = [0.0; 4];
        for i in 0..4 {
            for j in 0..4 {
                result[i] += self.matrix[i][j] * point[j];
            }
        }
        result
    }
}

impl Mul for AffineMatrix {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        let mut result = [[0.0; 4]; 4];
        for i in 0..4 {
            for j in 0..4 {
                for k in 0..4 {
                    result[i][j] += self.matrix[i][k] * other.matrix[k][j];
                }
            }
        }
        Self {
            matrix: result,
        }
    }
}