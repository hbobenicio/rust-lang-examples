extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;

mod app;

use piston::input::{RenderEvent, UpdateEvent};

fn main() {
    // Change this to OpenGL::V2_1 if not working.
    let opengl = opengl_graphics::OpenGL::V3_2;

    // Create an Glutin window.
    let mut window: glutin_window::GlutinWindow = piston::window::WindowSettings::new(
            "spinning-square",
            [200, 200]
        )
        .opengl(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    // Create a new game and run it.
    let mut app = app::App::new(opengl);

    let mut events = piston::event_loop::Events::new(piston::event_loop::EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(r) = e.render_args() {
            app.render(&r);
        }

        if let Some(u) = e.update_args() {
            app.update(&u);
        }
    }
}
