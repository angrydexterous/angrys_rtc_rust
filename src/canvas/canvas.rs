use minifb::{Key, Window, WindowOptions};
use std::fs::File;
use std::io::{self, Write};
use std::time::Duration;

pub struct Canvas {
    width: usize,
    height: usize,
    buffer: Vec<u32>,
    window: Window,
}

impl Canvas {
    /// Creates a new Canvas with the specified width and height and initializes a window.
    pub fn new(width: usize, height: usize) -> Self {
        let buffer = vec![0; width * height];
        let window = Window::new(
            "Canvas Display",
            width,
            height,
            WindowOptions::default(),
        )
        .expect("Unable to open window");

        Canvas {
            width,
            height,
            buffer,
            window,
        }
    }

    /// Sets the color of a pixel at (x, y) with RGB values.
    pub fn set_pixel(&mut self, x: usize, y: usize, r: u8, g: u8, b: u8) {
        if x < self.width && y < self.height {
            self.buffer[y * self.width + x] = ((r as u32) << 16) | ((g as u32) << 8) | (b as u32);
        }
    }

    /// Gets the color of a pixel at (x, y) as an RGB tuple.
    pub fn get_pixel(&self, x: usize, y: usize) -> Option<(u8, u8, u8)> {
        if x < self.width && y < self.height {
            let color = self.buffer[y * self.width + x];
            Some((
                ((color >> 16) & 0xFF) as u8,
                ((color >> 8) & 0xFF) as u8,
                (color & 0xFF) as u8,
            ))
        } else {
            None
        }
    }

    /// Runs the window display loop, continuously updating the window.
    /// The `update_callback` allows custom drawing or animation logic to be applied each frame.
    pub fn run<F>(&mut self, mut update_callback: F)
    where
        F: FnMut(&mut Self),
    {
        while self.window.is_open() && !self.window.is_key_down(Key::Escape) {
            // Call the user-defined update callback to modify the canvas
            update_callback(self);

            // Update the window with the buffer
            self.window
                .update_with_buffer(&self.buffer, self.width, self.height)
                .unwrap();

            // Control frame rate
            std::thread::sleep(Duration::from_millis(16));
        }
    }

    /// Saves the canvas as a PPM file.
    pub fn save_as_ppm(&self, filename: &str) -> io::Result<()> {
        let mut file = File::create(filename)?;

        // Write the PPM header
        writeln!(file, "P3")?;
        writeln!(file, "{} {}", self.width, self.height)?;
        writeln!(file, "255")?;

        // Write pixel data
        for y in 0..self.height {
            for x in 0..self.width {
                if let Some((r, g, b)) = self.get_pixel(x, y) {
                    writeln!(file, "{} {} {}", r, g, b)?;
                }
            }
        }
        Ok(())
    }
    
    pub fn width(&self) -> usize {
        self.width
    }
    
    pub fn height(&self) -> usize {
        self.height
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_set_get_pixel() {
        let mut canvas = Canvas::new(10, 10);
        canvas.set_pixel(5, 5, 255, 0, 0); // Set pixel to red

        assert_eq!(canvas.get_pixel(5, 5), Some((255, 0, 0)));
        assert_eq!(canvas.get_pixel(0, 0), Some((0, 0, 0))); // Default color is black
    }

    #[test]
    fn test_save_as_ppm() {
        let mut canvas = Canvas::new(3, 3);
        canvas.set_pixel(0, 0, 255, 0, 0); // Red
        canvas.set_pixel(1, 0, 0, 255, 0); // Green
        canvas.set_pixel(2, 0, 0, 0,  255); // Blue

        // Save canvas to a PPM file
        canvas.save_as_ppm("test_output.ppm").unwrap();

        // Read the file and check contents
        let contents = std::fs::read_to_string("test_output.ppm").unwrap();
        let expected_output = "P3\n3 3\n255\n255 0 0\n0 255 0\n0 0 255\n0 0 0\n0 0 0\n0 0 0\n0 0 0\n0 0 0\n0 0 0\n";

        assert_eq!(contents, expected_output);
    }
}