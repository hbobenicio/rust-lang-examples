extern crate piston_window;

use piston_window::PistonWindow;
use piston_window::WindowSettings;
use piston_window::clear;
use piston_window::rectangle;

fn main() {
    let BLACK = [0.0, 0.0, 0.0, 1.0];
    let WHITE = [1.0; 4];

    let mut window: PistonWindow = WindowSettings::new("Snake Game", [640, 480])
        .exit_on_esc(true)
        .build()
        .expect("Unable to create window");

    while let Some(event) = window.next() {
        window.draw_2d(&event, |context, graphics| {
            clear(BLACK, graphics);
            rectangle(
                WHITE,
                [0.0, 0.0, 100.0, 100.0],
                context.transform,
                graphics
            );
        });
    }
}
