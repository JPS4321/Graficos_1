use crate::bmp;

pub struct Framebuffer {
    pub width: usize,
    pub height: usize,
    pub buffer: Vec<u32>,
    pub background_color: u32,
    pub current_color: u32,
}

impl Framebuffer {
    pub fn new(width: usize, height: usize) -> Self {
        let buffer_size = width * height;
        let background_color = 0x000000; // Color de fondo predeterminado (negro)
        let buffer = vec![background_color; buffer_size];
        Self {
            width,
            height,
            buffer,
            background_color,
            current_color: 0xFFFFFF, // Color actual predeterminado (blanco)
        }
    }

    pub fn clear(&mut self) {
        for pixel in self.buffer.iter_mut() {
            *pixel = self.background_color;
        }
    }

    pub fn point(&mut self, x: usize, y: usize) {
        if x < self.width && y < self.height {
            let index = y * self.width + x;
            self.buffer[index] = self.current_color;
        }
    }

    #[allow(dead_code)]
    pub fn get_color(&self, x: usize, y: usize) -> Option<u32> {
        if x < self.width && y < self.height {
            let index = y * self.width + x;
            Some(self.buffer[index])
        } else {
            None
        }
    }

    pub fn set_background_color(&mut self, color: u32) {
        self.background_color = color;
    }

    pub fn set_current_color(&mut self, color: u32) {
        self.current_color = color;
    }

    pub fn render_buffer(&self, file_path: &str) -> std::io::Result<()> {
        bmp::write_bmp_file(file_path, &self.buffer, self.width, self.height)
    }
}
