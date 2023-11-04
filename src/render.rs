// Using engine
use crate::window::DoomSurface;

// Trait
pub trait Render {
    fn draw(&self, surface: &mut DoomSurface);
}

pub mod Render2D {
    // Use engine
    use crate::map::Map;
    use crate::map::Vertex;
    use crate::math;
    use crate::math::Vec2;
    use crate::window::DoomSurface;

    // Render 2D map
    pub struct RenderMap<'wad>{
        map: Box<Map<'wad>>,
        bound: [Vec2<i16>; 2], 
        padding: Vec2<i32>
    }

    impl<'wad> RenderMap<'wad> {
        pub fn new(map: &Box<Map<'wad>>) -> Self {
            RenderMap {
                map: map.clone(),
                bound: RenderMap::bound_from_vertices(&map.vertices),
                padding: Vec2::<i32>::zeros()
            }
        }

        pub fn new_with_padding(map: &Box<Map<'wad>>, padding: Vec2<i32>) -> Self {
            RenderMap {
                map: map.clone(),
                bound: RenderMap::bound_from_vertices(&map.vertices),
                padding: padding
            }
        }

        pub fn remap(&self, vertex: &Vertex, surf_min: &Vec2<i32>, surf_max: &Vec2<i32>) -> Vec2<i32> {
            Vec2 {
                x: (((vertex.x - self.bound[0].x) as i32 * (surf_max.x - surf_min.x)) as f32 / (self.bound[1].x - self.bound[0].x) as f32) as i32 + surf_min.x,
                y: (((vertex.y - self.bound[0].y) as i32 * (surf_max.y - surf_min.y)) as f32 / (self.bound[1].y - self.bound[0].y) as f32) as i32 + surf_min.y,
            }
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
            return [
                bound_min,
                bound_max
            ];
        }
    }

    impl crate::render::Render for RenderMap<'_> {
        fn draw(&self, surface: &mut DoomSurface){
            let extend = surface.pixels.texture().size();
            let size = Vec2::new(extend.width, extend.height).as_vec::<i32>() - self.padding;
            let offset = &self.padding;
            for vertex in &self.map.vertices {
                let position = self.remap(vertex, &offset, &size);
                surface.draw(&position.as_vec::<usize>(), &[0xFF, 0xFF, 0xFF, 0xFF]);
            }
        }
    }
}