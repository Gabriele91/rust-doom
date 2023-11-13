// Define
mod actors;
mod bsp;
mod configure;
mod doom;
mod map;
mod math;
mod render;
mod shape;
mod time;
mod wad;
mod window;
// Using engine
use configure::Configure;
use doom::Doom;
use window::{doom_loop, doom_window};
// Using
use winit::event_loop::EventLoop;

fn main() {
    let configure = Configure::load_from_file(String::from("assets/doom.ini")).unwrap();
    let event_loop = EventLoop::new().unwrap();
    let window = doom_window(
        &configure.screen.title,
        &configure.screen.window,
        &event_loop,
    )
    .unwrap();
    let doom = Doom::new(&window, &configure);

    make_doom_loop!(event_loop, window, doom, configure.screen.frame_rate).unwrap();
}
