use egui_extras::RetainedImage;

pub struct ScreenshotApp {
    image: RetainedImage,
}

impl Default for ScreenshotApp {
    fn default() -> Self {
        Self {
            image: RetainedImage::from_image_bytes(
                "capture.png",
                include_bytes!("../target/capture.png"),
            )
            .unwrap(),
        }
    }
}

impl eframe::App for ScreenshotApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| self.image.show(ui));
    }
}
