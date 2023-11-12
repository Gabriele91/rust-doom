// Using engine
use crate::actors::Actor;
use crate::bsp::BSP;
use crate::configure::Configure;
use crate::math::Vector2;
use crate::render;
use crate::render::{
    render_2d::{RenderBSP, RenderMap},
    Render,
};
use crate::shape::Size;
use crate::wad::Reader;
use crate::window::DoomSurface;
use crate::{actors::Player, map::Map};
// Utils
use std::boxed::Box;
use std::cell::RefCell;
use std::rc::Rc;
use std::vec::Vec;
use winit::dpi::PhysicalSize;
use winit::window::Window;
use winit::{event::Event, keyboard::KeyCode};
use winit_input_helper::WinitInputHelper;

pub struct Doom<'wad> {
    pub wad: Rc<Reader>,

    pub input: WinitInputHelper,
    pub map: Box<Map<'wad>>,
    pub bsp: BSP<'wad>,
    pub actors: Vec<Rc<RefCell<Box<dyn Actor>>>>,
    
    pub surface: DoomSurface,
    pub renders: Vec<Rc<RefCell<Box<dyn Render + 'wad>>>>,
}

macro_rules! crea_render {
    ($t:expr) => {
        Rc::new(RefCell::new(Box::new($t) as Box<dyn Render + 'wad>))
    };
}

impl<'wad> Doom<'wad> {
    pub fn new(window: &Window, configure: &Configure) -> Box<Self> {
        let wad = Rc::new(Reader::new(&configure.resource.wad).unwrap());
        let map = Box::new(Map::new(&wad, &String::from("E1M1")).unwrap());
        let surface = DoomSurface::new(PhysicalSize::<u32>::new(configure.screen.surface.width(),  configure.screen.surface.height()), &window).unwrap();    
        Box::new(Doom {
            // Resource
            wad,
            // Logic
            input: WinitInputHelper::new(),
            map: map.clone(),
            bsp: BSP::new(&map),
            actors: Doom::create_actors(&map),
            // Render
            surface,
            renders: {
                let mut renders = vec![];
                if let Some(debug) = &configure.debug {
                    if let Some(render_map_2d) = &debug.render_map_2d {
                        renders.push(crea_render!(RenderMap::new(&map, render_map_2d.zw(), render_map_2d.xy())));
                    }
                    if let Some(render_bsp_2d) = &debug.render_bsp_2d {
                        renders.push(crea_render!(RenderBSP::new(&map, render_bsp_2d.zw(), render_bsp_2d.xy())));
                    }
                }
                renders
            },
        })

    }

    pub fn update(&mut self) {
        for actor in &self.actors {
            actor.borrow_mut().update(self);
        }
    }

    pub fn draw(&mut self) {
        self.surface.clear([0, 0, 0, 0]);
        for render_id in 0..self.renders.len() {
            let render = self.renders[render_id].clone();
            render.borrow_mut().draw(self);
        }
        self.surface.swap().unwrap();
    }

    pub fn control(&mut self, event: &Event<()>) -> bool {
        // Input
        if self.input.update(&event) {
            // Close events
            if self.input.key_pressed(KeyCode::Escape) || self.input.close_requested() {
                return false;
            } else {
                for actor in &mut self.actors {
                    actor.borrow_mut().control(&self.input);
                }
            }
        }
        return true;
    }

    fn create_actors(map: &Box<Map<'wad>>) -> Vec<Rc<RefCell<Box<dyn Actor>>>> {
        let mut actors = vec![];
        for thing in &map.things {
            match thing.type_id {
                1 => actors.push(Rc::new(RefCell::new(Player::new(&thing)))),
                _ => (),
            }
        }
        return actors;
    }
}
