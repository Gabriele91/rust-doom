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
use winit::{event::Event, event::VirtualKeyCode};
use winit_input_helper::WinitInputHelper;

pub struct Doom<'wad> {
    pub input: WinitInputHelper,
    pub surface: DoomSurface,
    pub map: Box<Map<'wad>>,
    pub bsp: BSP<'wad>,
    pub actors: Vec<Rc<RefCell<Box<dyn Actor>>>>,
    pub render_map: Rc<RefCell<RenderMap<'wad>>>,
    pub render_bsp: Rc<RefCell<RenderBSP<'wad>>>,
}

impl<'wad> Doom<'wad> {
    pub fn new(surface: DoomSurface, map: Box<Map<'wad>>) -> Box<Self> {
        Box::new(Doom {
            input: WinitInputHelper::new(),
            surface: surface,
            map: map.clone(),
            bsp: BSP::new(&map),
            actors: Doom::create_actors(&map),
            render_map: Rc::new(RefCell::new(RenderMap::new(
                &map,
                Vector2::new(280, 200),
                Vector2::new(20, 20),
            ))),
            render_bsp: Rc::new(RefCell::new(RenderBSP::new(
                &map,
                Vector2::new(280, 200),
                Vector2::new(20, 20),
            ))),
        })
    }

    pub fn update(&mut self) {
        for actor in &self.actors {
            actor.borrow_mut().update(self);
        }
    }

    pub fn draw(&mut self) {
        self.surface.clear([0, 0, 0, 0]);
        let render_map = self.render_map.clone();
        let render_bsp = self.render_bsp.clone();
        render_map.borrow_mut().draw(self);
        render_bsp.borrow_mut().draw(self);
        self.surface.swap().unwrap();
    }

    pub fn control(&mut self, event: &Event<'_, ()>) -> bool {
        // Input
        if self.input.update(&event) {
            // Close events
            if self.input.key_pressed(VirtualKeyCode::Escape) || self.input.close_requested() {
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
