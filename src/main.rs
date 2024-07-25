mod framebuffer;
mod line_impl;
mod bmp;

use framebuffer::Framebuffer;
use line_impl::Line;
use nalgebra_glm::Vec3;
use nalgebra_glm::vec3;

fn draw_polygon(framebuffer: &mut Framebuffer, vertices: &[Vec3]) {
    if vertices.len() < 3 {
        return; 
    }

  
    for i in 0..vertices.len() {
        let start = vertices[i];
        let end = if i == vertices.len() - 1 {
            vertices[0]
        } else {
            vertices[i + 1]
        };
        framebuffer.line(start, end);
    }
}

fn fill_polygon(framebuffer: &mut Framebuffer, vertices: &[Vec3]) {
    
    if vertices.len() < 3 {
        return; 
    }

    let min_y = vertices.iter().map(|v| v.y).fold(f32::INFINITY, f32::min).round() as isize;
    let max_y = vertices.iter().map(|v| v.y).fold(f32::NEG_INFINITY, f32::max).round() as isize;

    for y in min_y..=max_y {
        let mut intersections = vec![];

        for i in 0..vertices.len() {
            let start = vertices[i];
            let end = if i == vertices.len() - 1 {
                vertices[0]
            } else {
                vertices[i + 1]
            };

            if (start.y.round() as isize <= y && end.y.round() as isize > y) || (end.y.round() as isize <= y && start.y.round() as isize > y) {
                let x = start.x + (y as f32 - start.y) * (end.x - start.x) / (end.y - start.y);
                intersections.push(x.round() as isize);
            }
        }

        intersections.sort();

        for i in (0..intersections.len()).step_by(2) {
            if i + 1 < intersections.len() {
                for x in intersections[i]..=intersections[i + 1] {
                    framebuffer.point(x as usize, y as usize);
                }
            }
        }
    }
}

fn main() {
    let width = 800;
    let height = 600;
    let mut framebuffer = Framebuffer::new(width, height);

    framebuffer.set_background_color(0xFFFFFF);
    framebuffer.clear();

    framebuffer.set_current_color(0xFFFF00);

    let polygon = vec![
        vec3(165.0, 380.0, 0.0),
        vec3(185.0, 360.0, 0.0),
        vec3(180.0, 330.0, 0.0),
        vec3(207.0, 345.0, 0.0),
        vec3(233.0, 330.0, 0.0),
        vec3(230.0, 360.0, 0.0),
        vec3(250.0, 380.0, 0.0),
        vec3(220.0, 385.0, 0.0),
        vec3(205.0, 410.0, 0.0),
        vec3(193.0, 383.0, 0.0),
    ];

    fill_polygon(&mut framebuffer, &polygon);

    framebuffer.set_current_color(0xFFFFFF);

    draw_polygon(&mut framebuffer, &polygon);

    framebuffer.render_buffer("filled_polygon.bmp").expect("Failed to write BMP file");

    println!("Framebuffer rendered to filled_polygon.bmp");
}
