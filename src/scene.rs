use eframe::{egui_glow, glow::HasContext, egui};
use egui_glow::glow;

use crate::dandelion::DandelionSeed;
use crate::ground::Ground;

pub trait Paintable {
    fn paint(&self, gl: &glow::Context, screen_size: (f32, f32));
}

pub struct Scene {
    //dandelion_seed1: DandelionSeed,
    ground: Ground,
}

impl Scene {
    pub fn new(gl: &glow::Context) -> Self {
        //let dandelion_seed1 = DandelionSeed::new(gl);
        let ground = Ground::new(gl);
        Self {
            //dandelion_seed1,
            ground,
        }
    }
}

impl Paintable for Scene {
    fn paint(&self, gl: &glow::Context, screen_size: (f32, f32)) {
        unsafe { gl.clear_color(0.0, 0.0, 0.0, 0.0); }
        //self.dandelion_seed1.paint(gl);
        self.ground.paint(gl, screen_size);
    }
}