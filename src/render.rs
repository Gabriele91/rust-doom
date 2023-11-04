// Using engine
use crate::window::DoomSurface;

// Trait
pub trait Render {
    fn draw(&self, surface: &mut DoomSurface);
}

pub mod Render2D {
    use pixels::wgpu::Surface;

    // Use engine
    use crate::map::Map;
    use crate::map::Vertex;
    use crate::math;
    use crate::math::Vec2;
    use crate::window::DoomSurface;

    // Render 2D map
    pub struct RenderMap<'wad> {
        map: Box<Map<'wad>>,
        vertices: Vec<Vec2<i32>>,
    }

    impl<'wad> RenderMap<'wad> {
        pub fn new(map: &Box<Map<'wad>>, size: Vec2<i32>, offset: Vec2<i32>) -> Self {
            RenderMap {
                map: map.clone(),
                vertices: RenderMap::remap_all_vertices(&map.vertices, size, offset),
            }
        }

        fn remap_all_vertices(
            map_vertices: &Vec<&'wad Vertex>,
            size: Vec2<i32>,
            offset: Vec2<i32>,
        ) -> Vec<Vec2<i32>> {
            let mut vertices = vec![];
            let bounds = RenderMap::<'wad>::bound_from_vertices(&map_vertices);
            for vertex in map_vertices {
                vertices.push(RenderMap::<'wad>::remap_vertex(
                    &bounds, &vertex, &offset, &size,
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
            bounds: &[Vec2<i16>; 2],
            vertex: &Vertex,
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
        fn draw(&self, surface: &mut DoomSurface) {
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
        }
    }
}
