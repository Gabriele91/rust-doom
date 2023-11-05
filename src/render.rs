// Using engine
use crate::actors::Actor;
use crate::window::DoomSurface;

// Trait
pub trait Render {
    fn draw(&self, surface: &mut DoomSurface, actors: &Vec<Box<dyn Actor>>);
}

pub mod render_2d {
    // Use engine
    use crate::map::Map;
    use crate::map::Vertex;
    use crate::math;
    use crate::math::Vec2;
    use crate::window::DoomSurface;
    use crate::actors::Actor;

    // Render 2D map
    pub struct RenderMap<'wad> {
        map: Box<Map<'wad>>,
        bounds: [Vec2<i16>; 2],
        size: Vec2<i32>,
        offset: Vec2<i32>,
        vertices: Vec<Vec2<i32>>,
    }

    impl<'wad> RenderMap<'wad> {
        pub fn new(map: &Box<Map<'wad>>, size: Vec2<i32>, offset: Vec2<i32>) -> Self {
            let bounds = RenderMap::<'wad>::bound_from_vertices(&map.vertices);
            let vertices = RenderMap::remap_all_vertices(&map.vertices, &bounds, &size, &offset);
            RenderMap {
                map: map.clone(),
                bounds: bounds,
                size: size,
                offset: offset,
                vertices: vertices,
            }
        }

        fn remap_all_vertices(
            map_vertices: &Vec<&'wad Vertex>,
            bounds: &[Vec2<i16>; 2],
            size: &Vec2<i32>,
            offset: &Vec2<i32>,
        ) -> Vec<Vec2<i32>> {
            let mut vertices = vec![];
            for vertex in map_vertices {
                vertices.push(RenderMap::<'wad>::remap_vertex(
                    &vertex, &bounds, &offset, &size,
                ));
            }
            return vertices;
        }

        fn bound_from_vertices(vertices: &Vec<&'wad Vertex>) -> [Vec2<i16>; 2] {
            let mut bound_min = Vec2::new(std::i16::MAX, std::i16::MAX);
            let mut bound_max = Vec2::new(std::i16::MIN, std::i16::MIN);
            for vertex in vertices {
                bound_min.x = math::min(bound_min.x, vertex.x);
                bound_min.y = math::min(bound_min.y, vertex.y);
                bound_max.x = math::max(bound_max.x, vertex.x);
                bound_max.y = math::max(bound_max.y, vertex.y);
            }
            return [bound_min, bound_max];
        }

        fn remap_vertex(
            vertex: &Vertex,
            bounds: &[Vec2<i16>; 2],
            surf_min: &Vec2<i32>,
            surf_max: &Vec2<i32>,
        ) -> Vec2<i32> {
            Vec2 {
                x: (((vertex.x - bounds[0].x) as i32 * (surf_max.x - surf_min.x)) as f32
                    / (bounds[1].x - bounds[0].x) as f32) as i32
                    + surf_min.x,
                y: (((vertex.y - bounds[0].y) as i32 * (surf_max.y - surf_min.y)) as f32
                    / (bounds[1].y - bounds[0].y) as f32) as i32
                    + surf_min.y,
            }
        }
    }

    impl crate::render::Render for RenderMap<'_> {
        fn draw(&self, surface: &mut DoomSurface, actors: &Vec<Box<dyn Actor>>) {
            // Draw lines
            for line_def in &self.map.line_defs {
                // draw point
                surface.draw_line(
                    &self.vertices[line_def.start_vertex_id as usize],
                    &self.vertices[line_def.end_vertex_id as usize],
                    &[0xFF, 0xA5, 0x00, 0xFF],
                );
            }
            // Draw screen points
            for vertex in &self.vertices {
                // draw point
                surface.draw(&vertex.as_vec::<usize>(), &[0xFF, 0xFF, 0xFF, 0xFF]);
            }
            // Draw player 1
            match actors.iter().find(|&actor| actor.type_id() == 1) {
                Some(actor) => {
                    let player_position = RenderMap::remap_vertex(
                        &actor.position().as_vec::<i16>(), 
                        &self.bounds, 
                        &self.offset, 
                        &self.size
                    );
                    surface.draw(&player_position.as_vec::<usize>(), &[0x00, 0x00, 0xFF, 0xFF]);
                },
                None => ()
            } 
        }
    }
}
