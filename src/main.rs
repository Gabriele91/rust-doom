// Define
mod map;
mod math;
mod time;
mod wad;
mod window;
mod render;
use math::Vec2;
// Using engine
use render::{
    Render2D::RenderMap, 
    Render
};
use window::{
    doom_loop,
    doom_window,
    DoomSurface
};
// Using
use winit::{
    dpi::{LogicalSize, PhysicalSize},
    event::VirtualKeyCode,
    event_loop::EventLoop,
};
use winit_input_helper::WinitInputHelper;
use std::rc::Rc;

struct Doom<'wad>{
    surface: DoomSurface,
    input: WinitInputHelper,
    map:  Box<map::Map<'wad>>,
    render: Box<RenderMap<'wad>>
}

fn main() {
    let doom1 = Rc::new(wad::Reader::new(String::from("assets/DOOM1.WAD")).unwrap());
    let map_e1m1 = Box::new(map::Map::new(&doom1, &String::from("E1M1")).unwrap());
    let render_map = Box::new(RenderMap::new(&map_e1m1, Vec2::new(280, 200), Vec2::new(20, 20)));

    let event_loop = EventLoop::new();
    let window = doom_window(
        String::from("Doom"),
        LogicalSize::<f64>::new(320.0 * 2., 240.0 * 2.),
        &event_loop,
    )
    .unwrap();
    let doom =  Box::new(Doom {
        surface: DoomSurface::new(PhysicalSize::<u32>::new(320, 240), &window).unwrap(),
        input: WinitInputHelper::new(),
        map: map_e1m1,
        render: render_map,
    });

    doom_loop(
        event_loop,
        window,
        doom,
        60,
        1.0 / 60.0,
        |_| {
            // Void
        },
        |dl| {
            dl.context.surface.clear([0, 0, 0, 0]);
            dl.context.render.draw(&mut dl.context.surface);
            dl.context.surface.swap().unwrap();
        },
        |dl, event| {
            // Input
            if dl.context.input.update(&event) {
                // Close events
                if dl.context.input.key_pressed(VirtualKeyCode::Escape) || dl.context.input.close_requested()
                {
                    dl.exit();
                }
            }
        },
    );
}
