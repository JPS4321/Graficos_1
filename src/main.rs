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

    
    let polygon1 = vec![
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

    
    let polygon2 = vec![
        vec3(321.0, 335.0, 0.0),
        vec3(288.0, 286.0, 0.0),
        vec3(339.0, 251.0, 0.0),
        vec3(374.0, 302.0, 0.0),
    ];

    
    let polygon3 = vec![
        vec3(377.0, 249.0, 0.0),
        vec3(411.0, 197.0, 0.0),
        vec3(436.0, 249.0, 0.0),
    ];

    
    let polygon4 = vec![
        vec3(413.0, 177.0, 0.0),
        vec3(448.0, 159.0, 0.0),
        vec3(502.0, 88.0, 0.0),
        vec3(553.0, 53.0, 0.0),
        vec3(535.0, 36.0, 0.0),
        vec3(676.0, 37.0, 0.0),
        vec3(660.0, 52.0, 0.0),
        vec3(750.0, 145.0, 0.0),
        vec3(761.0, 179.0, 0.0),
        vec3(672.0, 192.0, 0.0),
        vec3(659.0, 214.0, 0.0),
        vec3(615.0, 214.0, 0.0),
        vec3(632.0, 230.0, 0.0),
        vec3(580.0, 230.0, 0.0),
        vec3(597.0, 215.0, 0.0),
        vec3(552.0, 214.0, 0.0),
        vec3(517.0, 144.0, 0.0),
        vec3(466.0, 180.0, 0.0),
    ];

    
    let polygon5 = vec![
        vec3(682.0, 175.0, 0.0),
        vec3(708.0, 120.0, 0.0),
        vec3(735.0, 148.0, 0.0),
        vec3(739.0, 170.0, 0.0),
    ];

    
    framebuffer.set_current_color(0xFFFF00); 
    fill_polygon(&mut framebuffer, &polygon1);

    
    framebuffer.set_current_color(0xFFFFFF); 
    draw_polygon(&mut framebuffer, &polygon1);

    
    framebuffer.set_current_color(0x0000FF); 
    fill_polygon(&mut framebuffer, &polygon2);

    
    framebuffer.set_current_color(0xFFFFFF); 
    draw_polygon(&mut framebuffer, &polygon2);

    
    framebuffer.set_current_color(0xFF0000); 
    fill_polygon(&mut framebuffer, &polygon3);

    
    framebuffer.set_current_color(0xFFFFFF); 
    draw_polygon(&mut framebuffer, &polygon3);

    
    framebuffer.set_current_color(0x00FF00); 
    fill_polygon(&mut framebuffer, &polygon4);

    
    framebuffer.set_current_color(0xFFFFFF); 
    draw_polygon(&mut framebuffer, &polygon4);

    
    framebuffer.set_current_color(0xFFFFFF); 
    fill_polygon(&mut framebuffer, &polygon5);

    framebuffer.set_current_color(0xFFFFFF); 
    draw_polygon(&mut framebuffer, &polygon5);

    
    framebuffer.render_buffer("polygons.bmp").expect("Failed to write BMP file");

    println!("Framebuffer rendered to polygons_with_hole.bmp");
}
