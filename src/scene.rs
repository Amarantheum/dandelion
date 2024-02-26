use eframe::egui::accesskit::Affine;
use eframe::epaint::text;
use eframe::{egui_glow, glow::HasContext, egui};
use egui_glow::glow;
use std::f32::consts::PI;
use std::time::Instant;
use rand::prelude::*;
use rand::rngs::{OsRng, StdRng};

use crate::color::Color;
use crate::dandelion::DandelionSeed;
use crate::ground::{self, Ground};
use crate::affine_matrix::AffineMatrix;
use crate::{DandelionState, BODY1_BASE_SPINE, BODY1_HEAD, BODY2_BASE_SPINE, BODY2_HEAD};

pub trait Paintable {
    fn paint(&self, gl: &glow::Context, screen_size: (f32, f32), view_matrix: &AffineMatrix);
}

pub struct Scene {
    time: Instant,
    dandelion_seed1: DandelionSeed,
    dandelion_seed2: DandelionSeed,

    dancing_dandelion1: DandelionSeed,
    dancing_dandelion2: DandelionSeed,
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

        let mut dancing_dandelion1 = DandelionSeed::new(gl);
        dancing_dandelion1.scale.set_scale(0.04, 0.04, 0.04);
        let mut dancing_dandelion2 = DandelionSeed::new(gl);
        dancing_dandelion2.scale.set_scale(0.04, 0.04, 0.04);


        Self {
            time: Instant::now(),
            dandelion_seed1,
            dandelion_seed2,
            dancing_dandelion1,
            dancing_dandelion2,
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

    fn update_dandelion(dandelion: &mut DandelionSeed, rng: &mut OsRng, body_pos: [f32; 3], head_pos: [f32; 3], color: Color, drift_strength: f32) {
        let alpha_y = 0.01;
        dandelion.theta_delta = alpha_y * (2.0 * rng.gen::<f32>() - 1.0) + (1.0 - alpha_y) * dandelion.theta_delta;
        dandelion.theta += dandelion.theta_delta;
        let mut theta = AffineMatrix::new();
        theta.set_rotate_y(dandelion.theta);

        let mut drift_position = AffineMatrix::new();
        drift_position.set_translate(10.0, -1.0, -2.0);

        let alpha = 0.1;
        let mut translate = AffineMatrix::new();
        translate.set_translate(body_pos[0], body_pos[1], body_pos[2]);
        dandelion.translation.combine(translate, alpha);
        dandelion.translation.combine(drift_position, drift_strength);

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
            self.swap_dandelions();
        }
    }

    fn swap_dandelions(&mut self) {
        std::mem::swap(&mut self.dandelion_seed1, &mut self.dandelion_seed2);
    }

    pub fn dandelion_dance(&mut self, brightness: f32) {
        let initialx_offset = 0.1;
        let initialz_offset = 0.2;
        let x_angle = PI / 4.0; 
        let finaly_offset = -0.2;
        let finalz_offset = -0.5;
        let time = self.time.elapsed().as_secs_f32();
        let mut color = Color::from_rgb_float(1.0, 0.85, 0.45);
        color.scale(brightness);

        let mut initial_rotation = AffineMatrix::new();
        initial_rotation.set_rotate_x(x_angle);
        let mut position = AffineMatrix::new();
        position.set_translate(initialx_offset, 0.0, initialz_offset);
        
        let mut y_rotation = AffineMatrix::new();
        y_rotation.set_rotate_y(time);

        let mut translation = AffineMatrix::new();
        translation.set_translate(0.0, finaly_offset, finalz_offset);

        self.dancing_dandelion1.rotation = AffineMatrix::new();
        self.dancing_dandelion1.translation = initial_rotation * position * y_rotation * translation;
        self.dancing_dandelion1.color = color;

        let mut initial_rotation = AffineMatrix::new();
        initial_rotation.set_rotate_x(-x_angle);
        let mut position = AffineMatrix::new();
        position.set_translate(-initialx_offset, 0.0, -initialz_offset);
        
        let mut y_rotation = AffineMatrix::new();
        y_rotation.set_rotate_y(time);

        let mut translation = AffineMatrix::new();
        translation.set_translate(0.0, finaly_offset, finalz_offset);

        self.dancing_dandelion2.rotation = AffineMatrix::new();
        self.dancing_dandelion2.translation = initial_rotation * position * y_rotation * translation;
        self.dancing_dandelion2.color = color;

        self.dancing_dandelion1.fancy = true;
        self.dancing_dandelion2.fancy = true;
    }
    
