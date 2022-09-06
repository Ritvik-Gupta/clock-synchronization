mod app;
pub mod state;

use app::TemplateApp;
use eframe::egui;

fn main() {
    eframe::run_native(
        "Clocking",
        eframe::NativeOptions {
            initial_window_size: Some(egui::Vec2::new(900.0, 600.0)),
            resizable: false,
            decorated: true,
            ..eframe::NativeOptions::default()
        },
        Box::new(|cc| Box::new(TemplateApp::new(cc))),
    );
}
