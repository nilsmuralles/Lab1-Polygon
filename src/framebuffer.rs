use raylib::prelude::*;

pub struct FrameBuffer {
    pub width: i32,
    pub height: i32,
    pub color_buffer: Image,
    background_color: Color,
    current_color: Color
}

impl FrameBuffer {
    pub fn new(width: i32, height: i32, background_color: Color) -> Self {
        let color_buffer = Image::gen_image_color(width, height, background_color);
        return FrameBuffer {
            width,
            height,
            color_buffer,
            background_color,
            current_color: Color::WHITE
        }
    }

    pub fn clear(&mut self) {
        self.color_buffer = Image::gen_image_color(self.width, self.height, self.background_color);
    }

    pub fn set_pixel(&mut self, x: i32, y: i32) {
        if x > self.width || x < 0 {
           println!("Out of bounds on x"); 
        }
        if y > self.height || y < 0 {
           println!("Out of bounds on y"); 
        }
        self.color_buffer.draw_pixel(x, y, self.current_color);
    }

    pub fn set_background_color(&mut self, color: Color) {
        self.background_color = color;
    }

    pub fn set_current_color(&mut self, color: Color) {
        self.current_color = color;
    }

    pub fn render_to_file(&self, file_path: &str) {
        self.color_buffer.export_image(file_path);
        println!("Image saved successfully as '{}'!", file_path);
    }

    pub fn render_to_bmp(&self, file_path: &str) {
        self.color_buffer.export_image(file_path);
        println!("BMP file saved successfully as '{}'!", file_path);
    }
}
