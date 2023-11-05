// Define
mod actors;
mod bsp;
mod doom;
mod map;
mod math;
mod render;
mod time;
mod wad;
mod window;
mod configure;
// Using engine
use doom::Doom;
use math::Vector2;
use render::render_2d::RenderMap;
use window::{doom_loop, doom_window, DoomSurface};
use configure::Configure;
// Using
use std::rc::Rc;
use winit::{
    dpi::{LogicalSize, PhysicalSize},
    event_loop::EventLoop,
};

fn main() {
    let configure = Configure::load_from_file(String::from("assets/doom.ini")).unwrap();
    let doom1 = Rc::new(wad::Reader::new(&configure.resource.wad).unwrap());
    let map_e1m1 = Box::new(map::Map::new(&doom1, &String::from("E1M1")).unwrap());
    let render_map = Box::new(RenderMap::new(
        &map_e1m1,
        Vector2::new(280, 200),
        Vector2::new(20, 20),
    ));
    let event_loop = EventLoop::new();
    let window = doom_window(
        String::from("Doom"),
        LogicalSize::<f64>::new(configure.screen.window.width, configure.screen.window.height),
        &event_loop,
    )
    .unwrap();
    let surface = DoomSurface::new(PhysicalSize::<u32>::new(configure.screen.surface.width,  configure.screen.surface.height), &window).unwrap();
    let doom = Doom::new(surface, map_e1m1, render_map);

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
    );
}
