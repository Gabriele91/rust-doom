
// Engine
use crate::math::{Vector2, normalize_degrees, radians};
use crate::map::Thing;
use crate::doom::Doom;
use crate::configure;
use crate::types::ThingType;
use crate::collision::CollisionClass;
// Utils
use std::boxed::Box;
use winit::keyboard::KeyCode;
use winit_input_helper::WinitInputHelper;

#[derive(Debug, Clone)]
pub struct Transform {
    pub position: Vector2<f32>,
    pub angle: f32,
    pub height: f32
}

impl Transform {
    pub fn new() -> Self {
        Transform {
            position: Vector2::new(0.0,0.0),
            angle: 0.0,
            height: 0.0
        } 
    }

    pub fn set(position: Vector2<f32>, angle: f32, height: f32) -> Self {
        Transform {
            position: position,
            angle: angle,
            height: height
        }
    }

    pub fn position(&self) -> Vector2<f32> {
        self.position
    }

    pub fn angle(&self) -> f32 {
        self.angle
    }

    pub fn height(&self) -> f32 {
        self.height
    }

    pub fn position_as_int(&self) -> Vector2<i16> {
        Vector2::<i16>::from(&self.position.round())
    }

    pub fn angle_as_int(&self) -> i16 {
        self.angle.round() as i16
    }

    pub fn height_as_int(&self) -> i16 {
        self.height.round() as i16
    }

}

pub trait Actor {
    fn control(&mut self, input: &WinitInputHelper, last_frame_time: f64, blending_factor: f64);
    fn update<'wad>(&mut self, engine: &Doom<'wad>, last_frame_time: f64, blending_factor: f64); 

    fn type_id(&self) -> u16;
    fn thing_type(&self) -> ThingType;
    fn collision_class(&self) -> CollisionClass;
    fn flags(&self) -> u16;
    fn size(&self) -> u16;

    // Transform alias
    fn position(&self) -> &Vector2<f32>;
    fn angle(&self) -> f32;
    fn height(&self) -> f32;

    // Transform
    fn get_last_transform(&self) -> &Transform;
    fn get_transform(&self) -> &Transform;
    fn set_transform(&mut self, transform: &Transform);
}

pub struct Player {
    type_id: u16,
    thing_type: ThingType,
    collision_class: CollisionClass,
    flags: u16,
    // Transformation
    transform: Transform,
    last_transform: Transform,
    configure: configure::Player,
    // Control
    control_direction: Vector2<f32>, 
    control_angle: f32,
    control_angle_update: f32,
    player_jump: f32,
    player_jump_lock: bool,
}

impl Player {
    pub fn new<'wad>(thing: &'wad Thing, configure: &configure::Configure) -> Box<dyn Actor> {
        let transform = {
            Transform::set({
                let position_i16 = thing.position;
                Vector2::<f32>::from(&position_i16)
            },  
            thing.angle as f32, 
            configure.player.height as f32)
        };
        let thing_type =  ThingType::try_from(thing.type_id).unwrap_or(ThingType::Unknown);
        Box::new(Player {
            type_id: thing.type_id,
            thing_type: thing_type,
            collision_class: CollisionClass::new(thing_type),
            flags: thing.flags,
            last_transform: transform.clone(),
            transform: transform.clone(),
            configure: configure.player.clone(),
            // Control
            control_direction: Vector2::zeros(),
            control_angle: 0.0,
            control_angle_update: 0.0,
            player_jump: 0.0,
            player_jump_lock: false
        })
    }
}

#[allow(unused_variables)]
impl Actor for Player {
    fn update<'wad>(&mut self, engine: &Doom<'wad>, last_frame_time: f64, blending_factor: f64) {
        let last_frame_time = last_frame_time as f32;
        self.last_transform = self.transform.clone();
        // Angle
        if self.control_angle != 0.0 {
            self.control_angle /= self.control_angle_update;
            self.transform.angle = normalize_degrees(self.transform.angle + (self.control_angle * self.configure.angle_speed * last_frame_time));
            self.control_angle = 0.0;
            self.control_angle_update = 0.0;
        }
        // Get move direction
        if self.control_direction.x != 0.0 || self.control_direction.y != 0.0 {
            let direction = self.control_direction.normalize();
            self.control_direction = Vector2::zeros();
            // Move rotation
            let psin = radians(self.transform.angle - 90.0).sin();
            let pcos = radians(self.transform.angle - 90.0).cos();
            // Delta
            let velocity = Vector2::new(
                direction.x * pcos - direction.y * psin,
                direction.x * psin + direction.y * pcos,
            ) * self.configure.speed;
            // New position
            self.transform.position += velocity * last_frame_time;
        }
        // Height
        if self.player_jump_lock {
            self.player_jump -= self.configure.jump_speed * last_frame_time;
            if self.player_jump <= 0.0 {
                self.player_jump_lock = false;
                self.player_jump = 0.0;
            }
        }
        self.transform.height = engine.bsp.floor_height(&self.transform.position_as_int()) as f32 + self.configure.height as f32 + self.player_jump;

    }

    fn control(&mut self, input: &WinitInputHelper, last_frame_time: f64, blending_factor: f64) {
        let last_frame_time = last_frame_time as f32;
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
        && self.player_jump < self.configure.jump
        && !self.player_jump_lock {
            self.player_jump += self.configure.jump_speed * last_frame_time;
        } else if self.player_jump != 0.0 {
            self.player_jump_lock = true;
        }
    }

    fn type_id(&self) -> u16 {
        self.type_id
    }

    fn thing_type(&self) -> ThingType {
        self.thing_type
    }

    fn collision_class(&self) -> CollisionClass {
        self.collision_class
    }

    fn flags(&self) -> u16 {
        self.flags
    }    

    fn size(&self) -> u16 {
        self.configure.size
    }    

    fn position(&self) -> &Vector2<f32> {
        &self.transform.position
    }

    fn angle(&self) -> f32 {
        self.transform.angle()
    }

    fn height(&self) -> f32 {
        self.transform.height()
    }

    fn get_transform(&self) -> &Transform {
        &self.transform
    }

    fn get_last_transform(&self) -> &Transform {
        &self.last_transform
    }

    fn set_transform(&mut self, transform: &Transform) {
        self.transform = transform.clone();
    }
}