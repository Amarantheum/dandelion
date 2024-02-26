use eframe::egui::accesskit::Affine;
use eframe::egui::Color32;
use eframe::{egui_glow, glow::HasContext};
use egui_glow::glow;
use tobj;
use bytemuck;
use std::time::Instant;

use crate::affine_matrix::AffineMatrix;
use crate::color::Color;
use crate::scene::Paintable;
use crate::obj::OBJ;
use crate::create_program;

pub struct DandelionSeedJoined {
    program: glow::Program,
    translation: AffineMatrix,
    rotation: AffineMatrix,
    scale: AffineMatrix,
}

impl DandelionSeedJoined {
    pub fn new(gl: &glow::Context) -> Self {
        let mut obj = OBJ::new("./DandelionSeed.obj").unwrap();
        let program = create_program!(include_str!("./shaders/dandelion_fancy.vs"), include_str!("./shaders/dandelion_fancy.fs"), gl);
        obj.build_vao(gl, stem_program, "Circle").unwrap();
        obj.build_vao(gl, fluff_program, "Mesh").unwrap();
        Self {
            stem_program,
            fluff_program,
            fancy_program,
            fluff_fancy_program,
            obj,
            translation: AffineMatrix::new(),
            rotation: AffineMatrix::new(),
            scale: AffineMatrix::new(),
            theta: 0.0,
            theta_delta: 0.0,
            color: Color::from_gray(0.0, 1.0),
            fancy: false,
        }
    }

    pub fn get_position(&self) -> [f32; 3] {
        [self.translation.matrix[3][0], self.translation.matrix[3][1], self.translation.matrix[3][2]]
    }

