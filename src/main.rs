mod framebuffer;
mod line_impl;
mod bmp;

use framebuffer::Framebuffer;
use line_impl::Line;
use nalgebra_glm::Vec3;
use nalgebra_glm::vec3;

fn draw_polygon(framebuffer: &mut Framebuffer, vertices: &[Vec3]) {
    // Check if the number of vertices is at least 3 (minimum required for a polygon)
    if vertices.len() < 3 {
        return; // Early return or handle error appropriately
    }

    // Iterate over the vertices in the array
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
    // Check if the number of vertices is at least 3 (minimum required for a polygon)
    if vertices.len() < 3 {
        return; // Early return or handle error appropriately
    }

    // Find the min and max y-coordinates
    let min_y = vertices.iter().map(|v| v.y).fold(f32::INFINITY, f32::min).round() as isize;
    let max_y = vertices.iter().map(|v| v.y).fold(f32::NEG_INFINITY, f32::max).round() as isize;

    // Iterate through each scan line
    for y in min_y..=max_y {
        let mut intersections = vec![];

        // Find all intersections of the polygon with the scan line
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

        // Sort the intersections
        intersections.sort();

        // Fill between pairs of intersections
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

    // Clear the framebuffer with a white background
    framebuffer.set_background_color(0xFFFFFF);
    framebuffer.clear();

    // Define the vertices of the first polygon
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

    // Define the vertices of the second polygon
    let polygon2 = vec![
        vec3(321.0, 335.0, 0.0),
        vec3(288.0, 286.0, 0.0),
        vec3(339.0, 251.0, 0.0),
        vec3(374.0, 302.0, 0.0),
    ];

    // Define the vertices of the third polygon
    let polygon3 = vec![
        vec3(377.0, 249.0, 0.0),
        vec3(411.0, 197.0, 0.0),
        vec3(436.0, 249.0, 0.0),
    ];

    // Fill the first polygon with yellow color
    framebuffer.set_current_color(0xFFFF00); // Yellow
    fill_polygon(&mut framebuffer, &polygon1);

    // Draw the first polygon outline in white
    framebuffer.set_current_color(0xFFFFFF); // White
    draw_polygon(&mut framebuffer, &polygon1);

    // Fill the second polygon with blue color
    framebuffer.set_current_color(0x0000FF); // Blue
    fill_polygon(&mut framebuffer, &polygon2);

    // Draw the second polygon outline in white
    framebuffer.set_current_color(0xFFFFFF); // White
    draw_polygon(&mut framebuffer, &polygon2);

    // Fill the third polygon with red color
    framebuffer.set_current_color(0xFF0000); // Red
    fill_polygon(&mut framebuffer, &polygon3);

    // Draw the third polygon outline in white
    framebuffer.set_current_color(0xFFFFFF); // White
    draw_polygon(&mut framebuffer, &polygon3);

    // Save the framebuffer as a BMP file
    framebuffer.render_buffer("polygons.bmp").expect("Failed to write BMP file");

    println!("Framebuffer rendered to polygons.bmp");
}
