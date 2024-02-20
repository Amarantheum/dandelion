use eframe::{egui::{self, Margin}, egui_glow, epaint::Color32, App, CreationContext, Frame};
use dandelion::DandelionSeed;
use egui::mutex::Mutex;
use std::{num, sync::Arc};
use scene::{Scene, Paintable};

mod dandelion;
mod obj;
mod ground;
mod scene;
mod affine_matrix;
mod kinect_tracker;

lazy_static::lazy_static! {
    static ref MID_SPINE: Mutex<(f32, f32, f32)> = Mutex::new((0.0, 0.0, 0.0));
}

struct DandelionApp {
    scene: Arc<Mutex<Scene>>,
}

impl DandelionApp {
    fn new(cc: &CreationContext) -> Self {
        let gl = cc.gl.as_ref()
            .expect("No OpenGL context");
        Self {
            scene: Arc::new(Mutex::new(Scene::new(gl))),
        }
    }

    fn draw_scene(&mut self, ui: &mut egui::Ui) {
        let rect = ui.available_rect_before_wrap();
        let scene = self.scene.clone();
        let callback = egui::PaintCallback {
            rect,
            callback: Arc::new(egui_glow::CallbackFn::new(move |_info, painter| {
                let mut scene = scene.lock();
                scene.update();
                scene.paint(painter.gl(), (rect.width(), rect.height()));
            }))
        };
        ui.painter().add(callback);
    }
}


impl App for DandelionApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut Frame) {
        let default_frame = egui::containers::Frame {
            inner_margin: Margin::same(0.0),
            outer_margin: Margin::same(0.0),
            rounding: egui::Rounding { nw: 1.0, ne: 1.0, sw: 1.0, se: 1.0 },
            shadow: eframe::epaint::Shadow::NONE,
            fill: Color32::BLACK,
            stroke: egui::Stroke::NONE,
        };
        egui::CentralPanel::default().frame(default_frame).show(ctx, |ui| {
            // Draw your UI here...
            egui::Frame::canvas(ui.style()).show(ui, |ui| {
                self.draw_scene(ui);
            });
        });
        ctx.request_repaint_after(std::time::Duration::from_secs_f64(1.0 / 60.0));
    }
}

fn main() {
    let mut native_options = eframe::NativeOptions::default();
    native_options.multisampling = 8;
    eframe::run_native("Dandelions", native_options, Box::new(|cc| Box::new(DandelionApp::new(cc))))
        .unwrap();
}

macro_rules! create_program{
    ($vs:expr, $fs:expr, $gl:ident) => {
        unsafe {
            let program = $gl.create_program().expect("Cannot create program");
            let vertex_shader_source = $vs;
            let fragment_shader_source = $fs;

            let vertex_shader = $gl.create_shader(glow::VERTEX_SHADER).expect("Cannot create vertex shader");
            $gl.shader_source(vertex_shader, vertex_shader_source);
            $gl.compile_shader(vertex_shader);
            assert!($gl.get_shader_compile_status(vertex_shader), "Cannot compile vertex shader: {}", $gl.get_shader_info_log(vertex_shader));

            let fragment_shader = $gl.create_shader(glow::FRAGMENT_SHADER).expect("Cannot create fragment shader");
            $gl.shader_source(fragment_shader, fragment_shader_source);
            $gl.compile_shader(fragment_shader);
            assert!($gl.get_shader_compile_status(fragment_shader), "Cannot compile fragment shader: {}", $gl.get_shader_info_log(fragment_shader));

            $gl.attach_shader(program, vertex_shader);
            $gl.attach_shader(program, fragment_shader);

            $gl.link_program(program);
            assert!($gl.get_program_link_status(program), "Cannot link program: {}", $gl.get_program_info_log(program));

            $gl.detach_shader(program, vertex_shader);
            $gl.detach_shader(program, fragment_shader);
            $gl.delete_shader(vertex_shader);
            $gl.delete_shader(fragment_shader);
            program
        }
    }
}

pub(crate) use create_program;