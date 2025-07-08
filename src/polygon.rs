use raylib::color::Color;

use crate::{framebuffer::FrameBuffer, line::line, vertex::Vertex};

pub struct Polygon {
    pub vertices: Vec<Vertex>,
    pub fill_color: Color,
    pub border_color: Color
}

impl Polygon {
    pub fn new(vertices: Vec<Vertex>, fill_color: Color, border_color: Color) -> Self {
        return Polygon {
            vertices,
            fill_color,
            border_color
        };
    }

    pub fn draw_polygon(&self, fb: &mut FrameBuffer) {
        fb.set_current_color(self.border_color);
        let n = self.vertices.len();
        for i in 0..n {
            let start_vertex = &self.vertices[i];
            let end_vertex = &self.vertices[(i + 1) % n];
            line(fb, start_vertex, end_vertex);
        }
    }
}
