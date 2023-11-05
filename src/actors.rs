
// Engine
use crate::math::Vec2;
use crate::map::Thing;
use crate::doom::Doom;

// Utils
use std::boxed::Box;
use winit_input_helper::WinitInputHelper;

pub trait Actor {
    fn update<'wad>(&self, engine: &Doom<'wad>);
    fn control(&self, input: &WinitInputHelper);
    fn type_id(&self) -> u16;
    fn position(&self) -> &Vec2<i16>;
    fn angle(&self) -> u16;
    fn flags(&self) -> u16;
}

pub struct Player {
    pub type_id: u16,
    pub position: Vec2<i16>,
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
    fn update<'wad>(&self, engine: &Doom<'wad>) {

    }

    fn control(&self, input: &WinitInputHelper) {

    }

    fn type_id(&self) -> u16 {
        self.type_id
    }
    
    fn position(&self) -> &Vec2<i16>{
        &self.position
    }
    
    fn angle(&self) -> u16 {
        self.angle
    }

    fn flags(&self) -> u16 {
        self.flags
    }
}