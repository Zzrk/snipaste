use egui::ColorImage;
use egui_extras::RetainedImage;
use image::{ImageBuffer, Rgba};
use screenshots::Screen;

pub struct ScreenshotApp {
    // 截图 buffer
    capture_buffer: ImageBuffer<Rgba<u8>, Vec<u8>>,
    // 截图 image 用于渲染
    capture_image: RetainedImage,
    // 鼠标位置
    pos: egui::Pos2,
}

impl Default for ScreenshotApp {
    fn default() -> Self {
        let capture_buffer = capture_screens();
        let capture_image = buffer2retained_image("capture.png", &capture_buffer);
        Self {
            capture_buffer,
            capture_image,
            pos: egui::Pos2 { x: 0.0, y: 0.0 },
        }
    }
}

impl eframe::App for ScreenshotApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // 自定义 Window 样式
        let mut style = (*ctx.style()).clone();
        style.spacing.window_margin = egui::style::Margin {
            top: 0.0,
            bottom: 0.0,
            left: 0.0,
            right: 0.0,
        };
        ctx.set_style(style);

        // 渲染截图
        egui::Window::new("capture")
            .title_bar(false)
            .show(ctx, |ui| {
                self.capture_image.show(ui);
            });

        // 保存当前鼠标位置
        let pos = ctx.pointer_hover_pos().unwrap_or(self.pos);
        self.pos = pos;
        let pos_x = pos.x as u32;
        let pos_y = pos.y as u32;

        // 鼠标周围的截图片段
        let subimg = ImageBuffer::from_fn(200, 100, |x, y| {
            let sub_x = pos_x as i32 - 100 + x as i32;
            let sub_y = pos_y as i32 - 50 + y as i32;
            if sub_x < 0
                || sub_y < 0
                || sub_x >= self.capture_image.width() as i32
                || sub_y >= self.capture_image.height() as i32
            {
                // TODO: 图片范围之外的区域改为透明表示
                image::Rgba([255, 255, 255, 0])
            } else {
                *self.capture_buffer.get_pixel(sub_x as u32, sub_y as u32)
            }
        });
        let rect_image = buffer2retained_image("rect.png", &subimg);

        // 鼠标当前的颜色
        let pixel = self.capture_buffer.get_pixel(pos_x, pos_y);

        // 鼠标当前颜色的图片
        let color_buffer = ImageBuffer::from_pixel(10, 10, *pixel);
        let color_image = buffer2retained_image("color.png", &color_buffer);

        // 渲染截图片段
        egui::Window::new("rect")
            .title_bar(false)
            .current_pos(egui::Pos2 {
                x: pos.x + 10.0,
                y: pos.y + 10.0,
            })
            .show(ctx, |ui| {
                rect_image.show(ui);
                ui.horizontal(|ui| {
                    ui.label("color:");
                    ui.label(format!("{:?}", pixel));
                    color_image.show(ui);
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
