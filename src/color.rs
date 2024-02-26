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

    pub fn hue_angle_to_rgb(hue: f32) -> Self {
        let h = hue / 60.0;
        let c = 1.0;
        let x = (1.0 - (h % 2.0 - 1.0).abs()) * c;
        let m = 0.0;
        let (r, g, b) = if h < 1.0 {
            (c, x, 0.0)
        } else if h < 2.0 {
            (x, c, 0.0)
        } else if h < 3.0 {
            (0.0, c, x)
        } else if h < 4.0 {
            (0.0, x, c)
        } else if h < 5.0 {
            (x, 0.0, c)
        } else {
            (c, 0.0, x)
        };
        Self {
            v: [r + m, g + m, b + m, 1.0]
        }
    
    }

    pub fn from_hex_str(hex: &str) -> Self {
        let hex = hex.trim_start_matches("#");
        let r = u8::from_str_radix(&hex[0..2], 16).unwrap() as f32 / 255.0;
        let g = u8::from_str_radix(&hex[2..4], 16).unwrap() as f32 / 255.0;
        let b = u8::from_str_radix(&hex[4..6], 16).unwrap() as f32 / 255.0;
        Self {
            v: [r, g, b, 1.0]
        }
    
    }

    pub fn from_hex(hex: u32) -> Self {
        let r = ((hex >> 16) & 0xff) as f32 / 255.0;
        let g = ((hex >> 8) & 0xff) as f32 / 255.0;
        let b = (hex & 0xff) as f32 / 255.0;
        Self {
            v: [r, g, b, 1.0]
        }
    }

    pub fn from_rgb_float(r: f32, g: f32, b: f32) -> Self {
        Self {
            v: [r, g, b, 1.0]
        }
    }

    pub fn scale(&mut self, factor: f32) {
        self.v[0] *= factor;
        self.v[1] *= factor;
        self.v[2] *= factor;
    }
}

impl Index<usize> for Color {
    type Output = f32;

    fn index(&self, index: usize) -> &Self::Output {
        &self.v[index]
    }
}