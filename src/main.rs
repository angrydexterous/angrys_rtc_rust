
use canvas::canvas::Canvas;

mod math;
mod canvas;

fn main() {
    let mut canvas = Canvas::new(800, 600);

    // Use the `run` method with a custom update callback for drawing
    canvas.run(|canvas| {
        // Draw a red diagonal line
        for i in 0..canvas.width().min(canvas.height()) {
            canvas.set_pixel(i, i, 255, 0, 0);
        }

        // Draw a green rectangle
        for x in 100..300 {
            for y in 100..200 {
                canvas.set_pixel(x, y, 0, 255, 0);
            }
        }

        // Draw a blue circle
        let center_x = 400;
        let center_y = 300;
        let radius = 100;
        for x in (center_x - radius)..(center_x + radius) {
            for y in (center_y - radius)..(center_y + radius) {
                let dx = x as i32 - center_x as i32;
                let dy = y as i32 - center_y as i32;
                if dx * dx + dy * dy <= radius * radius {
                    canvas.set_pixel(x as usize, y as usize, 0, 0, 255);
                }
            }
        }
    });

    // Save the final canvas as a PPM file
    canvas.save_as_ppm("output.ppm").expect("Failed to save PPM file");

    println!("Image saved as output.ppm");
}