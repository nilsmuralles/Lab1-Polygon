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
        let m = dy as f32 / dx as f32;
        for x in 0..=dx {
            framebuffer.set_pixel(x0 + x, (y0 as f32 + x as f32 * m) as i32);
        }
    }
}
