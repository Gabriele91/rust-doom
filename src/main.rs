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
mod camera;
mod data_textures;
mod intersection;
// Using engine
use configure::Configure;
use doom::Doom;
use window::{doom_loop, doom_window};
// Using
use winit::event_loop::EventLoop;
use std::env;
use std::process::ExitCode;

fn cli_handler() -> Result<String, String> {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        1 => {
            Err(format!(
                "Error: No arguments provided.\nUsage: {} <config-file>",
                args[0]
            ))
        }
        2 => {
            let config_file = &args[1];
            Ok(config_file.clone())
        }
        _ => {
            Err(format!(
                "Error: Too many arguments.\nUsage: {} <config-file>",
                args[0]
            ))
        }
    }
}

fn main() -> ExitCode {
    let config_file = {
        match cli_handler() {
            Ok(config_file) => config_file,
            Err(err) => { 
                eprintln!("{}", err); 
                return ExitCode::FAILURE;
            }
        }
    };
    let configure = Configure::load_from_file(config_file).unwrap();
    let event_loop = EventLoop::new().unwrap();
    let window = doom_window(
        &configure.screen.title,
        &configure.screen.window,
        &event_loop,
    )
    .unwrap();
    let doom = Doom::new(&window, &configure);

    make_doom_loop!(event_loop, window, doom, configure.screen.frame_rate).unwrap();
    return ExitCode::SUCCESS;
}
