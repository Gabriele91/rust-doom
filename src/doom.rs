// Using engine
use crate::{map::Map, actors::Player};
use crate::render::{
    render_2d::RenderMap,
    Render
};
use crate::window::DoomSurface;
use crate::actors::Actor;

// Utils
use std::boxed::Box;
use std::vec::Vec;
use winit::{event::Event, event::VirtualKeyCode};
use winit_input_helper::WinitInputHelper;

pub struct Doom<'wad> {
    pub input: WinitInputHelper,
    pub surface: DoomSurface,
    pub map: Box<Map<'wad>>,
    pub render: Box<RenderMap<'wad>>,
    pub actors: Vec<Box<dyn Actor>>,
}

impl<'wad> Doom<'wad> {
    pub fn new(
        surface: DoomSurface,
        map: Box<Map<'wad>>,
        render: Box<RenderMap<'wad>>,
    ) -> Box<Self> {
        Box::new(Doom {
            input: WinitInputHelper::new(),
            surface: surface,
            actors: Doom::create_actors(&map),
            map: map,
            render: render,
        })
    }

    pub fn update(&self) {
        for actor in &self.actors {
            actor.update(&self);
        }
    }

    pub fn draw(&mut self) {
        self.surface.clear([0, 0, 0, 0]);
        self.render.draw(&mut self.surface, &self.actors);
        self.surface.swap().unwrap();
    }

    pub fn control(&mut self, event: &Event<'_, ()>) -> bool {
        // Input
        if self.input.update(&event) {
            // Close events
            if self.input.key_pressed(VirtualKeyCode::Escape) || self.input.close_requested() {
                return false;
            } else {
                for actor in &self.actors {
                    actor.control(&self.input);
                }
            }
        }
        return true;
    }

    fn create_actors(map: &Box<Map<'wad>>) -> Vec<Box<dyn Actor>> {
        let mut actors = vec![];
        for thing in &map.things {
            match thing.thing_type {
                1 => actors.push(Player::new(&thing)), 
                _ => ()
            }
        }
        return actors;
    }
    
}
