// Define
mod actors;
mod bsp;
mod doom;
mod map;
mod math;
mod shape;
mod render;
mod time;
mod wad;
mod window;
mod configure;
// Using engine
use doom::Doom;
use window::{doom_loop, doom_window};
use configure::Configure;
use shape::Size;
// Using
use winit::{
    dpi::LogicalSize,
    event_loop::EventLoop,
};

fn main() {
    let configure = Configure::load_from_file(String::from("assets/doom.ini")).unwrap();
    let event_loop = EventLoop::new().unwrap();
    let window = doom_window(
        String::from("Doom"),
        LogicalSize::<f64>::new(configure.screen.window.width(), configure.screen.window.height()),
        &event_loop,
    )
    .unwrap();
    let doom = Doom::new(&window, &configure);

    doom_loop(
        event_loop,
        window,
        doom,
        60,
        1.0 / 60.0,
        |dl| {
            dl.context.update();
        },
        |dl| {
            dl.context.draw();
        },
        |dl, event| {
            if !dl.context.control(&event) {
                dl.exit();
            }
        },
    ).unwrap();
}
