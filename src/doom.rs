// Using engine
use crate::actors::Actor;
use crate::bsp::BSP;
use crate::math::Vector2;
use crate::render::{
    render_2d::{RenderBSP, RenderMap},
    Render,
};
use crate::window::DoomSurface;
use crate::{actors::Player, map::Map};
// Utils
use std::boxed::Box;
use std::cell::RefCell;
use std::rc::Rc;
use std::vec::Vec;
use winit::{event::Event, keyboard::KeyCode};
use winit_input_helper::WinitInputHelper;

pub struct Doom<'wad> {
    pub input: WinitInputHelper,
    pub surface: DoomSurface,
    pub map: Box<Map<'wad>>,
    pub bsp: BSP<'wad>,
    pub actors: Vec<Rc<RefCell<Box<dyn Actor>>>>,
    pub renders: Vec<Rc<RefCell<Box<dyn Render + 'wad>>>>,
}

macro_rules! crea_render {
    ($t:expr) => {
        Rc::new(RefCell::new(Box::new($t) as Box<dyn Render + 'wad>))
    };
}

impl<'wad> Doom<'wad> {
    pub fn new(surface: DoomSurface, map: Box<Map<'wad>>) -> Box<Self> {
        Box::new(Doom {
            input: WinitInputHelper::new(),
            surface: surface,
            map: map.clone(),
            bsp: BSP::new(&map),
            actors: Doom::create_actors(&map),
            renders: vec![
                crea_render!(RenderMap::new(
                    &map,
                    Vector2::new(280, 200),
                    Vector2::new(20, 20),
                )),
                crea_render!(RenderBSP::<'wad>::new(
                    &map,
                    Vector2::new(280, 200),
                    Vector2::new(20, 20),
                ))
            ]
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
