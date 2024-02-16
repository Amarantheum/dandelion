pub struct AffineMatrix {
    pub matrix: [[f32; 4]; 4],
}

impl AffineMatrix {
    pub fn new() -> Self {
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
}