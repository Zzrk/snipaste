use egui::ColorImage;
use egui_extras::RetainedImage;
use image::{ImageBuffer, Rgba};
use screenshots::Screen;

pub struct ScreenshotApp {
    image: RetainedImage,
}

impl Default for ScreenshotApp {
    fn default() -> Self {
        // 从 Buffer 创建 ColorImage
        let image = capture_screens();
        let color_image = ColorImage::from_rgba_unmultiplied(
            [image.width() as usize, image.height() as usize],
            image.as_ref(),
        );
        // 从 ColorImage 创建 RetainedImage
        Self {
            image: RetainedImage::from_color_image("capture.png", color_image),
        }
    }
}

impl eframe::App for ScreenshotApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| self.image.show(ui));
    }
}

pub fn capture_screens() -> ImageBuffer<Rgba<u8>, Vec<u8>> {
    let screens = Screen::all().unwrap();
    screens[0].capture().unwrap()
}
