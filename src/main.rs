use camera::Camera;
use eframe::egui::{self, Key, Rgba};
use renderer::Ray;
use scene::Scene;
mod camera;
mod intersect;
mod light;
mod renderer;
mod scene;
mod sphere;

use std::{cell::RefCell, time::Instant};

struct FrameCounter {
    start_time: Instant,
    frame_count: u32,
    last_update: Instant,
}

impl FrameCounter {
    fn new() -> Self {
        Self {
            start_time: Instant::now(),
            frame_count: 0,
            last_update: Instant::now(),
        }
    }

    fn update(&mut self, frame_count: u32) {
        self.frame_count = frame_count;
        if Instant::now() - self.last_update > std::time::Duration::from_secs(1) {
            self.last_update = Instant::now();
        }
    }

    fn fps(&self) -> f32 {
        let elapsed = self.last_update.duration_since(self.start_time);
        self.frame_count as f32 / elapsed.as_secs_f32()
    }

    fn show(&mut self, ctx: &egui::Context, frame_count: u32) {
        self.update(frame_count);
        egui::Area::new(egui::Id::new("fps_area"))
            .fixed_pos(egui::pos2(10.0, 10.0))
            .show(ctx, |ui| {
                ui.set_min_width(150.0); //  Stop the frame count being put on a new line once it exceeds 9
                ui.label(format!("FPS: {:.2}", self.fps()));
                ui.label(format!("Frame Count: {:3.2}", frame_count));
            });
    }

}

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([820.0, 820.0]),
        ..Default::default()
    };
    eframe::run_native(
        "renderer",
        options,
        Box::new(|_| {
            // This gives us image support:
            Ok(Box::<RenderApp>::default())
        }),
    )
}

struct RenderApp {
    buffer: Vec<Vec<Rgba>>,
    camera: Camera,
    scene: Scene,
    frame_counter: RefCell<FrameCounter>,
    frame_count: RefCell<u32>,
}

impl Default for RenderApp {
    fn default() -> Self {
        let width = 500;
        let height = 800;

        let row = (0..width)
            .map(|_| Rgba::from_gray(0.0))
            .collect::<Vec<Rgba>>();

        let buffer = (0..height).map(|_| row.clone()).collect::<Vec<Vec<Rgba>>>();

        let ray_location = nalgebra::Vector3::new(-10.0, 0.0, 0.0);
        let ray_direction = nalgebra::Vector3::new(1.0, 0.0, 0.0);
        let origin_ray = Ray::new_preserve(ray_location, ray_direction);

        let camera = Camera {
            location: origin_ray,
            width,
            height,
        };

        RenderApp {
            buffer,
            camera,
            scene: Scene::pondering_orbs(),
            frame_counter: RefCell::new(FrameCounter::new()),
            frame_count: RefCell::new(0),
        }
    }
}

#[allow(deprecated)]
impl eframe::App for RenderApp {
    fn update(&mut self, ctx: &egui::Context, _: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            self.update_buffer_sharedstate();
            let img =
                egui_extras::image::RetainedImage::from_color_image("text", self.buffer_to_image());
            img.show(ui);

            *self.frame_count.borrow_mut() += 1;
            self.frame_counter.borrow_mut().show(ctx, *self.frame_count.borrow());
            ctx.input(|inputs| {
                for pressed in &inputs.keys_down {
                    match pressed {
                        Key::W => self.camera.location.origin.x += 0.1,
                        Key::S => self.camera.location.origin.x -= 0.1,
                        Key::A => self.camera.location.origin.y -= 0.1,
                        Key::D => self.camera.location.origin.y += 0.1,
                        Key::Z => self.camera.location.origin.z += 0.1,
                        Key::X => self.camera.location.origin.z -= 0.1,
                        Key::ArrowLeft => {
                            let x = self.camera.location.direction.x;
                            let y = self.camera.location.direction.y;

                            let theta = y.atan2(x);
                            let r = (x.powi(2) + y.powi(2)).sqrt();

                            let theta_1 = theta - 0.01;

                            let x_1 = theta_1.cos() * r;
                            let y_1 = theta_1.sin() * r;

                            self.camera.location.direction.x = x_1;
                            self.camera.location.direction.y = y_1;
                            println!("{:?}", self.camera.location.direction);
                        }
                        Key::ArrowRight => {
                            let x = self.camera.location.direction.x;
                            let y = self.camera.location.direction.y;

                            let theta = y.atan2(x);
                            let r = (x.powi(2) + y.powi(2)).sqrt();

                            let theta_1 = theta + 0.01;

                            let x_1 = theta_1.cos() * r;
                            let y_1 = theta_1.sin() * r;

                            self.camera.location.direction.x = x_1;
                            self.camera.location.direction.y = y_1;
                            println!("{:?}", self.camera.location.direction);
                        }
                        _ => (),
                    }
                }
            });
        });
    }
}

impl RenderApp {
    fn buffer_to_image(&self) -> egui::ColorImage {
        let mut flattened = vec![];
        for row in &self.buffer {
            for pixel in row {
                let values = pixel.to_srgba_unmultiplied();
                for v in values {
                    flattened.push(v);
                }
            }
        }
        egui::ColorImage::from_rgba_unmultiplied(
            [self.buffer[0].len(), self.buffer.len()],
            &flattened.as_slice(),
        )
    }

    fn update_buffer_sharedstate(&mut self) {
        //self.buffer = self.camera.create_buffer_parallel(self.scene.clone());
        self.buffer = self.camera.create_buffer(&self.scene);
    }
}
