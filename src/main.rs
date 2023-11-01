// Define
mod map;
mod math;
mod time;
mod wad;
mod window;
use math::Vec2;
// Using engine
use window::{doom_loop, doom_window, DoomSurface};
// Using
use winit::{
    dpi::{LogicalSize, PhysicalSize},
    event::VirtualKeyCode,
    event_loop::EventLoop,
};
use winit_input_helper::WinitInputHelper;

fn main() {
    if let Some(doom1) = wad::Reader::new(String::from("assets/DOOM1.WAD")) {
        if let Some(map) = map::Map::new(&doom1, &String::from("E1M1")) {
            println!("{:?}", map);
        }
    }

    let event_loop = EventLoop::new();
    let window = doom_window(
        String::from("Doom"),
        LogicalSize::<f64>::new(320.0 * 4., 240.0 * 4.),
        &event_loop,
    )
    .unwrap();
    let context = (
        DoomSurface::new(PhysicalSize::<u32>::new(320, 240), &window).unwrap(),
        WinitInputHelper::new(),
    );

    doom_loop(
        event_loop,
        window,
        context,
        60,
        0.0166666666666667,
        |_| {
            // Void
        },
        |dl| {
            dl.context.0.clear([0, 0, 0, 0]);
            for g in 0..360 {
                let x = math::SIN[g] * 50.0;
                let y = math::COS[g] * 50.0;
                let xoffset = 160.0 + math::SIN[(dl.running_time * 500.0) as usize % 360] * 25.0;
                let yoffset = 120.0 + math::COS[(dl.running_time * 100.0) as usize % 360] * 25.0;
                dl.context.0.draw(
                    &Vec2::new((x + xoffset) as usize, (y + yoffset) as usize),
                    &[0xFF, 0, 0, 0xFF],
                );
            }
            dl.context.0.swap().unwrap();
        },
        |dl, event| {
            // Input
            if dl.context.1.update(&event) {
                // Close events
                if dl.context.1.key_pressed(VirtualKeyCode::Escape)
                    || dl.context.1.close_requested()
                {
                    dl.exit();
                }
            }
        },
    );
}
