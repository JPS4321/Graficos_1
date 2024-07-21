use std::fs::File;
use std::io::{Write, BufWriter};

const BMP_HEADER_SIZE: usize = 54;
const BMP_PIXEL_OFFSET: usize = 54;
const BMP_BITS_PER_PIXEL: usize = 24; // 24 bits por pixel para un BMP de 24 bits

pub fn write_bmp_file(
    file_path: &str, // Path to the output BMP file
    buffer: &[u32], // Framebuffer pixel data
    width: usize,   // Width of the image
    height: usize,  // Height of the image
) -> std::io::Result<()> {
    let file = File::create(file_path)?;
    let mut writer = BufWriter::new(file);

    write_bmp_header(&mut writer, width, height)?;
    write_pixel_data(&mut writer, buffer, width, height)?;

    writer.flush()?;
    Ok(())
}

fn write_bmp_header(
    writer: &mut BufWriter<File>,
    width: usize,
    height: usize,
) -> std::io::Result<()> {
    let file_size = BMP_HEADER_SIZE + (width * height * BMP_BITS_PER_PIXEL / 8);
    let mut header = vec![0u8; BMP_HEADER_SIZE];

    // BMP signature
    header[0] = b'B';
    header[1] = b'M';

    // File size
    header[2] = (file_size & 0xFF) as u8;
    header[3] = ((file_size >> 8) & 0xFF) as u8;
    header[4] = ((file_size >> 16) & 0xFF) as u8;
    header[5] = ((file_size >> 24) & 0xFF) as u8;

    // Reserved
    header[6] = 0;
    header[7] = 0;
    header[8] = 0;
    header[9] = 0;

    // Pixel data offset
    header[10] = BMP_PIXEL_OFFSET as u8;
    header[11] = 0;
    header[12] = 0;
    header[13] = 0;

    // DIB header size
    header[14] = 40;
    header[15] = 0;
    header[16] = 0;
    header[17] = 0;

    // Image width
    header[18] = (width & 0xFF) as u8;
    header[19] = ((width >> 8) & 0xFF) as u8;
    header[20] = ((width >> 16) & 0xFF) as u8;
    header[21] = ((width >> 24) & 0xFF) as u8;

    // Image height
    header[22] = (height & 0xFF) as u8;
    header[23] = ((height >> 8) & 0xFF) as u8;
    header[24] = ((height >> 16) & 0xFF) as u8;
    header[25] = ((height >> 24) & 0xFF) as u8;

    // Color planes
    header[26] = 1;
    header[27] = 0;

    // Bits per pixel
    header[28] = BMP_BITS_PER_PIXEL as u8;
    header[29] = 0;

    // Compression (no compression)
    header[30] = 0;
    header[31] = 0;
    header[32] = 0;
    header[33] = 0;

    // Image size (no compression)
    header[34] = 0;
    header[35] = 0;
    header[36] = 0;
    header[37] = 0;

    // Resolution
    header[38] = 0x13;
    header[39] = 0x0B;
    header[40] = 0;
    header[41] = 0;
    header[42] = 0x13;
    header[43] = 0x0B;
    header[44] = 0;
    header[45] = 0;

    // Colors
    header[46] = 0;
    header[47] = 0;
    header[48] = 0;
    header[49] = 0;
    header[50] = 0;
    header[51] = 0;
    header[52] = 0;
    header[53] = 0;

    writer.write_all(&header)?;
    Ok(())
}

fn write_pixel_data(
    writer: &mut BufWriter<File>,
    buffer: &[u32],
    width: usize,
    height: usize,
) -> std::io::Result<()> {
    let padding_size = (4 - (width * 3 % 4)) % 4;
    for y in (0..height).rev() {
        for x in 0..width {
            let color = buffer[y * width + x];
            let blue = (color & 0xFF) as u8;
            let green = ((color >> 8) & 0xFF) as u8;
            let red = ((color >> 16) & 0xFF) as u8;

            writer.write_all(&[blue, green, red])?;
        }
        writer.write_all(&vec![0; padding_size])?;
    }
    Ok(())
}
