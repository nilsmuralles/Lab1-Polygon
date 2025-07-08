use crate::framebuffer::FrameBuffer;
use crate::vertex::Vertex;

pub fn line(
    framebuffer: &mut FrameBuffer,
    start: &Vertex,
    end: &Vertex
) {
    let (x0, y0) = (start.x, start.y);
    let (x1, y1) = (end.x, end.y);

    let dx = x1 - x0;
    let dy = y1 - y0;
    if dx > 0 {
        let mut y = y0;
        let mut p = 2*dy + dx;
        for x in 0..=dx {
            framebuffer.set_pixel(x1 + x, y);
            if p >= 0 {
                y = y + 1;
                p = p - 2*dx;
            }
            p = p + 2*dy;
        }
    }
}
