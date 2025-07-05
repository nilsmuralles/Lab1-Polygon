use framebuffer::FrameBuffer;
use raylib::prelude::*;

mod framebuffer;

fn main() {
    let mut fb = FrameBuffer::new(500, 500, Color::WHITE);
    fb.set_background_color(Color::BLUE);
    fb.clear();
    fb.set_current_color(Color::RED);
    fb.set_pixel(100, 100);
    fb.set_pixel(200, 200);
    fb.render_to_file("result.png");
}
