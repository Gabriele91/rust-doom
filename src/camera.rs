use num_traits::NumCast;
use crate::actors::Actor;
use crate::map::NodeBox;
use crate::math::{angle, degrees, normalize_degrees, radians, Vector2, self};
use crate::shape::Rectangle;

#[derive(Clone)]
#[readonly::make]
pub struct Camera {
    pub fov: f32,
    pub half_fov: f32,
    pub half_width: u32,
    pub screen_dist: f32,
    // Cache tables
    x_to_angle : Vec<f32>
}

impl Camera {
    pub fn new(fov: f32, width: u32) -> Self {
        let half_fov = fov / 2.0;
        let half_width = width / 2;
        let screen_dist = half_width as f32 / radians(half_fov).tan();
        Camera {
            fov,
            half_fov,
            half_width,
            screen_dist, 
            x_to_angle : {
                let mut table = vec![];
                table.reserve(width as usize + 1);
                for i in 0..=width as i32 {
                    table.push(math::degrees(((half_width as i32 - i) as f32 / screen_dist).atan()))
                }
                table
            }
        }
    }

    pub fn angle_to_x(&self, angle: f32) -> u32 {
        if angle > 0.0 {
            (self.screen_dist - (radians(angle).tan() * self.half_width as f32))  as u32
        } else {
            ((-radians(angle).tan() * self.half_width as f32) + self.screen_dist) as u32
        }
    }

    pub fn x_to_angle(&self, x: u32) -> f32 {
        self.x_to_angle[x as usize]
    }    
    
    pub fn scale_from_global_angle(&self, x: u32, wall_normal_angle1: f32, wall_distance: f32, actor_angle: f32) -> f32 {
        let x_angle = self.x_to_angle(x);
        let num = self.screen_dist * (math::radians(wall_normal_angle1 - x_angle - actor_angle)).cos();
        let den = wall_distance * math::radians(x_angle).cos();
        let scale = num / den;
        return scale;
    }

    pub fn is_box_in_frustum(&self, actor: &dyn Actor, bbox: &NodeBox) -> bool {
        let (a, b) = (
            Vector2::new(bbox.left(), bbox.bottom()),
            Vector2::new(bbox.left(), bbox.top()),
        );
        let (c, d) = (
            Vector2::new(bbox.right(), bbox.top()),
            Vector2::new(bbox.right(), bbox.bottom()),
        );

        let position = actor.get_transform().position_as_int();
        let mut bbox_sides = vec![];
        if position.x < bbox.left() {
            if position.y > bbox.top() {
                bbox_sides.push((b, a));
                bbox_sides.push((c, b));
            } else if position.y < bbox.bottom() {
                bbox_sides.push((b, a));
                bbox_sides.push((a, d));
            } else {
                bbox_sides.push((b, a));
            }
        } else if position.x > bbox.right() {
            if position.y > bbox.top() {
                bbox_sides.push((c, b));
                bbox_sides.push((d, c));
            } else if position.y < bbox.bottom() {
                bbox_sides.push((a, d));
                bbox_sides.push((d, c));
            } else {
                bbox_sides.push((d, c));
            }
        } else {
            if position.y > bbox.top() {
                bbox_sides.push((c, b));
            } else if position.y < bbox.bottom() {
                bbox_sides.push((a, d));
            } else {
                return true;
            }
        }

        for (v1, v2) in bbox_sides {
            let delta1 = Vector2::<f32>::from(&(v1 - position));
            let delta2 = Vector2::<f32>::from(&(v2 - position));
            let angle1 = degrees(delta1.y.atan2(delta1.x));
            let angle2 = degrees(delta2.y.atan2(delta2.x));

            let span = normalize_degrees(angle1 - angle2);

            let angle1 = angle1 - actor.angle() as f32;
            let span1 = normalize_degrees(angle1 + self.half_fov);
            if span1 > self.fov {
                if span1 >= span + self.fov {
                    continue;
                }
            }
            return true;
        }
        return false;
    }

    pub fn is_segment_in_frustum<T: Sized + Copy + NumCast + Default>(
        &self,
        actor: &dyn Actor,
        vertex1: &Vector2<T>,
        vertex2: &Vector2<T>,
    ) -> bool {
        let fv1 = Vector2::<f32>::from(&vertex1);
        let fv2 = Vector2::<f32>::from(&vertex2);
        let fpos = Vector2::<f32>::from(&actor.position());
        let fangle = actor.angle() as f32;
        let mut segment_angle1 = normalize_degrees(degrees(angle(&fpos, &fv1)));
        let mut segment_angle2 = normalize_degrees(degrees(angle(&fpos, &fv2)));
        // Span 0
        let span0 = normalize_degrees(segment_angle1 - segment_angle2);
        // Test 1
        if span0 >= 180.0 {
            return false;
        }

        segment_angle1 -= fangle;
        segment_angle2 -= fangle;

        // Span 1
        let span1 = normalize_degrees(segment_angle1 + self.half_fov);
        if span1 > self.fov && span1 >= span0 + self.fov {
            return false;
        }

        // Span 2
        let span2 = normalize_degrees(self.half_fov - segment_angle2);
        if span2 > self.fov && span2 >= span0 + self.fov {
            return false;
        }

        // End
        return true;
    }

    pub fn clip_segment_in_frustum<T: Sized + Copy + NumCast + Default>(
        &self,
        actor: &dyn Actor,
        vertex1: &Vector2<T>,
        vertex2: &Vector2<T>,
    ) -> Option<(u32, u32, f32)> {
        let fv1 = Vector2::<f32>::from(&vertex1);
        let fv2 = Vector2::<f32>::from(&vertex2);
        let fpos = actor.position();
        let fangle = actor.angle();
        let mut segment_angle1 = degrees(angle(&fpos, &fv1));
        let mut segment_angle2 = degrees(angle(&fpos, &fv2));
        // Save wall angle
        let wall_angle1 = segment_angle1;
        // Span 0
        let span0: f32 = normalize_degrees(segment_angle1 - segment_angle2);
        // Test 1
        if span0 >= 180.0 {
            return None;
        }
        // Compute the angles
        segment_angle1 -= fangle;
        segment_angle2 -= fangle;
        // Span 1
        let span1 = normalize_degrees(segment_angle1 + self.half_fov);
        if span1 > self.fov {
            if span1 >= span0 + self.fov {
                return None;
            }
            // Clip other side
            segment_angle1 = self.half_fov;
        }

        // Span 2
        let span2 = normalize_degrees(self.half_fov - segment_angle2);
        if span2 > self.fov {
            if span2 >= span0 + self.fov {
                return None;
            }
            // Clip other side
            segment_angle2 = -self.half_fov;
        }

        // End
        return Some((
            self.angle_to_x(segment_angle1),
            self.angle_to_x(segment_angle2),
            wall_angle1,
        ));
    }
}
