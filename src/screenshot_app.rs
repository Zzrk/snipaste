use arboard::{Clipboard, ImageData};
use egui::containers::Frame;
use egui::{Color32, ColorImage, Key, Modifiers, Rect};
use egui_extras::RetainedImage;
use image::{imageops, GenericImageView, ImageBuffer, Rgba};
use rfd::FileDialog;
use screenshots::Screen;
use std::borrow::Cow;

pub struct ScreenshotApp {
    // 截图 buffer
    capture_buffer: ImageBuffer<Rgba<u8>, Vec<u8>>,
    // 截图 image 用于渲染
    capture_image: RetainedImage,
    // 当前鼠标位置
    cur_pos: egui::Pos2,
    // 起始鼠标位置
    start_pos: Option<egui::Pos2>,
    // 终点鼠标位置
    end_pos: Option<egui::Pos2>,
}

impl Default for ScreenshotApp {
    fn default() -> Self {
        let capture_buffer = capture_screens();
        let capture_image = buffer2retained_image("capture.png", &capture_buffer);
        Self {
            capture_buffer,
            capture_image,
            cur_pos: egui::Pos2 { x: 0.0, y: 0.0 },
            start_pos: None,
            end_pos: None,
        }
    }
}

impl ScreenshotApp {
    // 渲染截图
    fn show_capture_image(&self, ctx: &egui::Context) {
        // 避免了 window 之间的遮挡
        egui::CentralPanel::default()
            .frame(Frame::none().fill(Color32::WHITE))
            .show(ctx, |ui| {
                self.capture_image.show(ui);
                self.show_shadow_area(ctx, ui.painter())
            });
        // egui::Window::new("capture")
        //     .title_bar(false)
        //     .resizable(false)
        //     .show(ctx, |ui| {
        //         self.capture_image.show(ui);
        //         self.show_shadow_area(ctx, ui.painter())
        //     });
    }

    // 渲染截图片段
    fn show_rect_image(&mut self, ctx: &egui::Context) {
        let pos = self.cur_pos;
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
            .resizable(false)
            .current_pos(egui::Pos2 {
                x: pos.x + 10.0,
                y: pos.y + 10.0,
            })
            .default_width(200.0)
            .show(ctx, |ui| {
                ui.vertical_centered(|ui| {
                    rect_image.show(ui);
                    ui.label(format!("{pos_x}, {pos_y}"));
                    ui.label(format!(
                        "{}, {}, {}, {}",
                        pixel[0], pixel[1], pixel[2], pixel[3]
                    ));
                    color_image.show(ui);
                });
            });
    }

    // 渲染遮罩区域
    fn show_shadow_area(&self, ctx: &egui::Context, painter: &egui::Painter) {
        if let Some(start_pos) = self.start_pos {
            let screen_rect = ctx.screen_rect();
            let cur_pos = self.end_pos.unwrap_or(self.cur_pos);
            // 计算左上角的位置
            let tl_pos = egui::Pos2 {
                x: start_pos.x.min(cur_pos.x),
                y: start_pos.y.min(cur_pos.y),
            };
            // 计算右下角的位置
            let br_pos = egui::Pos2 {
                x: start_pos.x.max(cur_pos.x),
                y: start_pos.y.max(cur_pos.y),
            };
            // 左上
            painter.rect_filled(
                Rect::from_two_pos(screen_rect.min, egui::pos2(tl_pos.x, br_pos.y)),
                0.0,
                Color32::from_black_alpha(170),
            );
            // 右上
            painter.rect_filled(
                Rect::from_two_pos(
                    egui::pos2(tl_pos.x, screen_rect.min.y),
                    egui::pos2(screen_rect.max.x, tl_pos.y),
                ),
                0.0,
                Color32::from_black_alpha(170),
            );
            // 左下
            painter.rect_filled(
                Rect::from_two_pos(
                    egui::pos2(screen_rect.min.x, br_pos.y),
                    egui::pos2(br_pos.x, screen_rect.max.y),
                ),
                0.0,
                Color32::from_black_alpha(170),
            );
            // 右下
            painter.rect_filled(
                Rect::from_two_pos(egui::pos2(br_pos.x, tl_pos.y), screen_rect.max),
                0.0,
                Color32::from_black_alpha(170),
            );
        }
    }
}

impl eframe::App for ScreenshotApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        // 自定义 Window 样式
        let mut style = (*ctx.style()).clone();
        style.spacing.window_margin = egui::style::Margin {
            top: 0.0,
            bottom: 0.0,
            left: 0.0,
            right: 0.0,
        };
        style.visuals.window_rounding = egui::Rounding {
            nw: 0.0,
            ne: 0.0,
            se: 0.0,
            sw: 0.0,
        };
        ctx.set_style(style);

        // 保存当前鼠标位置
        let cur_pos = ctx.pointer_hover_pos().unwrap_or(self.cur_pos);
        self.cur_pos = cur_pos;

        // 记录鼠标起始位置
        if self.start_pos.is_none() && ctx.input(|i| i.pointer.primary_pressed()) {
            self.start_pos = ctx.pointer_interact_pos();
        }

        // 记录鼠标终点位置
        if self.end_pos.is_none() && ctx.input(|i| i.pointer.primary_released()) {
            self.end_pos = ctx.pointer_interact_pos();
        }

        // 已经获取截图区域
        if self.start_pos.is_some() && self.end_pos.is_some() {
            let rect = egui::Rect::from_two_pos(self.start_pos.unwrap(), self.end_pos.unwrap());
            let tl_pos = rect.left_top();
            let image = imageops::crop(
                &mut self.capture_buffer,
                tl_pos.x as u32,
                tl_pos.y as u32,
                rect.width() as u32,
                rect.height() as u32,
            );
            // Ctrl+S 保存截图
            if ctx.input_mut(|i| i.consume_key(Modifiers::CTRL, Key::S)) {
                if let Some(path) = FileDialog::new().set_file_name("capture.png").save_file() {
                    println!("{}", path.display());
                    image.to_image().save(path.as_path()).unwrap();
                    self.start_pos = None;
                    self.end_pos = None;
                }
            }

            // Ctrl+C 复制到剪切板
            if ctx.input_mut(|i| i.consume_key(Modifiers::CTRL, Key::C)) {
                println!("图片复制到剪切板");
                let mut clipboard = Clipboard::new().unwrap();
                let mut buffer = Vec::new();
                for pixel in image.pixels() {
                    buffer.push(pixel.2[0]);
                    buffer.push(pixel.2[1]);
                    buffer.push(pixel.2[2]);
                    buffer.push(pixel.2[3]);
                }
                let image_data = ImageData {
                    width: image.width() as usize,
                    height: image.height() as usize,
                    bytes: Cow::from(&buffer),
                };
                clipboard.set_image(image_data).unwrap();
            }
        }

        // ESC 或鼠标右键返回上一步
        if ctx.input(|i| i.key_released(Key::Escape) || i.pointer.secondary_released()) {
            if self.start_pos.is_some() || self.end_pos.is_some() {
                self.start_pos = None;
                self.end_pos = None;
            } else {
                frame.close();
            }
        }

        self.show_capture_image(ctx);
        self.show_rect_image(ctx);
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
