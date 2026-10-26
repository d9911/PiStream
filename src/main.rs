#![cfg_attr(target_os = "windows", windows_subsystem = "windows")]

mod installer_lock;

use pi_stream_terminal::desktop::{appearance, App};

fn main() -> eframe::Result {
    let _installation_lock = installer_lock::acquire("org.d9911.pistreamterminal")
        .map_err(|error| eframe::Error::AppCreation(Box::new(error)))?;
    let options = eframe::NativeOptions {
        viewport: eframe::egui::ViewportBuilder::default()
            .with_title("PiStreamTerminal")
            .with_app_id("org.d9911.pistreamterminal")
            .with_inner_size([1120.0, 780.0])
            .with_min_inner_size([640.0, 580.0])
            .with_icon(appearance::app_icon()),
        renderer: eframe::Renderer::Glow,
        ..Default::default()
    };
    eframe::run_native(
        "PiStreamTerminal",
        options,
        Box::new(|cc| Ok(Box::new(App::new(cc)))),
    )
}
