use eframe::{egui_glow, glow::HasContext};
use egui_glow::glow;
use tobj;
use bytemuck;
use std::time::Instant;

use crate::scene::Paintable;

pub struct DandelionSeed {
    program: glow::Program,
    vertex_array: glow::VertexArray,
    num_indices: i32,
    position: Position3D,
    rotation: Rotation3D,
    time: Instant,
}

impl DandelionSeed {
    pub fn new(gl: &glow::Context) -> Self {
        let program = unsafe { Self::create_program(gl) };
        let (vertex_array, num_indices) = unsafe { Self::initialize_vertices(gl, program) };
        let position = Position3D { x: 0.0, y: 0.0, z: 0.0 };
        let rotation = Rotation3D { x: 0.0, y: 0.0, z: 0.0 };
        let time = Instant::now();
        Self {
            program,
            vertex_array,
            num_indices,
            position,
            rotation,
            time,
        }
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
            gl.delete_program(self.program);
            gl.delete_vertex_array(self.vertex_array);
        }
    }
}

impl Paintable for DandelionSeed {
    fn paint(&self, gl: &glow::Context, _screen_size: (f32, f32)) {
        unsafe {
            gl.use_program(Some(self.program));
            let time_location = gl.get_uniform_location(self.program, "time")
                .expect("could not find uniform location");
            let time = self.time.elapsed().as_secs_f32();
            gl.uniform_1_f32(Some(&time_location), time);
            gl.bind_vertex_array(Some(self.vertex_array));
            gl.draw_elements(glow::TRIANGLES, self.num_indices, glow::UNSIGNED_INT, 0);

            gl.bind_vertex_array(None);
        }
    }
}

struct Position3D {
    x: f64,
    y: f64,
    z: f64,
}

struct Rotation3D {
    x: f64,
    y: f64,
    z: f64,
}