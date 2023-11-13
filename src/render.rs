// Using engine
use crate::doom::Doom;
// Trait
pub trait Render {
    fn draw<'wad>(&mut self, doom: &mut Doom<'wad>);
}

pub mod render_2d {
    use crate::doom::Doom;
    // Use engine
    use crate::map::{Map, Vertex, NodeBox};
    use crate::math;
    use crate::math::Vector2;
    use crate::window::DoomSurface;

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
            // Ref
            let surface = &mut doom.surface.borrow_mut();
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
                surface.draw(&Vector2::<usize>::from(&vertex), &[0xFF, 0xFF, 0xFF, 0xFF]);
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
                    surface.draw(&Vector2::<usize>::from(&player_position), &[0x00, 0x00, 0xFF, 0xFF]);
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
        
        fn draw_node_box(&self, surface: &mut DoomSurface, node_box: &NodeBox, color: &[u8]){
            let topleft = utils::remap_vertex(
                &node_box.zx(), 
                &self.bounds, 
                &self.offset, 
                &self.size
            );
            let bottomright = utils::remap_vertex(
                &node_box.wy(), 
                &self.bounds, 
                &self.offset, 
                &self.size
            );
            surface.draw_box(&topleft, &bottomright, color);
        }

        fn draw_line(&self, surface: &mut DoomSurface, v1: &Vector2<i16>, v2: &Vector2<i16>, color: &[u8]){
            let remapv1 = utils::remap_vertex(
                &v1, 
                &self.bounds, 
                &self.offset, 
                &self.size
            );
            let remapv2 = utils::remap_vertex(
                &v2, 
                &self.bounds, 
                &self.offset, 
                &self.size
            );
            surface.draw_line(&remapv1, &remapv2, color);
        }
    }

    impl crate::render::Render for RenderBSP<'_> {
        fn draw<'wad>(&mut self, doom: &mut Doom<'wad>) {
            // Ref to bsp
            let bsp = &doom.bsp;
            let surface = doom.surface.clone();
            let render = &self;
            // Draw player 1
            match doom.actors.iter().find(|&actor| actor.borrow().type_id() == 1) {
                Some(actor) => {
                    bsp.visit_debug(&actor.borrow().position(), 
                    |subsector_id|{
                        let subsector = doom.map.sub_sectors[subsector_id as usize];
                        for sector_id in subsector.iter() {
                            let seg = doom.map.sectors[sector_id as usize];
                            let vertex1 = doom.map.vertices[seg.start_vertex_id as usize];
                            let vertex2 = doom.map.vertices[seg.end_vertex_id as usize];
                            render.draw_line(&mut surface.borrow_mut(), &vertex1, &vertex2, &[0x00,0x00, 0xFF, 0xFF]);
                        }
                    },
                    |id| {
                        let node = self.map.nodes[id as usize];
                        let left_box = node.left_box;
                        let right_box = node.right_box;
                        render.draw_node_box(&mut surface.borrow_mut(), &left_box, &[0xFF,0x00, 0x00, 0xFF]);
                        render.draw_node_box(&mut surface.borrow_mut(), &right_box, &[0x00,0xFF, 0x00, 0xFF]);
                    },
                    |_id|{ });
                },
                None => ()
            } 
        }

    }



}
