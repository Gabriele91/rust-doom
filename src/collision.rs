#![allow(dead_code)]
use std::{ops::Div, rc::Rc};
use num_traits::{Float, NumCast};
use crate::{ 
    actors, doom::Doom, map::{LineDef, Map}, math::Vector2
};


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
            if let Some(ref map) = engine.map.blockmaps {
                let position = actor.get_transform().position_as_int();
                if let Some(list_lines) = map.get(position.x, position.y) {
                    for line in list_lines.iter() {
                        if (line.flag & 0x0001) != 0 {
                            transformation.position = self.try_move(
                                &old_transformation.position(), 
                                &transformation.position(),
                                actor.size() as f32, 
                                &engine.map, 
                                &line);
                        }
                    }
                }
            }
            actor.set_transform(&transformation);
        }
    }

    
    pub fn try_move<'a, T: Float + Sized + Copy + NumCast + Default + Div>(
        &self,
        position: &Vector2<T>,
        newposition: &Vector2<T>,
        radius: T,
        map: &Map<'a>,
        line: &LineDef,
    ) -> Vector2<T> {
        // Get line segment points (note: fixed order from end to start)
        let start = Vector2::<T>::from(&line.start_vertex(&map));
        let end = Vector2::<T>::from(&line.end_vertex(&map));
        
        // Calculate attempted movement vector
        let movement = *newposition - *position;
        
        // Calculate wall properties
        let wall_vec = end - start;
        let wall_length = wall_vec.magnitude();
        
        // Handle degenerate walls
        if wall_length < T::epsilon() {
            return *newposition;
        }
        
        let wall_dir = wall_vec * (T::one() / wall_length);
        let wall_normal = Vector2::new(wall_dir.y, -wall_dir.x);
    
        // Vector from wall start to current position
        let to_wall = *position - start;
        
        // Distance to wall along its normal
        let perp_dist = to_wall.dot(&wall_normal);
        
        // Early out if we're too far from wall
        let collision_margin = radius + T::from(1.0).unwrap();
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
    
    // Helper function to check if a point is on the backside of a line
    fn is_backside<T: Float>(
        point: &Vector2<T>,
        line_start: &Vector2<T>,
        line_end: &Vector2<T>
    ) -> bool {
        let dx = line_end.x - line_start.x;
        let dy = line_end.y - line_start.y;
        let px = point.x - line_start.x;
        let py = point.y - line_start.y;
        
        (px * dy - py * dx) > T::zero()
    }

}
