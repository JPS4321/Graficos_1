use crate::framebuffer::Framebuffer;
use nalgebra_glm::Vec3;

pub trait Line {
    fn line(&mut self, start: Vec3, end: Vec3);
    fn draw_polygon(&mut self, vertices: &[Vec3]);
}

impl Line for Framebuffer {
    fn line(&mut self, start: Vec3, end: Vec3) {
        let x1 = start.x.round() as isize;
        let y1 = start.y.round() as isize;
        let x2 = end.x.round() as isize;
        let y2 = end.y.round() as isize;

        let dx = (x2 - x1).abs();
        let sx = if x1 < x2 { 1 } else { -1 };
        let dy = -(y2 - y1).abs();
        let sy = if y1 < y2 { 1 } else { -1 };
        let mut err = dx + dy;

        let mut x = x1;
        let mut y = y1;

        loop {
            if x >= 0 && y >= 0 && (x as usize) < self.width && (y as usize) < self.height {
                self.point(x as usize, y as usize);
            }
            if x == x2 && y == y2 { break; }
            let e2 = 2 * err;
            if e2 >= dy {
                err += dy;
                x += sx;
            }
            if e2 <= dx {
                err += dx;
                y += sy;
            }
        }
    }

    fn draw_polygon(&mut self, vertices: &[Vec3]) {
        if vertices.len() < 2 {
            return;
        }

        for i in 0..vertices.len() {
            let start = vertices[i];
            let end = if i == vertices.len() - 1 {
                vertices[0]
            } else {
                vertices[i + 1]
            };
            self.line(start, end);
        }
    }
}
