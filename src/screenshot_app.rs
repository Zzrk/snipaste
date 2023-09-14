use egui::ColorImage;
use egui_extras::RetainedImage;
use image::{imageops, ImageBuffer, Rgba};
use screenshots::Screen;

pub struct ScreenshotApp {
    // 截图 buffer
    capture_buffer: ImageBuffer<Rgba<u8>, Vec<u8>>,
    // 截图 image 用于渲染
    capture_image: RetainedImage,
    // 鼠标位置
    pos: egui::Pos2,
    // 鼠标周围的截图片段
    rect_image: Option<RetainedImage>,
}

impl Default for ScreenshotApp {
    fn default() -> Self {
        let capture_buffer = capture_screens();
        let capture_image = buffer2retained_image("capture.png", &capture_buffer);
        Self {
            capture_buffer,
            capture_image,
            pos: egui::Pos2 { x: 0.0, y: 0.0 },
            rect_image: None,
        }
    }
}

impl eframe::App for ScreenshotApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // 保存当前鼠标位置
        let pos = ctx.pointer_hover_pos().unwrap_or(self.pos);
        self.pos = pos;
        let x = pos.x as u32;
        let y = pos.y as u32;

        // 鼠标周围的截图片段
        let subimg = imageops::crop(&mut self.capture_buffer, x, y, 200, 100);
        self.rect_image = Some(buffer2retained_image("rect.png", &subimg.to_image()));

        // 鼠标当前的颜色
        let pixel = self.capture_buffer.get_pixel(x, y);

        egui::TopBottomPanel::top("capture")
            .max_height(400.0)
            .show(ctx, |ui| {
                // 渲染截图
                self.capture_image.show(ui);

                // 渲染截图片段
                egui::Window::new("rect")
                    .current_pos(egui::Pos2 {
                        x: pos.x + 10.0,
                        y: pos.y + 10.0,
                    })
                    .show(ctx, |ui| {
                        if let Some(rc) = &self.rect_image {
                            rc.show(ui);
                            ui.horizontal(|ui| {
                                ui.label("color:");
                                ui.label(format!("{:?}", pixel));
                            });
                        }
                    });
            });
    }
}

pub fn capture_screens() -> ImageBuffer<Rgba<u8>, Vec<u8>> {
    let screens = Screen::all().unwrap();
    screens[0].capture().unwrap()
}

fn buffer2color_image(buffer: &ImageBuffer<Rgba<u8>, Vec<u8>>) -> ColorImage {
    ColorImage::from_rgba_unmultiplied(
        [buffer.width() as usize, buffer.height() as usize],
        buffer.as_ref(),
    )
}

fn color_image2retained_image(name: &str, color_image: ColorImage) -> RetainedImage {
    RetainedImage::from_color_image(name, color_image)
}

fn buffer2retained_image(name: &str, buffer: &ImageBuffer<Rgba<u8>, Vec<u8>>) -> RetainedImage {
    color_image2retained_image(name, buffer2color_image(&buffer))
}
