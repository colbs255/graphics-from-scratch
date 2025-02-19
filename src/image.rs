use png::*;
use std::{fs::File, io::BufWriter};

// Define the Color struct with red, green, and blue values
#[derive(Debug, Clone, Copy)]
pub struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

impl Color {
    // Constructor for the Color struct
    pub fn new(red: u8, green: u8, blue: u8) -> Self {
        Color { red, green, blue }
    }

    pub const WHITE: Color = Color { red: 255, green: 255, blue: 255 };
    pub const BLACK: Color = Color { red: 0, green: 0, blue: 0 };
    pub const RED: Color = Color { red: 255, green: 0, blue: 0 };
    pub const GREEN: Color = Color { red: 0, green: 255, blue: 0 };

    // Convert Color to RGBA format
    fn to_rgba(&self) -> (u8, u8, u8, u8) {
        (self.red, self.green, self.blue, 255) // Full opacity (alpha = 255)
    }
}

// Define the Image struct which stores a 2D Vec of Color structs
#[derive(Debug)]
pub struct Image {
    pixels: Vec<Vec<Color>>, // 2D Vec of Color structs
}

impl Image {
    // Constructor for the Image struct
    pub fn new(width: usize, height: usize, default_color: Color) -> Self {
        // Create a 2D Vec filled with the default color
        let pixels = vec![vec![default_color; width]; height];
        Image { pixels }
    }

    pub fn save_to_png(&self, filename: &str) -> Result<(), Box<dyn std::error::Error>> {
        // Open the file where we will save the image
        let file = File::create(filename)?;
        let ref mut w = BufWriter::new(file);

        // Create the PNG encoder
        let mut encoder = Encoder::new(w, self.pixels[0].len() as u32, self.pixels.len() as u32);
        encoder.set_color(png::ColorType::Rgba);
        encoder.set_depth(png::BitDepth::Eight);

        // Write the image data to the file
        let mut writer = encoder.write_header()?;
        let rgba_bytes = self.to_rgba_bytes();
        writer.write_image_data(&rgba_bytes)?;

        Ok(())
    }

    // Get the color of a pixel at position (col, row)
    pub fn get_pixel(&self, col: usize, row: usize) -> Option<Color> {
        if row < self.pixels.len() && col < self.pixels[row].len() {
            Some(self.pixels[row][col].clone())
        } else {
            None
        }
    }

    // Set the color of a pixel at position (col, row)
    pub fn set_pixel(&mut self, col: usize, row: usize, color: Color) {
        if row < self.pixels.len() && col < self.pixels[row].len() {
            self.pixels[row][col] = color;
        } else {
            panic!("Invalid coordinates");
        }
    }

    fn to_rgba_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        for row in &self.pixels {
            for pixel in row {
                let (r, g, b, a) = pixel.to_rgba();
                bytes.push(r);
                bytes.push(g);
                bytes.push(b);
                bytes.push(a);
            }
        }
        bytes
    }

    pub fn width(&self) -> usize {
        self.pixels[0].len()
    }

    pub fn height(&self) -> usize {
        self.pixels.len()
    }
}
