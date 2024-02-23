use std::ops::Index;

#[derive(Copy, Clone, Debug)]
pub struct Color {
    v: [f32; 4],
}

impl Color {
    pub fn from_gray(value: f32, alpha: f32) -> Self {
        Self {
            v: [value, value, value, alpha]
        }
    }
}

impl Index<usize> for Color {
    type Output = f32;

    fn index(&self, index: usize) -> &Self::Output {
        &self.v[index]
    }
}