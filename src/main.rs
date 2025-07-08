use raylib::prelude::*;
use framebuffer::FrameBuffer;
use line::line;
use vertex::Vertex;

mod framebuffer;
mod line;
mod vertex;

fn main() {
    let mut fb = FrameBuffer::new(500, 500, Color::WHITE);
    fb.set_background_color(Color::BLUE);
    fb.clear();
    fb.set_current_color(Color::RED);
    let v1 = Vertex::new(20, 20);
    let v2 = Vertex::new(130, 60);
    line(&mut fb, &v1, &v2);
    line(&mut fb, &Vertex::new(140, 40), &Vertex::new(250, 65));
    line(&mut fb, &Vertex::new(250, 250), &Vertex::new(100, 230));
    line(&mut fb, &Vertex::new(250, 250), &Vertex::new(200, 330));
    fb.render_to_file("result.png");
}
