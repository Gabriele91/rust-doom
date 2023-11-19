
// Engine
use crate::math::{Vector2, normalize_degrees};
use crate::map::Thing;
use crate::doom::Doom;

// Utils
use std::boxed::Box;
use winit::keyboard::KeyCode;
use winit_input_helper::WinitInputHelper;

pub trait Actor {
    fn update<'wad>(&mut self, engine: &Doom<'wad>);
    fn control(&mut self, input: &WinitInputHelper);
    fn type_id(&self) -> u16;
    fn position(&self) -> &Vector2<i16>;
    fn height(&self) -> &i16;
    fn angle(&self) -> u16;
    fn flags(&self) -> u16;
}

pub struct Player {
    pub type_id: u16,
    pub position: Vector2<i16>,
    pub height: i16,
    pub angle: u16,
    pub flags: u16
}

impl Player {
    pub fn new<'wad>(thing: &'wad Thing) -> Box<dyn Actor> {
        Box::new(Player {
            type_id: thing.type_id,
            position: thing.position,
            height: 45,
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
        if  input.key_held(KeyCode::KeyW) 
        && !input.key_held(KeyCode::KeyS) {
            self.position += Vector2::new(0, 1);
        }
        if !input.key_held(KeyCode::KeyW) 
        &&  input.key_held(KeyCode::KeyS) {
            self.position -= Vector2::new(0, 1);
        }        
        if  input.key_held(KeyCode::KeyA) 
        && !input.key_held(KeyCode::KeyD) {
            self.position -= Vector2::new(1, 0);
        }
        if !input.key_held(KeyCode::KeyA) 
        &&  input.key_held(KeyCode::KeyD) {
            self.position += Vector2::new(1, 0);
        }      
        if  input.key_held(KeyCode::ArrowLeft) 
        && !input.key_held(KeyCode::ArrowRight) {
            self.angle = normalize_degrees(self.angle as f32 + 1.0) as u16;
        }
        if !input.key_held(KeyCode::ArrowLeft) 
        &&  input.key_held(KeyCode::ArrowRight) {
            self.angle = normalize_degrees(self.angle as f32 - 1.0) as u16;
        }
    }

    fn type_id(&self) -> u16 {
        self.type_id
    }
    
    fn position(&self) -> &Vector2<i16>{
        &self.position
    }

    fn height(&self) -> &i16 {
        &self.height
    }

    fn angle(&self) -> u16 {
        self.angle
    }

    fn flags(&self) -> u16 {
        self.flags
    }
}