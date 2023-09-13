use screenshots::Screen;

#[derive(Default)]
pub struct MainApp {}

impl eframe::App for MainApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            if ui.button("Color Picker").clicked() {
                // TODO: 创建窗口的事件较长
                capture_screens();
                frame.close();
            }
        });
    }
}

pub fn capture_screens() {
    let screens = Screen::all().unwrap();
    let image = screens[0].capture().unwrap();
    image.save("target/capture.png").unwrap();
}
