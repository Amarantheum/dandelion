use eframe::{egui_glow, glow::HasContext, egui};
use egui_glow::glow;
use std::time::Instant;

use crate::dandelion::DandelionSeed;
use crate::ground::{self, Ground};

pub trait Paintable {
    fn paint(&self, gl: &glow::Context, screen_size: (f32, f32));
}

pub struct Scene {
    time: Instant,
    //dandelion_seed1: DandelionSeed,
    ground: Ground,
    ground_mirror: Ground,
    ground_2: Ground,
    ground_mirror_2: Ground,
}

impl Scene {
    pub fn new(gl: &glow::Context) -> Self {
        //let dandelion_seed1 = DandelionSeed::new(gl);
        let ground = Ground::new(gl);
        let mut ground_mirror = Ground::new(gl);
        ground_mirror.scale.set_scale(-1.0, 1.0, 1.0);
        let ground_2 = Ground::new(gl);
        let mut ground_mirror_2 = Ground::new(gl);
        ground_mirror_2.scale.set_scale(-1.0, 1.0, 1.0);
        Self {
            time: Instant::now(),
            //dandelion_seed1,
            ground,
            ground_mirror,
            ground_2,
            ground_mirror_2,
        }
    }
    
    pub fn update(&mut self) {
        let elapsed = self.time.elapsed().as_secs_f32() * 2.0 + 32.0;
        //self.dandelion_seed1.update(elapsed);
        let ground_x = elapsed % 32.0 - 16.0;
        let ground_mirror_x = (elapsed - 8.0) % 32.0 - 16.0;
        let ground_2_x = (elapsed - 16.0) % 32.0 - 16.0;
        let ground_mirror_2_x = (elapsed - 24.0) % 32.0 - 16.0;
        
        self.ground.translation.set_translate(ground_x, -1.0, -2.0);
        self.ground_mirror.translation.set_translate(ground_mirror_x, -1.0, -2.0);
        self.ground_2.translation.set_translate(ground_2_x, -1.0, -2.0);
        self.ground_mirror_2.translation.set_translate(ground_mirror_2_x, -1.0, -2.0);
    }
}

impl Paintable for Scene {
    fn paint(&self, gl: &glow::Context, screen_size: (f32, f32)) {
        unsafe { gl.clear_color(0.0, 0.0, 0.0, 0.0); }
        //self.dandelion_seed1.paint(gl);
        self.ground.paint(gl, screen_size);
        self.ground_mirror.paint(gl, screen_size);
        self.ground_2.paint(gl, screen_size);
        self.ground_mirror_2.paint(gl, screen_size);
    }
}