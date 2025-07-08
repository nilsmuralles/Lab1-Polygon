use polygon::Polygon;
use raylib::prelude::*;
use framebuffer::FrameBuffer;
use vertex::Vertex;

mod framebuffer;
mod line;
mod vertex;
mod polygon;

fn main() {
    let mut fb = FrameBuffer::new(500, 500, Color::WHITE);
    fb.set_background_color(Color::WHITE);
    fb.clear();

    let p1 = Polygon::new(vec![
        Vertex::new(50, 50),
        Vertex::new(120, 60),
        Vertex::new(100, 120),
        Vertex::new(60, 100)
    ], Color::RED, Color::RED);
    p1.draw_polygon(&mut fb);

    fb.render_to_file("result.png");
}
