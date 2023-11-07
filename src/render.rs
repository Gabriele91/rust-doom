// Using engine
use crate::doom::Doom;
use crate::window::DoomSurface;

// Trait
pub trait Render {
    fn draw<'wad>(&mut self, doom: &mut Doom<'wad>);
}

pub mod render_2d {
    use crate::doom::Doom;
    // Use engine
    use crate::map::{Map, Vertex};
    use crate::math;
    use crate::math::Vector2;

    mod utils {
        use crate::math;
        use crate::map::Vertex;
        use crate::math::Vector2;

        pub fn bound_from_vertices<'wad>(vertices: &Vec<&'wad Vertex>) -> [Vector2<i16>; 2] {
            let mut bound_min = Vector2::new(std::i16::MAX, std::i16::MAX);
            let mut bound_max = Vector2::new(std::i16::MIN, std::i16::MIN);
            for vertex in vertices {
                bound_min.x = math::min(bound_min.x, vertex.x);
                bound_min.y = math::min(bound_min.y, vertex.y);
                bound_max.x = math::max(bound_max.x, vertex.x);
                bound_max.y = math::max(bound_max.y, vertex.y);
            }
            return [bound_min, bound_max];
        }

        pub fn remap_vertex(
            vertex: &Vector2<i16>,
            bounds: &[Vector2<i16>; 2],
            surf_min: &Vector2<i32>,
            surf_max: &Vector2<i32>,
        ) -> Vector2<i32> {
            Vector2 {
                x: (((vertex.x - bounds[0].x) as i32 * (surf_max.x - surf_min.x)) as f32
                    / (bounds[1].x - bounds[0].x) as f32) as i32
                    + surf_min.x,
                y: (((vertex.y - bounds[0].y) as i32 * (surf_max.y - surf_min.y)) as f32
                    / (bounds[1].y - bounds[0].y) as f32) as i32
                    + surf_min.y,
            }
        }
    }

    // Render 2D map
    #[derive(Clone)]
    pub struct RenderMap<'wad> {
        map: Box<Map<'wad>>,
        bounds: [Vector2<i16>; 2],
        size: Vector2<i32>,
        offset: Vector2<i32>,
        vertices: Vec<Vector2<i32>>,
    }

    impl<'wad> RenderMap<'wad> {
        pub fn new(map: &Box<Map<'wad>>, size: Vector2<i32>, offset: Vector2<i32>) -> Self {
            let bounds = utils::bound_from_vertices(&map.vertices);
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
            bounds: &[Vector2<i16>; 2],
            size: &Vector2<i32>,
            offset: &Vector2<i32>,
        ) -> Vec<Vector2<i32>> {
            let mut vertices = vec![];
            for vertex in map_vertices {
                vertices.push(utils::remap_vertex(
                    &vertex, &bounds, &offset, &size,
                ));
            }
            return vertices;
        }

        fn bound_from_vertices(vertices: &Vec<&'wad Vertex>) -> [Vector2<i16>; 2] {
            let mut bound_min = Vector2::new(std::i16::MAX, std::i16::MAX);
            let mut bound_max = Vector2::new(std::i16::MIN, std::i16::MIN);
            for vertex in vertices {
                bound_min.x = math::min(bound_min.x, vertex.x);
                bound_min.y = math::min(bound_min.y, vertex.y);
                bound_max.x = math::max(bound_max.x, vertex.x);
                bound_max.y = math::max(bound_max.y, vertex.y);
            }
            return [bound_min, bound_max];
        }
    }

    impl crate::render::Render for RenderMap<'_> {
        fn draw<'wad>(&mut self, doom: &mut Doom<'wad>) {
            // Draw lines
            for line_def in &self.map.line_defs {
                // draw point
                doom.surface.draw_line(
                    &self.vertices[line_def.start_vertex_id as usize],
                    &self.vertices[line_def.end_vertex_id as usize],
                    &[0xFF, 0xA5, 0x00, 0xFF],
                );
            }
            // Draw screen points
            for vertex in &self.vertices {
                // draw point
                doom.surface.draw(&Vector2::<usize>::from(&vertex), &[0xFF, 0xFF, 0xFF, 0xFF]);
            }
            // Draw player 1
            match doom.actors.iter().find(|&actor| actor.borrow().type_id() == 1) {
                Some(actor) => {
                    let player_position = utils::remap_vertex(
                        &actor.borrow().position(), 
                        &self.bounds, 
                        &self.offset, 
                        &self.size
                    );
                    doom.surface.draw(&Vector2::<usize>::from(&player_position), &[0x00, 0x00, 0xFF, 0xFF]);
                },
                None => ()
            } 
        }
    }

    // Render 2D bsp
    #[derive(Clone)]
    pub struct RenderBSP<'wad> {
        map: Box<Map<'wad>>,
        bounds: [Vector2<i16>; 2],
        size: Vector2<i32>,
        offset: Vector2<i32>,
    }

    impl<'wad> RenderBSP<'wad> {
        pub fn new(map: &Box<Map<'wad>>, size: Vector2<i32>, offset: Vector2<i32>) -> Self {
            let bounds = utils::bound_from_vertices(&map.vertices);
            RenderBSP {
                map: map.clone(),
                bounds: bounds,
                size: size,
                offset: offset
            }
        }
    }

    impl crate::render::Render for RenderBSP<'_> {
        fn draw<'wad>(&mut self, doom: &mut Doom<'wad>) {
            // Ref to bsp
            let bsp = &doom.bsp;
            let render = &self;
            // Draw player 1
            match doom.actors.iter().find(|&actor| actor.borrow().type_id() == 1) {
                Some(actor) => {
                    bsp.visit_debug(&actor.borrow().position(), 
                    |_id|{},
                    |id| {
                        let node = self.map.nodes[id as usize];
                        let top = math::max(node.left_box.top, node.right_box.top);
                        let bottom = math::min(node.left_box.bottom, node.right_box.bottom);
                        let left = math::min(node.left_box.left, node.right_box.left);
                        let right = math::max(node.left_box.right, node.right_box.right);
                        let topleft = utils::remap_vertex(
                            &Vector2::new(left, top), 
                            &render.bounds, 
                            &render.offset, 
                            &render.size
                        );
                        let bottomright = utils::remap_vertex(
                            &Vector2::new(right, bottom), 
                            &render.bounds, 
                            &render.offset, 
                            &render.size
                        );
                        doom.surface.draw_box(&topleft, &bottomright, &[0xFF, 0x00, 0x00, 0xFF]);
                    });
                },
                None => ()
            } 
        }
    }



}