    unsafe fn create_program(gl: &glow::Context) -> glow::Program {
        let program = gl.create_program().expect("Cannot create program");
        let vertex_shader_source = include_str!("./shaders/dandelion.vs");
        let fragment_shader_source = include_str!("./shaders/dandelion.fs");

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

    unsafe fn initialize_vertices(gl: &glow::Context, program: glow::Program) -> (glow::VertexArray, i32) {
        
        let (models, _) = tobj::load_obj("./DandelionSeed.obj", &tobj::GPU_LOAD_OPTIONS).expect("Cannot load DandelionSeed.obj");
        let stem = &models[0].mesh;
        let fluff = &models[1].mesh;

        let vertices_combined = stem.positions.iter().chain(fluff.positions.iter()).map(|v| *v).collect::<Vec<f32>>();
        let indices_combined = stem.indices.iter().map(|i| *i).chain(fluff.indices.iter().map(|i| i + stem.positions.len() as u32)).collect::<Vec<u32>>();

        let vao = gl.create_vertex_array().expect("Cannot create vertex array");
        gl.bind_vertex_array(Some(vao));

        let vbo = gl.create_buffer().expect("Cannot create vertex buffer");
        gl.bind_buffer(glow::ARRAY_BUFFER, Some(vbo));
        gl.buffer_data_u8_slice(
            glow::ARRAY_BUFFER,
            bytemuck::cast_slice(&vertices_combined),
            glow::STATIC_DRAW,
        );

        let ibo = gl.create_buffer().expect("Cannot create index buffer");
        gl.bind_buffer(glow::ELEMENT_ARRAY_BUFFER, Some(ibo));
        gl.buffer_data_u8_slice(
            glow::ELEMENT_ARRAY_BUFFER,
            bytemuck::cast_slice(&indices_combined),
            glow::STATIC_DRAW,
        );

        let position_attrib_location = gl.get_attrib_location(program, "position").unwrap();

        gl.enable_vertex_attrib_array(position_attrib_location);

        gl.vertex_attrib_pointer_f32(position_attrib_location, 3, glow::FLOAT, false, 3 * std::mem::size_of::<f32>() as i32, 0);

        gl.bind_vertex_array(None);

        (vao, indices_combined.len() as i32)
    }

    pub fn destroy(self, gl: &glow::Context) {
        unsafe {
            gl.delete_program(self.stem_program);
            gl.delete_program(self.fluff_program);
            self.obj.destroy(gl);
        }
    }

    fn set_transformation_uniforms(&self, gl: &glow::Context, program: glow::Program) {
        let translation_location = unsafe { gl.get_uniform_location(program, "translation").expect("Cannot get uniform location") };
        let rotation_location = unsafe { gl.get_uniform_location(program, "rotation").expect("Cannot get uniform location") };
        let scale_location = unsafe { gl.get_uniform_location(program, "scale").expect("Cannot get uniform location") };
        unsafe {
            gl.uniform_matrix_4_f32_slice(Some(&translation_location), false, &self.translation.to_uniform());
            gl.uniform_matrix_4_f32_slice(Some(&rotation_location), false, &self.rotation.to_uniform());
            gl.uniform_matrix_4_f32_slice(Some(&scale_location), false, &self.scale.to_uniform());
        }
    }
}

impl Paintable for DandelionSeed {
    fn paint(&self, gl: &glow::Context, screen_size: (f32, f32), view_matrix: &AffineMatrix) {
        unsafe {
            if !self.fancy {
                gl.use_program(Some(self.stem_program));
                let screen_size_location = gl.get_uniform_location(self.stem_program, "screen_size").expect("Cannot get uniform location");
                gl.uniform_2_f32(Some(&screen_size_location), screen_size.0, screen_size.1);
                let camera_matrix_location = gl.get_uniform_location(self.stem_program, "view_matrix").expect("Cannot get view_matrix uniform location");
                gl.uniform_matrix_4_f32_slice(Some(&camera_matrix_location), false, &view_matrix.to_uniform());
                let color_location = gl.get_uniform_location(self.stem_program, "color").expect("unable to find color location");
                gl.uniform_4_f32(Some(&color_location), self.color[0], self.color[1], self.color[2], self.color[3]);
                self.set_transformation_uniforms(gl, self.stem_program);

                let stem_vao = self.obj["Circle"].get_vao().unwrap();
                gl.bind_vertex_array(Some(stem_vao.vao));
                gl.draw_elements(glow::TRIANGLES, stem_vao.num_indices, glow::UNSIGNED_INT, 0);
                gl.bind_vertex_array(None);

                gl.use_program(Some(self.fluff_program));
                let screen_size_location = gl.get_uniform_location(self.fluff_program, "screen_size").expect("Cannot get uniform location");
                gl.uniform_2_f32(Some(&screen_size_location), screen_size.0, screen_size.1);
                let camera_matrix_location = gl.get_uniform_location(self.fluff_program, "view_matrix").expect("Cannot get view_matrix uniform location");
                gl.uniform_matrix_4_f32_slice(Some(&camera_matrix_location), false, &view_matrix.to_uniform());
                let color_location = gl.get_uniform_location(self.fluff_program, "color").expect("unable to find color location");
                gl.uniform_4_f32(Some(&color_location), self.color[0], self.color[1], self.color[2], self.color[3]);
                self.set_transformation_uniforms(gl, self.fluff_program);

                let fluff_vao = self.obj["Mesh"].get_vao().unwrap();
                gl.bind_vertex_array(Some(fluff_vao.vao));
                gl.draw_elements(glow::TRIANGLES, fluff_vao.num_indices, glow::UNSIGNED_INT, 0);
                gl.bind_vertex_array(None);
            } else {
                gl.use_program(Some(self.fancy_program));
                let screen_size_location = gl.get_uniform_location(self.fancy_program, "screen_size").expect("Cannot get uniform location");
                gl.uniform_2_f32(Some(&screen_size_location), screen_size.0, screen_size.1);
                let camera_matrix_location = gl.get_uniform_location(self.fancy_program, "view_matrix").expect("Cannot get view_matrix uniform location");
                gl.uniform_matrix_4_f32_slice(Some(&camera_matrix_location), false, &view_matrix.to_uniform());
                //let color_location = gl.get_uniform_location(self.fancy_program, "color").expect("unable to find color location");
                //gl.uniform_4_f32(Some(&color_location), self.color[0], self.color[1], self.color[2], self.color[3]);
                self.set_transformation_uniforms(gl, self.fancy_program);

                let stem_vao = self.obj["Circle"].get_vao().unwrap();
                gl.bind_vertex_array(Some(stem_vao.vao));
                gl.draw_elements(glow::TRIANGLES, stem_vao.num_indices, glow::UNSIGNED_INT, 0);
                gl.bind_vertex_array(None);

                gl.use_program(Some(self.fluff_fancy_program));
                let screen_size_location = gl.get_uniform_location(self.fluff_fancy_program, "screen_size").expect("Cannot get uniform location");
                gl.uniform_2_f32(Some(&screen_size_location), screen_size.0, screen_size.1);
                let camera_matrix_location = gl.get_uniform_location(self.fluff_fancy_program, "view_matrix").expect("Cannot get view_matrix uniform location");
                gl.uniform_matrix_4_f32_slice(Some(&camera_matrix_location), false, &view_matrix.to_uniform());
                let color_location = gl.get_uniform_location(self.fluff_fancy_program, "color").expect("unable to find color location");
                gl.uniform_4_f32(Some(&color_location), self.color[0], self.color[1], self.color[2], self.color[3]);
                self.set_transformation_uniforms(gl, self.fluff_fancy_program);

                let fluff_vao = self.obj["Mesh"].get_vao().unwrap();
                gl.bind_vertex_array(Some(fluff_vao.vao));
                gl.draw_elements(glow::TRIANGLES, fluff_vao.num_indices, glow::UNSIGNED_INT, 0);
                gl.bind_vertex_array(None);
            }
            
        }
    }
}