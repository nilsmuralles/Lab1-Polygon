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

    pub fn draw(&self, fb: &mut FrameBuffer) {
        self.fill(fb);

        fb.set_current_color(self.border_color);
        let n = self.vertices.len();
        for i in 0..n {
            let start_vertex = &self.vertices[i];
            let end_vertex = &self.vertices[(i + 1) % n];
            line(fb, start_vertex, end_vertex);
        }
    }

    pub fn fill(&self, fb: &mut FrameBuffer) {
        fb.set_current_color(self.fill_color);

        let n = self.vertices.len();
        if n < 3 {
            return; 
        }

        let mut min_y = self.vertices[0].y;
        let mut max_y = self.vertices[0].y;

        for v in &self.vertices {
            if v.y < min_y { 
                min_y = v.y; 
            }
            if v.y > max_y { 
                max_y = v.y; 
            }
        }

        for y in min_y..=max_y {
            let mut intersections = Vec::new();

            for i in 0..n {
                let v1 = &self.vertices[i];
                let v2 = &self.vertices[(i + 1) % n];

                if (v1.y <= y && v2.y > y) || (v2.y <= y && v1.y > y) {
                    let dy = v2.y - v1.y;
                    let dx = v2.x - v1.x;

                    let x = v1.x as f32 + ((y - v1.y) as f32) * (dx as f32) / (dy as f32);
                    intersections.push(x);
                }
            }

            intersections.sort_by(|a, b| a.partial_cmp(b).unwrap());

            let mut i = 0;
            while i + 1 < intersections.len() {
                let x_start = intersections[i].ceil() as i32;
                let x_end = intersections[i + 1].floor() as i32;

                for x in x_start..=x_end {
                    fb.set_pixel(x, y);
                }

                i += 2;
            }
        }
    }
}
