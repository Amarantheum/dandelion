use eframe::egui::accesskit::Affine;
use eframe::epaint::text;
use eframe::{egui_glow, glow::HasContext, egui};
use egui_glow::glow;
use std::time::Instant;
use rand::prelude::*;
use rand::rngs::{OsRng, StdRng};

use crate::color::Color;
use crate::dandelion::DandelionSeed;
use crate::ground::{self, Ground};
use crate::affine_matrix::AffineMatrix;
use crate::{BODY1_BASE_SPINE, BODY2_BASE_SPINE, BODY1_HEAD, BODY2_HEAD};

pub trait Paintable {
    fn paint(&self, gl: &glow::Context, screen_size: (f32, f32), view_matrix: &AffineMatrix);
}

pub struct Scene {
    time: Instant,
    dandelion_seed1: DandelionSeed,
    dandelion_seed1_theta: f32,
    dandelion_seed1_theta_delta: f32,
    dandelion_seed2: DandelionSeed,
    dandelion_seed2_theta: f32,
    dandelion_seed2_theta_delta: f32,
    ground: Ground,
    ground_mirror: Ground,
    ground_2: Ground,
    ground_mirror_2: Ground,
    rng: rand::rngs::OsRng,
    frame_buffer: glow::Framebuffer,
    texture: glow::Texture,
    view_port: (f32, f32),
    pub camera_pos: [f32; 3],
}

impl Scene {
    pub fn new(gl: &glow::Context) -> Self {
        let mut dandelion_seed1 = DandelionSeed::new(gl);
        dandelion_seed1.translation.set_translate(0.0, 0.0, -2.0);
        dandelion_seed1.scale.set_scale(0.04, 0.04, 0.04);

        let mut dandelion_seed2 = DandelionSeed::new(gl);
        dandelion_seed2.translation.set_translate(0.0, 0.0, -2.0);
        dandelion_seed2.scale.set_scale(0.04, 0.04, 0.04);

        let ground = Ground::new(gl);
        let mut ground_mirror = Ground::new(gl);
        ground_mirror.scale.set_scale(-1.0, 1.0, 1.0);
        let ground_2 = Ground::new(gl);
        let mut ground_mirror_2 = Ground::new(gl);
        ground_mirror_2.scale.set_scale(-1.0, 1.0, 1.0);
        let rng = rand::rngs::OsRng::default();
        let texture = unsafe {
            let texture = gl.create_texture().expect("Failed to create texture");
            gl.bind_texture(glow::TEXTURE_2D, Some(texture));

            // Set texture parameters (same as before)
            gl.tex_parameter_i32(glow::TEXTURE_2D, glow::TEXTURE_WRAP_S, glow::CLAMP_TO_EDGE as i32);
            gl.tex_parameter_i32(glow::TEXTURE_2D, glow::TEXTURE_WRAP_T, glow::CLAMP_TO_EDGE as i32);
            gl.tex_parameter_i32(glow::TEXTURE_2D, glow::TEXTURE_MIN_FILTER, glow::LINEAR as i32);
            gl.tex_parameter_i32(glow::TEXTURE_2D, glow::TEXTURE_MAG_FILTER, glow::LINEAR as i32);
            gl.tex_image_2d(
                glow::TEXTURE_2D,
                0,
                glow::RGBA as i32,
                100 as i32,
                100 as i32,
                0,
                glow::RGBA,
                glow::UNSIGNED_BYTE,
                None,
            );
            texture
        };
        let fbo = unsafe {
            let fbo = gl.create_framebuffer().expect("Failed to create FBO");
            gl.bind_framebuffer(glow::FRAMEBUFFER, Some(fbo));

            // Attach the texture to the FBO
            gl.framebuffer_texture_2d(
                glow::FRAMEBUFFER,
                glow::COLOR_ATTACHMENT0,
                glow::TEXTURE_2D,
                Some(texture),
                0,
            );
            // Check the FBO status
            let status = gl.check_framebuffer_status(glow::FRAMEBUFFER);
            if status != glow::FRAMEBUFFER_COMPLETE {
                panic!("Framebuffer is not complete: {:?}", status);
            }
            gl.bind_framebuffer(glow::FRAMEBUFFER, None);
            fbo
        };

        Self {
            time: Instant::now(),
            dandelion_seed1,
            dandelion_seed1_theta: 0.0,
            dandelion_seed1_theta_delta: 0.0,
            dandelion_seed2,
            dandelion_seed2_theta: 0.0,
            dandelion_seed2_theta_delta: 0.0,
            ground,
            ground_mirror,
            ground_2,
            ground_mirror_2,
            rng,
            frame_buffer: fbo,
            texture,
            view_port: (0.0, 0.0),
            camera_pos: [0.0; 3],
        }
    }

