
// Engine
use crate::math::Vector2;
use crate::map::Thing;
use crate::doom::Doom;

// Utils
use std::boxed::Box;
use winit::event::VirtualKeyCode;
use winit_input_helper::WinitInputHelper;

pub trait Actor {
    fn update<'wad>(&mut self, engine: &Doom<'wad>);
    fn control(&mut self, input: &WinitInputHelper);
    fn type_id(&self) -> u16;
    fn position(&self) -> &Vector2<i16>;
    fn angle(&self) -> u16;
    fn flags(&self) -> u16;
}

pub struct Player {
    pub type_id: u16,
    pub position: Vector2<i16>,
    pub angle: u16,
    pub flags: u16
}

impl Player {
    pub fn new<'wad>(thing: &'wad Thing) -> Box<dyn Actor> {
        Box::new(Player {
            type_id: thing.type_id,
            position: thing.position,
            angle: thing.angle,
            flags: thing.flags,
        })
    }
}

#[allow(unused_variables)]
impl Actor for Player {
    fn update<'wad>(&mut self, engine: &Doom<'wad>) {

    }

    fn control(&mut self, input: &WinitInputHelper) {
        if  input.key_held(VirtualKeyCode::W) 
        && !input.key_held(VirtualKeyCode::S) {
            self.position += Vector2::new(0, 1);
        }
        if !input.key_held(VirtualKeyCode::W) 
        &&  input.key_held(VirtualKeyCode::S) {
            self.position -= Vector2::new(0, 1);
        }        
        if  input.key_held(VirtualKeyCode::A) 
        && !input.key_held(VirtualKeyCode::D) {
            self.position -= Vector2::new(1, 0);
        }
        if !input.key_held(VirtualKeyCode::A) 
        &&  input.key_held(VirtualKeyCode::D) {
            self.position += Vector2::new(1, 0);
        }
    }

    fn type_id(&self) -> u16 {
        self.type_id
    }
    
    fn position(&self) -> &Vector2<i16>{
        &self.position
    }
    
    fn angle(&self) -> u16 {
        self.angle
    }

    fn flags(&self) -> u16 {
        self.flags
    }
}