#![allow(dead_code)]
use num_traits::{Float, NumCast};
use crate::{map::{LineDef, Map}, math::Vector2};


mod aabb_utils {
    use crate::math::Vector2;
    use num_traits::Float;
    use std::cmp::{min, max};

    // Check if a point is inside the AABB
    fn point_in_aabb<T: Float>(p: &Vector2<T>, aabb_position: &Vector2<T>, size: T) -> bool {
        let left = aabb_position.x - size;
        let right = aabb_position.x + size;
        let top = aabb_position.y + size;
        let bottom = aabb_position.y - size;

        p.x >= left && p.x <= right && p.y >= bottom && p.y <= top
    }

    // Check if a line segment intersects a vertical line at x
    fn intersects_vertical_segment<T: Float + Ord + Copy>(p1: &Vector2<T>, p2: &Vector2<T>, x: T) -> bool {
        if p1.x == p2.x { return false; }
        
        // Ensure the line crosses the vertical line at `x`
        let t = (x - p1.x) / (p2.x - p1.x);
        let y_intersection = p1.y + t * (p2.y - p1.y);
        
        min(p1.y, p2.y) <= y_intersection && y_intersection <= max(p1.y, p2.y)
    }

    // Check if a line segment intersects a horizontal line at y
    fn intersects_horizontal_segment<T: Float + Ord + Copy>(p1: &Vector2<T>, p2: &Vector2<T>, y: T) -> bool {
        if p1.y == p2.y { return false; }
        
        // Ensure the line crosses the horizontal line at `y`
        let t = (y - p1.y) / (p2.y - p1.y);
        let x_intersection = p1.x + t * (p2.x - p1.x);
        
        min(p1.x, p2.x) <= x_intersection && x_intersection <= max(p1.x, p2.x)
    }

    // Main function to check if the AABB and line segment intersect
    fn aabb_intersects_segment<T: Float + Ord + Copy>(aabb_position: Vector2<T>, size: T, p1: Vector2<T>, p2: Vector2<T>) -> bool {
        // Step 1: Check if either endpoint of the segment is inside the AABB
        if point_in_aabb(&p1, &aabb_position, size) || point_in_aabb(&p2, &aabb_position, size) {
            return true;
        }
        
        // Step 2: Check intersection with each of the AABB sides
        // Left side
        if intersects_vertical_segment(&p1, &p2, aabb_position.x - size) {
            return true;
        }
        // Right side
        if intersects_vertical_segment(&p1, &p2, aabb_position.x + size) {
            return true;
        }
        // Top side
        if intersects_horizontal_segment(&p1, &p2, aabb_position.y + size) {
            return true;
        }
        // Bottom side
        if intersects_horizontal_segment(&p1, &p2, aabb_position.y - size) {
            return true;
        }
        
        false
    }
}

/// Attempts to move an object while handling collisions with line segments
pub fn try_move<'a, T: Float + Sized + Copy + NumCast + Default>(
    position: &Vector2<T>,
    velocity: &Vector2<T>,
    radius: T,
    map: &Map<'a>,
    line: &LineDef,
) -> Vector2<T> {
    // Get line segment points
    let start = Vector2::<T>::from(&line.end_vertex(&map));
    let end = Vector2::<T>::from(&line.start_vertex(&map));

    // Calculate wall direction and normal
    let wall_vec = end - start;
    let wall_length = wall_vec.magnitude();
    
    if wall_length < T::epsilon() {
        return *velocity;
    }

    let wall_dir = wall_vec.normalize();
    let wall_normal = Vector2::new(-wall_dir.y, wall_dir.x);

    // Check distance to wall
    let to_wall = *position - start;
    let perp_dist = to_wall.dot(&wall_normal);

    // DOOM-style collision check:
    // 1. Only check if we're within collision range
    if perp_dist.abs() > radius + T::from(1.0).unwrap() {
        return *velocity;
    }

    // 2. Project current position onto wall line
    let along_wall = to_wall.dot(&wall_dir);
    
    // 3. Check if we're actually near the wall segment
    if along_wall < -radius || along_wall > wall_length + radius {
        return *velocity;
    }

    // 4. Check if we're moving towards the wall
    let vel_towards_wall = velocity.dot(&wall_normal);
    if vel_towards_wall >= T::zero() {
        return *velocity;
    }

    // DOOM-style response:
    // 1. If we're too close or inside, push out
    let min_dist = radius + T::from(0.01).unwrap();  // Small buffer
    if perp_dist.abs() < min_dist {
        let push_out = wall_normal * (min_dist - perp_dist.abs());
        
        // 2. Split velocity into parallel and perpendicular components
        let vel_parallel = wall_dir * velocity.dot(&wall_dir);
        
        // 3. DOOM keeps the parallel component (sliding) and removes the perpendicular
        // Also apply the push-out correction
        return vel_parallel + push_out;
    }

    // If we're just moving towards wall but not too close yet,
    // do DOOM-style sliding
    let vel_parallel = wall_dir * velocity.dot(&wall_dir);
    vel_parallel
}

fn ray_intersects_segment<T: Float>(
    p1: &Vector2<T>,
    p2: &Vector2<T>,
    q1: &Vector2<T>,
    q2: &Vector2<T>
) -> Option<Vector2<T>> {
    // Calculate the direction vectors of the segments
    let r = *p2 - *p1; // Vector for the first segment
    let s = *q2 - *q1; // Vector for the second segment

    // Calculate the determinant
    let det = -r.x * s.y + r.y * s.x;

    if !det.is_zero() {
        // Parametric equations to find t1 and t2
        let t1 = ((q1.x - p1.x) * s.y - (q1.y - p1.y) * s.x) / det;
        let t2 = ((p1.x - q1.x) * r.y - (p1.y - q1.y) * r.x) / det;

        // Check if t1 and t2 lie within the bounds [0, 1] (i.e., intersection is within the segments)
        if  t1 >= T::zero() && t1 <= T::one()
         && t2 >= T::zero() && t2 <= T::one() {
            // Compute the intersection point
            let newpoint = *p1 + r * t1;
            return Some(newpoint);
        }
    }
    // None
    return None;
    
}


