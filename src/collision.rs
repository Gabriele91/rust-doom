#![allow(dead_code)]
use std::fmt::Display;
use std::{ops::Div, rc::Rc};
use num_traits::{Float, NumCast};
use crate::math::Vector2;
use crate::doom::Doom;
use crate::map::{LineDef, Map, LineDefFlags};
use crate::types::ThingType;

#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(u16)]
pub enum CollisionClass {
    Player,
    Monster,
    Static,
    Projectile,
    Pickup,
}

impl CollisionClass {
    pub fn new(thing_type: ThingType) -> Self {
        match thing_type {
            // Players
            ThingType::Player1Start 
            | ThingType::Player2Start 
            | ThingType::Player3Start 
            | ThingType::Player4Start => CollisionClass::Player,

            // Monsters
            ThingType::ZombieMan
            | ThingType::ShotgunGuy
            | ThingType::HeavyWeaponDude
            | ThingType::Imp
            | ThingType::Demon
            | ThingType::Spectre
            | ThingType::LostSoul
            | ThingType::Cacodemon
            | ThingType::HellKnight
            | ThingType::BaronOfHell
            | ThingType::Arachnotron
            | ThingType::PainElemental
            | ThingType::Revenant
            | ThingType::Mancubus
            | ThingType::ArchVile
            | ThingType::Cyberdemon
            | ThingType::SpiderDemon => CollisionClass::Monster,

            // Pickups
            ThingType::Shotgun
            | ThingType::SuperShotgun
            | ThingType::Chaingun
            | ThingType::RocketLauncher
            | ThingType::PlasmaRifle
            | ThingType::BFG9000
            | ThingType::Shell4
            | ThingType::BulletBox
            | ThingType::RocketBox
            | ThingType::ShellBox
            | ThingType::Clip
            | ThingType::EnergyCell
            | ThingType::EnergyPack => CollisionClass::Pickup,

            // Statics
            ThingType::GreenArmor
            | ThingType::BlueArmor
            | ThingType::Stimpack
            | ThingType::Medikit
            | ThingType::Berserk
            | ThingType::Soulsphere
            | ThingType::Invulnerability
            | ThingType::LightAmp
            | ThingType::ComputerMap
            | ThingType::RadSuit => CollisionClass::Static,

            // Default case
            _ => CollisionClass::Static,
        }
    }

    pub fn can_collide_with(&self, other_class: &CollisionClass) -> bool {        
        match (*self, *other_class) {
            (CollisionClass::Player, CollisionClass::Monster) => true,
            (CollisionClass::Player, CollisionClass::Static) => true,
            (CollisionClass::Monster, CollisionClass::Static) => true,
            (CollisionClass::Monster, CollisionClass::Monster) => true,
            _ => false
        }
    }

    pub fn is_shootable(&self) -> bool {
        matches!(*self, CollisionClass::Monster | CollisionClass::Player)
    }

    pub fn blocks_movement(&self) -> bool {
        matches!(*self, CollisionClass::Monster | CollisionClass::Static | CollisionClass::Player)
    }
}

fn get_default_radius(thing_type:&ThingType) -> i32 {
    match thing_type {
        // Players
        ThingType::Player1Start 
        | ThingType::Player2Start 
        | ThingType::Player3Start 
        | ThingType::Player4Start => 16,

        // Specific monsters
        ThingType::ZombieMan 
        | ThingType::ShotgunGuy 
        | ThingType::HeavyWeaponDude => 20,

        ThingType::Imp => 20,
        ThingType::Demon 
        | ThingType::Spectre => 30,

        ThingType::Cacodemon => 31,
        ThingType::LostSoul => 16,
        ThingType::BaronOfHell => 24,

        // Default case
        _ => 20,
    }
}

pub struct CollisionSolver<'wad> {
    map: Rc<Map<'wad>>,
}

impl <'wad> CollisionSolver<'wad> {
    pub fn new(map: &Rc<Map<'wad>>) -> Self {
        CollisionSolver {
            map: map.clone()
        }
    }

