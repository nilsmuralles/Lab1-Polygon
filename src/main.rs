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
    let bg_color = Color::WHITE;
    fb.set_background_color(bg_color);
    fb.clear();

    let p1 = Polygon::new(vec![
        Vertex::new(165, 380), Vertex::new(185, 360), Vertex::new(180, 330), Vertex::new(207, 345), 
        Vertex::new(233, 330), Vertex::new(230, 360), Vertex::new(250, 380), Vertex::new(220, 385), 
        Vertex::new(205, 410), Vertex::new(193, 383),
    ], 
        Color::RED, 
        Color::BLACK
    );
    p1.draw(&mut fb);

    fb.render_to_file("out.png");
    fb.render_to_bmp("out.bmp");
}
