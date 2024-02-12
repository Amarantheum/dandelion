use eframe::{egui_glow, glow::HasContext};
use egui_glow::glow;

pub struct JoinedOBJ {
    pub vao: glow::VertexArray,
    pub num_indices: i32,
}

impl JoinedOBJ {
    pub unsafe fn new(gl: &glow::Context, path: &str, progam: glow::Program) -> Result<Self, Box<dyn std::error::Error>> {
        let (models, _) = tobj::load_obj(path, &tobj::GPU_LOAD_OPTIONS)?;
        // vertices are [v_x, v_y, v_z, n_x, n_y, n_z]
        let mut vertices_normals_combined = Vec::new();
        let mut indices_combined = Vec::new();
        let mut num_indices = 0;
        let mut index_offset = 0;
        for model in models {
            let mesh = &model.mesh;
            let num_vertices = mesh.positions.len() / 3;
            let num_normals = mesh.normals.len() / 3;
            assert!(num_vertices == num_normals);
            for i in 0..num_vertices {
                vertices_normals_combined.push(mesh.positions[i * 3]);
                vertices_normals_combined.push(mesh.positions[i * 3 + 1]);
                vertices_normals_combined.push(mesh.positions[i * 3 + 2]);
                vertices_normals_combined.push(mesh.normals[i * 3]);
                vertices_normals_combined.push(mesh.normals[i * 3 + 1]);
                vertices_normals_combined.push(mesh.normals[i * 3 + 2]);
            }
            for i in 0..mesh.indices.len() {
                indices_combined.push(mesh.indices[i] + index_offset);
            }
            index_offset += num_vertices as u32;
            num_indices += mesh.indices.len() as i32;
        }
        let vao = gl.create_vertex_array().expect("Cannot create vertex array");
        gl.bind_vertex_array(Some(vao));

        let vbo = gl.create_buffer().expect("Cannot create buffer");
        gl.bind_buffer(glow::ARRAY_BUFFER, Some(vbo));
        gl.buffer_data_u8_slice(glow::ARRAY_BUFFER, bytemuck::cast_slice(&vertices_normals_combined), glow::STATIC_DRAW);

        let ebo = gl.create_buffer().expect("Cannot create buffer");
        gl.bind_buffer(glow::ELEMENT_ARRAY_BUFFER, Some(ebo));
        gl.buffer_data_u8_slice(glow::ELEMENT_ARRAY_BUFFER, bytemuck::cast_slice(&indices_combined), glow::STATIC_DRAW);

        let position_attrib_location = gl.get_attrib_location(progam, "position").unwrap();
        let normal_attrib_location = gl.get_attrib_location(progam, "normal").unwrap();

        gl.enable_vertex_attrib_array(position_attrib_location);
        gl.vertex_attrib_pointer_f32(position_attrib_location, 3, glow::FLOAT, false, 6 * std::mem::size_of::<f32>() as i32, 0);
        gl.enable_vertex_attrib_array(normal_attrib_location);
        gl.vertex_attrib_pointer_f32(normal_attrib_location, 3, glow::FLOAT, false, 6 * std::mem::size_of::<f32>() as i32, 3 * std::mem::size_of::<f32>() as i32);

        gl.bind_vertex_array(None);

        Ok(Self {
            vao,
            num_indices,
        })
    }
}