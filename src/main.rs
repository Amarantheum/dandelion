use dancer_mock::spawn_dancer_mock;
use eframe::{egui::{self, Margin, viewport::ViewportCommand}, egui_glow, epaint::Color32, App, CreationContext, Frame};
use dandelion::DandelionSeed;
use egui::mutex::Mutex;
use std::{num, sync::Arc};
use scene::{Scene, Paintable};
use kinect_tracker::spawn_osc_handler;

mod dandelion;
mod obj;
mod ground;
mod scene;
mod affine_matrix;
mod kinect_tracker;
mod dancer_mock;
mod color;
//mod dandelion_joined;

lazy_static::lazy_static! {
    pub static ref BODY1_BASE_SPINE: Mutex<[f32; 3]> = Mutex::new([0.0, 0.0, 0.0]);
    pub static ref BODY2_BASE_SPINE: Mutex<[f32; 3]> = Mutex::new([0.0, 0.0, 0.0]);
    pub static ref BODY1_HEAD: Mutex<[f32; 3]> = Mutex::new([0.0, 0.0, 0.0]);
    pub static ref BODY2_HEAD: Mutex<[f32; 3]> = Mutex::new([0.0, 0.0, 0.0]);
}

const AFFECTION_STEP_SIZE: f32 = 0.001;

#[derive(Debug, Clone, Copy)]
struct DandelionState {
    pub started: bool,
    pub brightness: f32,
    pub affection: f32,
    pub scene_3: bool,
    pub dancing_brightness: f32,
    pub scene_5: bool,
    pub drift_strength: f32,
}

struct DandelionApp {
    scene: Arc<Mutex<Scene>>,
    fullscreen: bool,
    state: DandelionState,
}

impl DandelionApp {
    fn new(cc: &CreationContext) -> Self {
        let gl = cc.gl.as_ref()
            .expect("No OpenGL context");
        let state = DandelionState {
            started: false,
            brightness: 0.0,
            affection: 0.0,
            scene_3: false,
            dancing_brightness: 0.0,
            scene_5: false,
            drift_strength: 0.0,

        };
        Self {
            scene: Arc::new(Mutex::new(Scene::new(gl))),
            fullscreen: false,
            state,
        }
    }

    fn draw_scene(&mut self, ui: &mut egui::Ui) {
        let rect = ui.available_rect_before_wrap();
        let scene = self.scene.clone();
        let mut motion_vector = [0; 2];
        if ui.input(|i| i.key_down(egui::Key::ArrowUp)) {
            motion_vector[0] += 1;
        }
        if ui.input(|i| i.key_down(egui::Key::ArrowDown)) {
            motion_vector[0] -= 1;
        }
        if ui.input(|i| i.key_down(egui::Key::ArrowRight)) {
            motion_vector[1] += 1;
        }
        if ui.input(|i| i.key_down(egui::Key::ArrowLeft)) {
            motion_vector[1] -= 1;
        }

        if ui.input(|i| i.key_pressed(egui::Key::Num3)) {
            self.state.scene_3 = !self.state.scene_3;
        }
        if ui.input(|i| i.key_pressed(egui::Key::Num5)) {
            self.state.scene_5 = !self.state.scene_5;
        }

        if ui.input(|i| i.key_pressed(egui::Key::Space)) {
            self.state.started = !self.state.started;
        }

        if ui.input(|i| i.key_down(egui::Key::W)) {
            if self.state.affection < 1.0 {
                self.state.affection += AFFECTION_STEP_SIZE;
            }
        }
        if ui.input(|i| i.key_down(egui::Key::S)) {
            if self.state.affection > 0.0 {
                self.state.affection -= AFFECTION_STEP_SIZE * 10.0;
            }
        }

        let dancing_transition_speed = 0.01;
        if self.state.started && self.state.scene_3 && self.state.dancing_brightness < 1.0 {
            self.state.dancing_brightness += dancing_transition_speed;
            if self.state.brightness > 0.0 {
                self.state.brightness -= dancing_transition_speed;
            }
        }
        if self.state.started && !self.state.scene_3 && self.state.dancing_brightness > 0.0 {
            self.state.dancing_brightness = 0_f32.max(self.state.dancing_brightness - dancing_transition_speed);
            if self.state.brightness < 1.0 {
                self.state.brightness += dancing_transition_speed;
            }
        }

        if self.state.started && self.state.brightness < 1.0 && self.state.dancing_brightness == 0.0 {
            self.state.brightness += 0.01;
        }
        if !self.state.started && self.state.brightness > 0.0 && self.state.dancing_brightness == 0.0 {
            self.state.brightness -= 0.01;
        }

        if self.state.started && self.state.scene_5 && self.state.drift_strength < 1.0 {
            self.state.drift_strength += 0.0001;
        }
        if self.state.started && !self.state.scene_5 && self.state.drift_strength > 0.0 {
            self.state.drift_strength -= 0.0001;
        }
        //println!("brightness: {}", self.state.brightness);
        let state = self.state;

        let callback = egui::PaintCallback {
            rect,
            callback: Arc::new(egui_glow::CallbackFn::new(move |_info, painter| {
                let mut scene = scene.lock();
                scene.update(state);
                scene.camera_pos[2] += motion_vector[0] as f32 * -0.01;
                scene.camera_pos[0] += motion_vector[1] as f32 * 0.01;
                scene.paint(painter.gl(), (rect.width(), rect.height()), state);
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
            if ui.input(|i| i.key_pressed(egui::Key::Escape)) {
                ctx.send_viewport_cmd(ViewportCommand::Close);
            }
            if ui.input(|i| i.key_pressed(egui::Key::F11)) {
                self.fullscreen = !self.fullscreen;
                ctx.send_viewport_cmd(ViewportCommand::Fullscreen(self.fullscreen));
                //ctx.send_viewport_cmd(ViewportCommand::CursorVisible(!self.fullscreen));
            }
            
            egui::Frame::canvas(ui.style()).show(ui, |ui| {
                self.draw_scene(ui);
            });
        });
        ctx.request_repaint_after(std::time::Duration::from_secs_f64(1.0 / 60.0));
    }
}

fn main() {
    spawn_osc_handler().unwrap();
    spawn_dancer_mock().unwrap();
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