    pub fn update(&self, engine: &mut Doom<'wad>, last_frame_time: f64, blending_factor: f64) {
        for rc_actor in engine.actors.iter() {
            // Actor
            let mut actor =  rc_actor.borrow_mut();
            // Move
            let old_transformation = actor.get_last_transform();
            let mut transformation = actor.get_transform().clone();
            // Collision
            match actor.collision_class() {
                  CollisionClass::Player 
                | CollisionClass::Monster 
                | CollisionClass::Projectile => {
                    if let Some(ref map) = engine.map.blockmaps {
                        let position = actor.get_transform().position_as_int();
                        for list_lines in map.get_with_radius(position.x, position.y, actor.size()) {
                            for line in list_lines.iter() {
                                match actor.collision_class() {
                                    CollisionClass::Player => {
                                        if line.has_flag(LineDefFlags::Blocking) {
                                            transformation.position = self.try_move(
                                                &old_transformation.position(), 
                                                &transformation.position(),
                                                actor.size() as f32, 
                                                &engine.map, 
                                                &line);
                                        }
                                    },
                                    CollisionClass::Monster => {
                                        if line.has_flag(LineDefFlags::Blocking) || line.has_flag(LineDefFlags::BlockMonsters) {
                                            transformation.position = self.try_move(
                                                &old_transformation.position(), 
                                                &transformation.position(),
                                                actor.size() as f32, 
                                                &engine.map, 
                                                &line);
                                        }
                                    },
                                    _ => {}
                                }
                            }
                        }
                    }
                },
                _ => {}
            }
            actor.set_transform(&transformation);
        }
    }

    
    pub fn try_move<'a, T: Float + Sized + Copy + NumCast + Default + Div + Display>(
        &self,
        position: &Vector2<T>,
        newposition: &Vector2<T>,
        radius: T,
        map: &Map<'a>,
        line: &LineDef,
    ) -> Vector2<T> {
        let velocity = *newposition - *position;
        let velocity_length = velocity.magnitude();
        // Get line segment points (note: fixed order from end to start)
        let wall_start = Vector2::<T>::from(&line.start_vertex(&map));
        let wall_end = Vector2::<T>::from(&line.end_vertex(&map));
        
        // If velocity is small enough, use single check
        if velocity_length <= radius {
            return self.single_collision_check(position, newposition, radius, &wall_start, &wall_end);
        }
        
        // Multiple checks for high velocity
        let steps = (velocity_length / radius).ceil();
        let inv_steps = T::one() / steps;
        let step_velocity = velocity * inv_steps;
        
        let mut current_pos = *position;
        let mut next_pos;
        
        // Perform multiple smaller steps
        for _ in 0..steps.to_u32().unwrap() {
            next_pos = current_pos + step_velocity;
            current_pos = self.single_collision_check(&current_pos, &next_pos, radius, &wall_start, &wall_end);
        }
        
        return current_pos;
    }
    
    fn single_collision_check<'a, T: Float + Sized + Copy + NumCast + Default + Div + Display>(
        &self,
        position: &Vector2<T>,
        newposition: &Vector2<T>,
        radius: T,
        wall_start: &Vector2<T>,
        wall_end: &Vector2<T>
    ) -> Vector2<T> {
        // Calculate attempted movement vector
        let movement = *newposition - *position;
        
        // Calculate wall properties
        let wall_vec = *wall_end - *wall_start;
        let wall_length = wall_vec.magnitude();
        
        // Handle degenerate walls
        if wall_length < T::epsilon() {
            return *newposition;
        }
        
        let wall_dir = wall_vec * (T::one() / wall_length);
        let wall_normal = Vector2::new(wall_dir.y, -wall_dir.x);
    
        // Vector from wall start to current position
        let to_wall = *position - *wall_start;
        
        // Distance to wall along its normal
        let perp_dist = to_wall.dot(&wall_normal);
        
        // Early out if we're too far from wall
        let collision_margin = radius + T::one();
        if perp_dist.abs() > collision_margin {
            return *newposition;
        }
    
        // Project position onto wall line to check if we're near the segment
        let along_wall = to_wall.dot(&wall_dir);
        
        // Check if we're beyond wall endpoints (considering radius)
        if along_wall < -radius || along_wall > wall_length + radius {
            return *newposition;
        }
    
        // Check if the attempted movement is towards the wall
        let movement_towards_wall = movement.dot(&wall_normal);
        if movement_towards_wall >= T::zero() {
            return *newposition;
        }
    
        // DOOM-style collision response
        let min_separation = radius + T::from(0.01).unwrap(); // Small buffer for numerical stability
    
        // If we're too close to or inside the wall
        if perp_dist.abs() < min_separation {
            // Calculate push-out vector to maintain minimum separation
            let push_out = if perp_dist < T::zero() {
                wall_normal * (perp_dist.abs() - min_separation)
            } else {
                wall_normal * (min_separation - perp_dist)
            };
    
            // Calculate movement parallel to wall (sliding)
            let parallel_movement = wall_dir * movement.dot(&wall_dir);
            
            // New position is: original position + push-out + allowed parallel movement
            return *position + push_out + parallel_movement;
        }
    
        // If we're moving towards the wall but not too close
        // Project movement onto wall direction for sliding
        let parallel_movement = wall_dir * movement.dot(&wall_dir);
        *position + parallel_movement
    }

}
