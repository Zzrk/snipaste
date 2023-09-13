#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

fn main() -> eframe::Result<()> {
    let default_options = eframe::NativeOptions::default();
    let fullscreen_options = eframe::NativeOptions {
        fullscreen: true,
        ..Default::default()
    };

    eframe::run_native(
        "snipaste",
        default_options,
        Box::new(|_cc| Box::new(snipaste::MainApp::default())),
    )?;

    eframe::run_native(
        "screenshot",
        fullscreen_options,
        Box::new(|_cc| Box::new(snipaste::ScreenshotApp::default())),
    )
}
