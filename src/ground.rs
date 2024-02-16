use eframe::{egui::accesskit::Affine, egui_glow, glow::HasContext};
use egui_glow::glow;
use core::time;
use std::time::Instant;

use crate::{obj::JoinedOBJ, scene::Paintable};
use crate::affine_matrix::AffineMatrix;

pub struct Ground {
    program: glow::Program,
    obj: JoinedOBJ,
    time: Instant,
    pub translation: AffineMatrix,
    pub rotation: AffineMatrix,
    pub scale: AffineMatrix,
}

impl Ground {
    pub fn new(gl: &glow::Context) -> Self {
        let program = unsafe { Self::create_program(gl) };
        let obj = unsafe { JoinedOBJ::new(gl, "./long_ground_25.obj", program).expect("Cannot create JoinedOBJ") };
        let mut translation = AffineMatrix::new();
        let rotation = AffineMatrix::new();
        let scale = AffineMatrix::new();
        translation.translate(0.0, -1.0, -2.0);
        Self {
            program,
            obj,
            time: Instant::now(),
            translation,
            rotation,
            scale,
        }
    }

    unsafe fn create_program(gl: &glow::Context) -> glow::Program {
        let program = gl.create_program().expect("Cannot create program");
        let vertex_shader_source = include_str!("./shaders/ground.vs");
        let fragment_shader_source = include_str!("./shaders/ground.fs");

        let vertex_shader = gl.create_shader(glow::VERTEX_SHADER).expect("Cannot create vertex shader");
        gl.shader_source(vertex_shader, vertex_shader_source);
        gl.compile_shader(vertex_shader);
        assert!(gl.get_shader_compile_status(vertex_shader), "Cannot compile vertex shader: {}", gl.get_shader_info_log(vertex_shader));

        let fragment_shader = gl.create_shader(glow::FRAGMENT_SHADER).expect("Cannot create fragment shader");
        gl.shader_source(fragment_shader, fragment_shader_source);
        gl.compile_shader(fragment_shader);
        assert!(gl.get_shader_compile_status(fragment_shader), "Cannot compile fragment shader: {}", gl.get_shader_info_log(fragment_shader));

        gl.attach_shader(program, vertex_shader);
        gl.attach_shader(program, fragment_shader);

        gl.link_program(program);
        assert!(gl.get_program_link_status(program), "Cannot link program: {}", gl.get_program_info_log(program));

        gl.detach_shader(program, vertex_shader);
        gl.detach_shader(program, fragment_shader);
        gl.delete_shader(vertex_shader);
        gl.delete_shader(fragment_shader);

        program
    }

    fn set_transformation_uniforms(&self, gl: &glow::Context) {
        let translation_location = unsafe { gl.get_uniform_location(self.program, "translation").expect("Cannot get uniform location") };
        let rotation_location = unsafe { gl.get_uniform_location(self.program, "rotation").expect("Cannot get uniform location") };
        let scale_location = unsafe { gl.get_uniform_location(self.program, "scale").expect("Cannot get uniform location") };
        unsafe {
            gl.uniform_matrix_4_f32_slice(Some(&translation_location), false, &self.translation.to_uniform());
            gl.uniform_matrix_4_f32_slice(Some(&rotation_location), false, &self.rotation.to_uniform());
            gl.uniform_matrix_4_f32_slice(Some(&scale_location), false, &self.scale.to_uniform());
        }
    }
}

impl Paintable for Ground {
    fn paint(&self, gl: &glow::Context, screen_size: (f32, f32)) {
        unsafe {
            gl.use_program(Some(self.program));
            let screen_size_location = gl.get_uniform_location(self.program, "screen_size").expect("Cannot get uniform location");
            gl.uniform_2_f32(Some(&screen_size_location), screen_size.0, screen_size.1);
            self.set_transformation_uniforms(gl);
            gl.bind_vertex_array(Some(self.obj.vao));
            gl.draw_elements(glow::TRIANGLES, self.obj.num_indices, glow::UNSIGNED_INT, 0);

            gl.bind_vertex_array(None);
        }
    }
}