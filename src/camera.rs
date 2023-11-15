use num_traits::NumCast;
use crate::actors::Actor;
use crate::map::NodeBox;
use crate::math::{angle, degrees, normalize_degrees, radians, Vector2};
use crate::shape::Rectangle;

#[derive(Clone)]
pub struct Camera {
    pub fov: f32,
    half_fov: f32,
    half_width: u32,
    screen_dist: f32,
    tan_angle_360: Vec<f32>,
}

impl Camera {
    pub fn new(fov: f32, width: u32) -> Self {
        let half_fov = fov / 2.0;
        let half_width = width / 2;
        let screen_dist = half_width as f32 / radians(fov / 2.0).tan();
        Camera {
            fov,
            half_fov,
            half_width,
            screen_dist,
            tan_angle_360: {
                let mut atan_angle_360 = vec![];
                atan_angle_360.reserve(360);
                for angle in 0..360 {
                    atan_angle_360.push(radians(angle as f32).tan());
                }
                atan_angle_360
            },
        }
    }

    pub fn angle_to_x(&self, angle: u16) -> u32 {
        if angle > 0 {
            (self.screen_dist - (self.tan_angle_360[angle as usize] * self.half_width as f32))
                as u32
        } else {
            ((-self.tan_angle_360[angle as usize] * self.half_width as f32) + self.screen_dist)
                as u32
        }
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

        let position = actor.position();
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
            let delta1 = Vector2::<f32>::from(&(v1 - *position));
            let delta2 = Vector2::<f32>::from(&(v2 - *position));
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
        let fpos = Vector2::<f32>::from(&actor.position());
        let fangle = actor.angle() as f32;
        let mut segment_angle1 = normalize_degrees(degrees(angle(&fpos, &fv1)));
        let mut segment_angle2 = normalize_degrees(degrees(angle(&fpos, &fv2)));
        // Save raw angle
        let raw_angle1 = segment_angle1;
        // Span 0
        let span0 = normalize_degrees(segment_angle1 - segment_angle2);
        // Test 1
        if span0 >= 180.0 {
            return None;
        }

        segment_angle1 -= fangle;
        segment_angle2 -= fangle;

        // Span 1
        let span1 = normalize_degrees(segment_angle1 + self.half_fov);
        if span1 > self.fov {
            if span1 >= span0 + self.fov {
                return None;
            }
            // Clip other side
            segment_angle1 = self.fov;
        }

        // Span 2
        let span2 = normalize_degrees(self.half_fov - segment_angle2);
        if span2 > self.fov {
            if span2 >= span0 + self.fov {
                return None;
            }
            // Clip other side
            segment_angle2 = self.fov;
        }

        // End
        return Some((
            self.angle_to_x(normalize_degrees(segment_angle1) as u16),
            self.angle_to_x(normalize_degrees(segment_angle2) as u16),
            raw_angle1,
        ));
    }
}