    pub fn update(&mut self, state: DandelionState) {
        
 
        if state.dancing_brightness > 0.0 {
            self.dandelion_dance(state.dancing_brightness);
        }
        self.update_ground(state.brightness);
        self.dandelion_seed1.fancy = false;
        self.dandelion_seed2.fancy = false;
        let color = Color::from_gray(state.brightness, 1.0);
        let affection = state.affection;
        let body_pos = *BODY1_BASE_SPINE.lock();
        let head_pos = *BODY1_HEAD.lock();
        let other_pos = *BODY2_BASE_SPINE.lock();
        let pos = [
            other_pos[0] * affection + (1.0 - affection) * head_pos[0],
            other_pos[1] * affection + (1.0 - affection) * head_pos[1],
            other_pos[2] * affection + (1.0 - affection) * head_pos[2],
        ];
        Self::update_dandelion(&mut self.dandelion_seed1, &mut self.rng, body_pos, pos, color, 0.0);
        

        let body_pos = *BODY2_BASE_SPINE.lock();
        let head_pos = *BODY2_HEAD.lock();
        let other_pos = *BODY1_BASE_SPINE.lock();
        let pos = [
            other_pos[0] * affection + (1.0 - affection) * head_pos[0],
            other_pos[1] * affection + (1.0 - affection) * head_pos[1],
            other_pos[2] * affection + (1.0 - affection) * head_pos[2],
        ];
        Self::update_dandelion(&mut self.dandelion_seed2, &mut self.rng, body_pos, pos, color, state.drift_strength);        
    }

    pub fn paint(&self, gl: &glow::Context, screen_size: (f32, f32), state: DandelionState) {
        let mut view_matrix = AffineMatrix::new();
        view_matrix.set_translate(-self.camera_pos[0], -self.camera_pos[1], -self.camera_pos[2]);

        unsafe { gl.clear_color(0.0, 0.0, 0.0, 1.0); }
        self.ground.paint(gl, screen_size, &view_matrix);
        self.ground_mirror.paint(gl, screen_size, &view_matrix);
        self.ground_2.paint(gl, screen_size, &view_matrix);
        self.ground_mirror_2.paint(gl, screen_size, &view_matrix);

        if state.brightness > 0.0 {
            if self.dandelion_seed1.get_position()[2] < self.dandelion_seed2.get_position()[2] {
                self.dandelion_seed1.paint(gl, screen_size, &view_matrix);
                self.dandelion_seed2.paint(gl, screen_size, &view_matrix);
            } else {
                self.dandelion_seed2.paint(gl, screen_size, &view_matrix);
                self.dandelion_seed1.paint(gl, screen_size, &view_matrix);
            }
        }
        if state.dancing_brightness > 0.0 {
            if self.dancing_dandelion1.get_position()[2] < self.dancing_dandelion2.get_position()[2] {
                self.dancing_dandelion1.paint(gl, screen_size, &view_matrix);
                self.dancing_dandelion2.paint(gl, screen_size, &view_matrix);
            } else {
                self.dancing_dandelion2.paint(gl, screen_size, &view_matrix);
                self.dancing_dandelion1.paint(gl, screen_size, &view_matrix);
            }
        }
        
        
    }
}