use raylib::prelude::*;

pub struct FrameBuffer {
    pub width: u32,
    pub height: u32,
    pub color_buffer: Image,
    background_color: Color,
    current_color: Color
}

impl FrameBuffer {
    pub fn new(width: u32, height: u32, background_color: Color) -> Self {
        let color_buffer = Image::gen_image_color(width, height, background_color);
        return FrameBuffer {
            width,
            height,
            color_buffer,
            background_color,
            current_color: Color::WHITE
        }
    }
}
