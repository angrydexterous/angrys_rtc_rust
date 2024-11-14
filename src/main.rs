use canvas::canvas::Canvas;

mod math;
mod canvas;

fn main() {
    let mut canvas = Canvas::new(800, 600);

    // Variables to control the position of the animated shapes
    let mut rect_x = 100;
    let mut rect_y = 100;
    let mut circle_x = 400;
    let mut circle_y = 300;
    let rect_speed = 2;
    let circle_speed = 3;

    // Use the `run` method with an update callback for drawing and animating
    canvas.run(move |canvas| {
        // Clear the canvas with black
        for x in 0..canvas.width() {
            for y in 0..canvas.height() {
                canvas.set_pixel(x, y, 0, 0, 0);
            }
        }

        // Draw a red diagonal line (static)
        for i in 0..canvas.width().min(canvas.height()) {
            canvas.set_pixel(i, i, 255, 0, 0);
        }

        // Draw the animated green rectangle
        for x in rect_x..rect_x + 200 {
            for y in rect_y..rect_y + 100 {
                if x < canvas.width() && y < canvas.height() {
                    canvas.set_pixel(x, y, 0, 255, 0);
                }
            }
        }

        // Draw the animated blue circle
        let radius = 50;
        for x in (circle_x - radius)..(circle_x + radius) {
            for y in (circle_y - radius)..(circle_y + radius) {
                let dx = x as i32 - circle_x as i32;
                let dy = y as i32 - circle_y as i32;
                if dx * dx + dy * dy <= radius * radius {
                    if x >= 0 && x < canvas.width() as i32 && y >= 0 && y < canvas.height() as i32 {
                        canvas.set_pixel(x as usize, y as usize, 0, 0, 255);
                    }
                }
            }
        }

        // Update positions to animate the rectangle and circle
        rect_x = (rect_x + rect_speed) % canvas.width();
        rect_y = (rect_y + rect_speed) % canvas.height();

        circle_x = (circle_x + circle_speed) % canvas.width() as i32;
        circle_y = (circle_y + circle_speed) % canvas.height() as i32;
    });

    // Save the final canvas as a PPM file when the window is closed
    canvas.save_as_ppm("output.ppm").expect("Failed to save PPM file");

    println!("Image saved as output.ppm");
}
