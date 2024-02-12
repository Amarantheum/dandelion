use eframe::{egui::{self, Margin}, egui_glow, epaint::Color32, App, CreationContext, Frame};
use dandelion::DandelionSeed;
use egui::mutex::Mutex;
use std::{num, sync::Arc};
use scene::{Scene, Paintable};

mod dandelion;
mod obj;
mod ground;
mod scene;

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
                scene.lock().paint(painter.gl(), (rect.width(), rect.height()));
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
    let (models, _) = tobj::load_obj("./DandelionSeed.obj", &tobj::GPU_LOAD_OPTIONS).expect("Cannot load DandelionSeed.obj");
    let min_x = models[0].mesh.positions.iter().chain(models[1].mesh.positions.iter().step_by(3)).step_by(3).min_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();
    let min_y = models[0].mesh.positions.iter().chain(models[1].mesh.positions.iter().skip(1).step_by(3)).skip(1).step_by(3).min_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();
    let max_x = models[0].mesh.positions.iter().chain(models[1].mesh.positions.iter().step_by(3)).step_by(3).max_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();
    let max_y = models[0].mesh.positions.iter().chain(models[1].mesh.positions.iter().skip(1).step_by(3)).skip(1).step_by(3).max_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();
    println!("min_x: {}, min_y: {}, max_x: {}, max_y: {}", min_x, min_y, max_x, max_y);

    let num_indices = models[0].mesh.indices.len() as i32;
    let num_normals = models[0].mesh.normals.len() as i32;
    let num_vertices = models[0].mesh.positions.len() as i32;
    println!("num_indices: {}, num_normals: {}, num_vertices: {}", num_indices, num_normals, num_vertices);
    let mut native_options = eframe::NativeOptions::default();
    native_options.multisampling = 16;
    eframe::run_native("Dandelions", native_options, Box::new(|cc| Box::new(DandelionApp::new(cc))))
        .unwrap();
}
