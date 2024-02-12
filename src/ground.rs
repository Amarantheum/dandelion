use eframe::{egui_glow, glow::HasContext};
use egui_glow::glow;
use core::time;
use std::time::Instant;

use crate::{obj::JoinedOBJ, scene::Paintable};

pub struct Ground {
    program: glow::Program,
    obj: JoinedOBJ,
    time: Instant,
}

impl Ground {
    pub fn new(gl: &glow::Context) -> Self {
        let program = unsafe { Self::create_program(gl) };
        let obj = unsafe { JoinedOBJ::new(gl, "./Ground.obj", program).expect("Cannot create JoinedOBJ") };
        Self {
            program,
            obj,
            time: Instant::now(),
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

    
}

impl Paintable for Ground {
    fn paint(&self, gl: &glow::Context, screen_size: (f32, f32)) {
        unsafe {
            gl.use_program(Some(self.program));
            let screen_size_location = gl.get_uniform_location(self.program, "screen_size").expect("Cannot get uniform location");
            gl.uniform_2_f32(Some(&screen_size_location), screen_size.0, screen_size.1);
            let time_location = gl.get_uniform_location(self.program, "time").expect("Cannot get uniform location");
            let time = self.time.elapsed().as_secs_f32();
            gl.uniform_1_f32(Some(&time_location), time);
            gl.bind_vertex_array(Some(self.obj.vao));
            gl.draw_elements(glow::TRIANGLES, self.obj.num_indices, glow::UNSIGNED_INT, 0);

            gl.bind_vertex_array(None);
        }
    }
}