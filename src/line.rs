use crate::framebuffer::FrameBuffer;
use crate::vertex::Vertex;

pub fn line(
    framebuffer: &mut FrameBuffer,
    start: &Vertex,
    end: &Vertex
) {
    let (mut x0, mut y0) = (start.x, start.y);
    let (mut x1, mut y1) = (end.x, end.y);

    if x0 > x1 {
        std::mem::swap(&mut x0, &mut x1);
        std::mem::swap(&mut y0, &mut y1);
    }

    let dx = x1 - x0;
    let mut dy = y1 - y0;

    let dir;
    if dy < 0 {
        dir = -1;
    } else {
        dir = 1;
    }
    dy = dy * dir;

    if dx != 0 {
        let mut y = y0;
        let mut p = 2*dy + dx;
        for x in 0..=dx {
            framebuffer.set_pixel(x0 + x, y);
            if p >= 0 {
                y = y + dir;
                p = p - 2*dx;
            }
            p = p + 2*dy;
        }
    }
}
