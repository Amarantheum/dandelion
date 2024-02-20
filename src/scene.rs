use eframe::{egui_glow, glow::HasContext, egui};
use egui_glow::glow;
use std::time::Instant;
use rand::prelude::*;
use rand::rngs::StdRng;

use crate::dandelion::DandelionSeed;
use crate::ground::{self, Ground};
use crate::affine_matrix::AffineMatrix;

pub trait Paintable {
    fn paint(&self, gl: &glow::Context, screen_size: (f32, f32));
}

pub struct Scene {
    time: Instant,
    dandelion_seed1: DandelionSeed,
    dandelion_seed1_theta: f32,
    dandelion_seed1_theta_delta: f32,
    dandelion_seed1_phi: f32,
    ground: Ground,
    ground_mirror: Ground,
    ground_2: Ground,
    ground_mirror_2: Ground,
    rng: rand::rngs::OsRng,
}

impl Scene {
    pub fn new(gl: &glow::Context) -> Self {
        let mut dandelion_seed1 = DandelionSeed::new(gl);
        dandelion_seed1.translation.set_translate(0.0, 0.0, -2.0);
        dandelion_seed1.scale.set_scale(0.04, 0.04, 0.04);

        let ground = Ground::new(gl);
        let mut ground_mirror = Ground::new(gl);
        ground_mirror.scale.set_scale(-1.0, 1.0, 1.0);
        let ground_2 = Ground::new(gl);
        let mut ground_mirror_2 = Ground::new(gl);
        ground_mirror_2.scale.set_scale(-1.0, 1.0, 1.0);
        let rng = rand::rngs::OsRng::default();
        Self {
            time: Instant::now(),
            dandelion_seed1,
            dandelion_seed1_theta: 0.0,
            dandelion_seed1_theta_delta: 0.0,
            dandelion_seed1_phi: 0.0,
            ground,
            ground_mirror,
            ground_2,
            ground_mirror_2,
            rng,
        }
    }
    
    pub fn update(&mut self) {
        let elapsed = self.time.elapsed().as_secs_f32() * 0.5 + 32.0;
        //self.dandelion_seed1.update(elapsed);
        let ground_x = elapsed % 32.0 - 16.0;
        let ground_mirror_x = (elapsed - 8.0) % 32.0 - 16.0;
        let ground_2_x = (elapsed - 16.0) % 32.0 - 16.0;
        let ground_mirror_2_x = (elapsed - 24.0) % 32.0 - 16.0;
        
        self.ground.translation.set_translate(ground_x, -1.0, -2.0);
        self.ground_mirror.translation.set_translate(ground_mirror_x, -1.0, -2.0);
        self.ground_2.translation.set_translate(ground_2_x, -1.0, -2.0);
        self.ground_mirror_2.translation.set_translate(ground_mirror_2_x, -1.0, -2.0);

        let alpha_theta = 0.01;
        let alpha_phi = 0.0001;
        self.dandelion_seed1_theta_delta = alpha_theta * (2.0 * self.rng.gen::<f32>() - 1.0) + (1.0 - alpha_theta) * self.dandelion_seed1_theta_delta;
        self.dandelion_seed1_theta += self.dandelion_seed1_theta_delta * 0.1;

        self.dandelion_seed1_phi = alpha_phi * (2.0 * self.rng.gen::<f32>() - 1.0) + (1.0 - alpha_phi) * self.dandelion_seed1_phi;
        let mut theta = AffineMatrix::new();
        theta.set_rotate_y(self.dandelion_seed1_theta);
        let mut phi = AffineMatrix::new();
        phi.set_rotate_x(self.dandelion_seed1_phi * 100.0);
        phi.multiply(&theta);
        self.dandelion_seed1.rotation = phi;
    }
}

impl Paintable for Scene {
    fn paint(&self, gl: &glow::Context, screen_size: (f32, f32)) {
        unsafe { gl.clear_color(0.0, 0.0, 0.0, 0.0); }
        self.ground.paint(gl, screen_size);
        self.ground_mirror.paint(gl, screen_size);
        self.ground_2.paint(gl, screen_size);
        self.ground_mirror_2.paint(gl, screen_size);

        self.dandelion_seed1.paint(gl, screen_size);
    }
}