
// Engine
use crate::math::{Vector2, normalize_degrees, radians};
use crate::map::Thing;
use crate::doom::Doom;
use crate::configure::Configure;

// Utils
use std::boxed::Box;
use winit::keyboard::KeyCode;
use winit_input_helper::WinitInputHelper;

pub trait Actor {
    fn update<'wad>(&mut self, engine: &Doom<'wad>, last_frame_time: f64, blending_factor: f64);
    fn control(&mut self, input: &WinitInputHelper, last_frame_time: f64, blending_factor: f64);
    fn type_id(&self) -> u16;
    fn position(&self) -> &Vector2<i16>;
    fn height(&self) -> &i16;
    fn angle(&self) -> u16;
    fn flags(&self) -> u16;
}

pub struct Player {
    type_id: u16,
    position: Vector2<i16>,
    height: i16,
    angle: u16,
    flags: u16,
    // local
    internal_increment: Vector2<f32>, 
    internal_position: Vector2<f32>,
    internal_angle: f32,
    internal_height: i16,
    // Settings
    speed: f32,
    angle_speed: f32,
    player_height: i16,

}

impl Player {
    pub fn new<'wad>(thing: &'wad Thing, configure: &Configure) -> Box<dyn Actor> {
        let position = thing.position;
        let angle = thing.angle;
        Box::new(Player {
            type_id: thing.type_id,
            position: position,
            height: 0,
            angle: angle,
            flags: thing.flags,
            // local            
            internal_increment: Vector2::zeros(),
            internal_position: Vector2::<f32>::from(&position),
            internal_angle: thing.angle as f32,
            internal_height:0,
            // Configure
            speed: configure.player.speed,
            angle_speed: configure.player.angle_speed,
            player_height: configure.player.height,
        })
    }
}

#[allow(unused_variables)]
impl Actor for Player {
    fn update<'wad>(&mut self, engine: &Doom<'wad>, last_frame_time: f64, blending_factor: f64) {
        // Position
        let psin = radians(self.internal_angle - 90.0).sin();
        let pcos = radians(self.internal_angle - 90.0).cos();
        self.internal_position += Vector2::new(
            self.internal_increment.x * pcos - self.internal_increment.y * psin,
            self.internal_increment.x * psin + self.internal_increment.y * pcos,
        );
        self.position = Vector2::<i16>::from(&self.internal_position);
        self.internal_increment = Vector2::zeros();
        // Height
        self.internal_height = engine.bsp.floor_height(self.position());
        self.height = self.internal_height + self.player_height;    
        // Angle
        self.angle = self.internal_angle as u16;
    }

    fn control(&mut self, input: &WinitInputHelper, last_frame_time: f64, blending_factor: f64) {
        if  input.key_held(KeyCode::KeyW) 
        && !input.key_held(KeyCode::KeyS) {
            self.internal_increment += Vector2::new(0.0, self.speed);
        }
        if !input.key_held(KeyCode::KeyW) 
        &&  input.key_held(KeyCode::KeyS) {
            self.internal_increment -= Vector2::new(0.0, self.speed);
        }        
        if  input.key_held(KeyCode::KeyA) 
        && !input.key_held(KeyCode::KeyD) {
            self.internal_increment -= Vector2::new(self.speed, 0.0);
        }
        if !input.key_held(KeyCode::KeyA) 
        &&  input.key_held(KeyCode::KeyD) {
            self.internal_increment += Vector2::new(self.speed, 0.0);
        }      
        if  input.key_held(KeyCode::ArrowLeft) 
        && !input.key_held(KeyCode::ArrowRight) {
            self.internal_angle = normalize_degrees(self.internal_angle + self.angle_speed);
        }
        if !input.key_held(KeyCode::ArrowLeft) 
        &&  input.key_held(KeyCode::ArrowRight) {
            self.internal_angle = normalize_degrees(self.internal_angle - self.angle_speed);
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
        self.angle as u16
    }

    fn flags(&self) -> u16 {
        self.flags
    }    

}