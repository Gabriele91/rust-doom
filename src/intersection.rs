#![allow(dead_code)]
use num_traits::Float;
use crate::math::Vector2;

pub enum Result<T:Float> {
    NOCOLLIDE,
    COLLIDE(T,Vector2<T>)
}

pub struct Sphere<T:Float> {
    center: Vector2<T>,
    radius: T,
}

pub struct Segment<T: Float> {
    start: Vector2<T>,
    end: Vector2<T>,
}

impl<T: Float> Segment<T> {
    pub fn magnitude(&self) -> T {
        self.start.distance(&self.end)
    }

    pub fn direction(&self) -> Vector2<T> {
        self.end - self.start
    }

    pub fn normalized_direction(&self) -> Vector2<T> {
        self.direction().normalize()
    }
}

// Function to check the collision between the sphere and sphere
pub fn sphere_vs_sphere<T: Float>(sphere1: &Sphere<T>, sphere2: &Sphere<T>) -> Result<T> {
    let radius_total = sphere1.radius + sphere2.radius;
    let square_dist = sphere1.center.dot(&sphere2.center);
    let direction = sphere2.center - sphere1.center;

    if square_dist < radius_total * radius_total {
        let penetration_depth = radius_total - direction.magnitude();
        let normalized_direction = direction.normalize();
        Result::COLLIDE(penetration_depth, normalized_direction)
    } else {
        Result::NOCOLLIDE // No collision
    }
}

// Function to check the collision between the sphere and segment
pub fn sphere_vs_segment<T: Float>(sphere:&Sphere<T>, segment: &Segment<T>) -> Result<T> {
    // Calculate the vector from the sphere's center to the segment's start point
    let segment_dir = segment.direction(); // Get the direction of the segment
    let sphere_to_start = sphere.center - segment.start;

    // Project the vector from the sphere center to the segment's start onto the segment direction
    let t = sphere_to_start.dot(&segment_dir) / segment_dir.dot(&segment_dir);

    // Clamp the value of t to the bounds of the segment [0, 1]
    let t_clamped = t.max(T::zero()).min(T::one());

    // Calculate the closest point on the segment to the sphere's center
    let closest_point = segment.start + segment_dir * t_clamped;

    // Calculate the distance squared from the closest point to the sphere's center
    let dist_squared = sphere.center.dot(&closest_point);

    // Check if the sphere's radius is greater than the distance to the closest point
    if dist_squared < sphere.radius * sphere.radius {
        // If so, there's a collision
        let direction = closest_point - sphere.center; // Direction from sphere to collision point
        let magnitude = direction.magnitude(); // Calculate the magnitude of the collision vector
        let normalized_direction = direction.normalize(); // Normalize the direction vector
        Result::COLLIDE(magnitude, normalized_direction)
    } else {
        Result::NOCOLLIDE
    }
}