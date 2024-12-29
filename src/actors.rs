
// Engine
use crate::math::{Vector2, normalize_degrees, radians};
use crate::map::Thing;
use crate::doom::Doom;
use crate::configure::Configure;
// Utils
use std::boxed::Box;
use winit::keyboard::KeyCode;
use winit_input_helper::WinitInputHelper;

pub struct Movement {
    pub position: Vector2<f32>,
    pub angle: f32,
    pub height: i16
}

pub trait Actor {

    fn control(&mut self, input: &WinitInputHelper, last_frame_time: f64, blending_factor: f64);
    fn compute<'wad>(&mut self, engine: &Doom<'wad>, last_frame_time: f64, blending_factor: f64) -> Movement; 
    fn apply<'wad>(&mut self, engine: &Doom<'wad>, movement: &Movement);

    fn type_id(&self) -> u16;
    fn position(&self) -> &Vector2<i16>;
    fn height(&self) -> &i16;
    fn angle(&self) -> u16;
    fn flags(&self) -> u16;
    // Best precision
    fn float_position(&self) -> &Vector2<f32>;
    fn float_angle(&self) -> f32;
}

pub struct Player {
    type_id: u16,
    position: Vector2<i16>,
    height: i16,
    angle: u16,
    flags: u16,
    // local
    internal_position: Vector2<f32>,
    internal_angle: f32,
    // Control
    control_direction: Vector2<f32>, 
    control_angle: f32,
    control_angle_update: f32,
    // Settings
    speed: f32,
    angle_speed: f32,
    player_height: i16,
    // Jump
    player_jump_max: i16,
    player_jump_speed: i16,
    player_jump: i16,
    player_jump_lock: bool,
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
            internal_position: Vector2::<f32>::from(&position),
            internal_angle: thing.angle as f32,
            // Control
            control_direction: Vector2::zeros(),
            control_angle: 0.0,
            control_angle_update: 0.0,
            // Configure
            speed: configure.player.speed,
            angle_speed: configure.player.angle_speed,
            player_height: configure.player.height,
            player_jump_max: configure.player.jump,
            player_jump_speed: configure.player.jump_speed,
            player_jump: 0,
            player_jump_lock: false
        })
    }
}

#[allow(unused_variables)]
impl Actor for Player {
    fn compute<'wad>(&mut self, engine: &Doom<'wad>, last_frame_time: f64, blending_factor: f64) -> Movement {
        let mut movement = Movement {
            position: self.internal_position,
            angle: self.internal_angle,
            height: engine.bsp.floor_height(self.position()) + self.player_height
        };
        // Angle
        if self.control_angle != 0.0 {
            self.control_angle /= self.control_angle_update;
            movement.angle = normalize_degrees(self.internal_angle + (self.control_angle * self.angle_speed));
            self.control_angle = 0.0;
            self.control_angle_update = 0.0;
        }
        // Get move direction
        if self.control_direction.x != 0.0 || self.control_direction.y != 0.0 {
            let direction = self.control_direction.normalize();
            self.control_direction = Vector2::zeros();
            // Move rotation
            let psin = radians(self.internal_angle - 90.0).sin();
            let pcos = radians(self.internal_angle - 90.0).cos();
            // Delta
            let velocity = Vector2::new(
                direction.x * pcos - direction.y * psin,
                direction.x * psin + direction.y * pcos,
            ) * self.speed;
            // New position
            movement.position = self.internal_position + velocity;
        }
        // Height
        if self.player_jump_lock {
            self.player_jump -= self.player_jump_speed;
            if self.player_jump <= 0 {
                self.player_jump_lock = false;
                self.player_jump = 0;
            }
        }
        if 0 < self.player_jump {
            movement.height += self.player_jump;
        }
        // Return
        return movement;
    }

    fn apply<'wad>(&mut self, engine: &Doom<'wad>, movement: &Movement) {
        self.internal_position = movement.position;
        self.internal_angle = movement.angle;
        self.position = Vector2::<i16>::from(&movement.position.round());
        self.angle = movement.angle.round() as u16;
        self.height = movement.height;  
    }

    fn control(&mut self, input: &WinitInputHelper, last_frame_time: f64, blending_factor: f64) {
        if  input.key_held(KeyCode::KeyW) 
        && !input.key_held(KeyCode::KeyS) {
            self.control_direction += Vector2::new(0.0, 1.0);
        }
        if !input.key_held(KeyCode::KeyW) 
        &&  input.key_held(KeyCode::KeyS) {
            self.control_direction -= Vector2::new(0.0, 1.0);
        }   
        if  input.key_held(KeyCode::KeyA) 
        && !input.key_held(KeyCode::KeyD) {
            self.control_direction -= Vector2::new(1.0, 0.0);
        }
        if !input.key_held(KeyCode::KeyA) 
        &&  input.key_held(KeyCode::KeyD) {
            self.control_direction += Vector2::new(1.0, 0.0);
        }      
        if  input.key_held(KeyCode::ArrowLeft) 
        && !input.key_held(KeyCode::ArrowRight) {
            self.control_angle += 1.0;
            self.control_angle_update += 1.0;
        }
        if !input.key_held(KeyCode::ArrowLeft) 
        &&  input.key_held(KeyCode::ArrowRight) {
            self.control_angle -= 1.0;
            self.control_angle_update += 1.0;
        }
        if input.key_held(KeyCode::KeyE) 
        && self.player_jump < self.player_jump_max 
        && !self.player_jump_lock {
            self.player_jump += self.player_jump_speed;
        } else if self.player_jump != 0 {
            self.player_jump_lock = true;
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

    fn float_position(&self) -> &Vector2<f32> {
        &self.internal_position
    }

    fn float_angle(&self) -> f32 {
        self.internal_angle
    }

}