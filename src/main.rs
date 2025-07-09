use polygon::Polygon;
use raylib::prelude::*;
use framebuffer::FrameBuffer;
use vertex::Vertex;

mod framebuffer;
mod line;
mod vertex;
mod polygon;

fn main() {
    let mut fb = FrameBuffer::new(800, 800, Color::WHITE);
    let bg_color = Color::BLACK;
    fb.set_background_color(bg_color);
    fb.clear();

    let p1 = Polygon::new(vec![
        Vertex::new(165, 380), Vertex::new(185, 360), Vertex::new(180, 330), Vertex::new(207, 345), 
        Vertex::new(233, 330), Vertex::new(230, 360), Vertex::new(250, 380), Vertex::new(220, 385), 
        Vertex::new(205, 410), Vertex::new(193, 383),
    ], 
        Color::YELLOW, 
        Color::WHITE
    );
    p1.draw(&mut fb);

    let p2 = Polygon::new(vec![
        Vertex::new(321, 335), Vertex::new(288, 286), Vertex::new(339, 251), Vertex::new(374, 302),
    ],
        Color::BLUE,
        Color::WHITE,
    );
    p2.draw(&mut fb);

    let p3 = Polygon::new(vec![
        Vertex::new(377, 249), Vertex::new(411, 197), Vertex::new(436, 249),
    ],
        Color::RED,
        Color::WHITE,
    );
    p3.draw(&mut fb);

    let p4 = Polygon::new(vec![
        Vertex::new(413, 177), Vertex::new(448, 159), Vertex::new(502, 88), Vertex::new(553, 53),
        Vertex::new(535, 36), Vertex::new(676, 37), Vertex::new(660, 52), Vertex::new(750, 145),
        Vertex::new(761, 179), Vertex::new(672, 192), Vertex::new(659, 214), Vertex::new(615, 214),
        Vertex::new(632, 230), Vertex::new(580, 230), Vertex::new(597, 215), Vertex::new(552, 214),
        Vertex::new(517, 144), Vertex::new(466, 180),
    ],
        Color::GREEN,
        Color::WHITE,
    );
    p4.draw(&mut fb);

    let hole = Polygon::new(vec![
        Vertex::new(682, 175), Vertex::new(708, 120), Vertex::new(735, 148), Vertex::new(739, 170),
    ],
        bg_color,
        Color::WHITE,
    );
    hole.draw(&mut fb);

    fb.render_to_file("out.png");
    fb.render_to_bmp("out.bmp");
}
