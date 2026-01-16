mod jssp;
mod gui;

use eframe::egui;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1200.0, 800.0])
            .with_title("JSSP Scheduler - Greedy Algorithm"),
        ..Default::default()
    };
    
    eframe::run_native(
        "JSSP Scheduler",
        options,
        Box::new(|_cc| Ok(Box::new(gui::JsspApp::default()))),
    )
}