    fn update_ground(&mut self, brightness: f32) {
        let elapsed = self.time.elapsed().as_secs_f32() * 0.5 + 32.0;
        let ground_x = elapsed % 32.0 - 16.0;
        let ground_mirror_x = (elapsed - 8.0) % 32.0 - 16.0;
        let ground_2_x = (elapsed - 16.0) % 32.0 - 16.0;
        let ground_mirror_2_x = (elapsed - 24.0) % 32.0 - 16.0;
        
        self.ground.translation.set_translate(ground_x, -1.0, -2.0);
        self.ground_mirror.translation.set_translate(ground_mirror_x, -1.0, -2.0);
        self.ground_2.translation.set_translate(ground_2_x, -1.0, -2.0);
        self.ground_mirror_2.translation.set_translate(ground_mirror_2_x, -1.0, -2.0);

        self.ground.color = Color::from_gray(brightness, 1.0);
        self.ground_mirror.color = Color::from_gray(brightness, 1.0);
        self.ground_2.color = Color::from_gray(brightness, 1.0);
        self.ground_mirror_2.color = Color::from_gray(brightness, 1.0);
    }

    fn update_dandelion(dandelion: &mut DandelionSeed, rng: &mut OsRng, body_pos: [f32; 3], head_pos: [f32; 3], color: Color) {
        let alpha_y = 0.01;
        dandelion.theta_delta = alpha_y * (2.0 * rng.gen::<f32>() - 1.0) + (1.0 - alpha_y) * dandelion.theta_delta;
        dandelion.theta += dandelion.theta_delta;
        let mut theta = AffineMatrix::new();
        theta.set_rotate_y(dandelion.theta);

        let alpha = 0.1;
        let cur_pos = dandelion.get_position();
        dandelion.translation.set_translate(
            (1.0 - alpha) * cur_pos[0] + alpha * body_pos[0],
            (1.0 - alpha) * cur_pos[1] + alpha * body_pos[1],
            (1.0 - alpha) * cur_pos[2] + alpha * body_pos[2]
        );

        let mut rotation = AffineMatrix::new();
        rotation.rotate_towards(theta.multiply_3d(body_pos), theta.multiply_3d(head_pos));

        dandelion.rotation.combine(rotation * theta, alpha);
        dandelion.color = color;
    }

    fn dist(left: [f32; 3], right: [f32; 3]) -> f32 {
        ((left[0] - right[0]).powi(2) + (left[1] - right[1]).powi(2) + (left[2] - right[2]).powi(2)).sqrt()
    }

    fn check_dandelions(&mut self) {
        let d1_pos = self.dandelion_seed1.get_position();
        let d2_pos = self.dandelion_seed2.get_position();

        let body1_pos = *BODY1_BASE_SPINE.lock();
        let body2_pos = *BODY2_BASE_SPINE.lock();

        let d1_b1_dist = Self::dist(d1_pos, body1_pos);
        let d2_b1_dist = Self::dist(d2_pos, body1_pos);
        
        let d1_b2_dist = Self::dist(d1_pos, body2_pos);
        let d2_b2_dist = Self::dist(d2_pos, body2_pos);

        if d2_b1_dist > d1_b1_dist && d1_b2_dist > d2_b2_dist && (d1_b1_dist > 1.0 || d2_b2_dist > 1.0) {
            
        }
    }

    fn swap_dandelions(&mut self) {
        //std::mem::swap()
    }
    
    pub fn update(&mut self, brightness: f32, affection: f32) {
        self.update_ground(brightness);



        let color = Color::from_gray(brightness, 1.0);

        let body_pos = *BODY1_BASE_SPINE.lock();
        let head_pos = *BODY1_HEAD.lock();
        let other_pos = *BODY2_BASE_SPINE.lock();
        let pos = [
            other_pos[0] * affection + (1.0 - affection) * head_pos[0],
            other_pos[1] * affection + (1.0 - affection) * head_pos[1],
            other_pos[2] * affection + (1.0 - affection) * head_pos[2],
        ];
        Self::update_dandelion(&mut self.dandelion_seed1, &mut self.rng, body_pos, pos, color);

        let body_pos = *BODY2_BASE_SPINE.lock();
        let head_pos = *BODY2_HEAD.lock();
        let other_pos = *BODY1_BASE_SPINE.lock();
        let pos = [
            other_pos[0] * affection + (1.0 - affection) * head_pos[0],
            other_pos[1] * affection + (1.0 - affection) * head_pos[1],
            other_pos[2] * affection + (1.0 - affection) * head_pos[2],
        ];
        Self::update_dandelion(&mut self.dandelion_seed2, &mut self.rng, body_pos, pos, color);
    }

    pub fn paint(&self, gl: &glow::Context, screen_size: (f32, f32)) {
        let mut view_matrix = AffineMatrix::new();
        view_matrix.set_translate(-self.camera_pos[0], -self.camera_pos[1], -self.camera_pos[2]);

        unsafe { gl.clear_color(0.0, 0.0, 0.0, 1.0); }
        self.ground.paint(gl, screen_size, &view_matrix);
        self.ground_mirror.paint(gl, screen_size, &view_matrix);
        self.ground_2.paint(gl, screen_size, &view_matrix);
        self.ground_mirror_2.paint(gl, screen_size, &view_matrix);

        self.dandelion_seed1.paint(gl, screen_size, &view_matrix);
        self.dandelion_seed2.paint(gl, screen_size, &view_matrix);

        
    }
}