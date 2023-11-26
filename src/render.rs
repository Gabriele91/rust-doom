#![allow(dead_code)]
// Using engine
use crate::doom::Doom;
// Trait
pub trait Render {
    fn draw<'wad>(&mut self, doom: &mut Doom<'wad>, last_frame_time: f64, blending_factor: f64);
}

pub mod render_2d {
    use std::rc::Rc;
    // Use engine
    use crate::map::{Map, Vertex, NodeBox};
    use crate::{math, configure};
    use crate::math::Vector2;
    use crate::shape::Size;
    use crate::window::DoomSurface;
    use crate::camera::Camera;
    use crate::doom::Doom;
    use crate::data_textures::{DataTextures, Texture};

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
        map: Rc<Map<'wad>>,
        bounds: [Vector2<i16>; 2],
        size: Vector2<i32>,
        offset: Vector2<i32>,
        vertices: Vec<Vector2<i32>>,
    }

    impl<'wad> RenderMap<'wad> {
        pub fn new(map: &Rc<Map<'wad>>, size: Vector2<i32>, offset: Vector2<i32>) -> Self {
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
        fn draw<'wad>(&mut self, doom: &mut Doom<'wad>, _last_frame_time: f64, _blending_factor: f64) {
            // Ref
            let surface = &mut doom.surface.borrow_mut();
            // Draw lines
            for line_def in &self.map.line_defs {
                // draw point
                surface.draw_line_lb(
                    &self.vertices[line_def.start_vertex_id as usize],
                    &self.vertices[line_def.end_vertex_id as usize],
                    &[0xFF, 0xA5, 0x00, 0xFF],
                );
            }
            // Draw screen points
            for vertex in &self.vertices {
                // draw point
                surface.draw_lb(&Vector2::<usize>::from(&vertex), &[0xFF, 0xFF, 0xFF, 0xFF]);
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
                    surface.draw_lb(&Vector2::<usize>::from(&player_position), &[0x00, 0x00, 0xFF, 0xFF]);
                },
                None => ()
            } 
        }
    }

    // Render 2D bsp
    #[derive(Clone)]
    pub struct RenderBSP<'wad> {
        map: Rc<Map<'wad>>,
        bounds: [Vector2<i16>; 2],
        size: Vector2<i32>,
        offset: Vector2<i32>,
    }

    impl<'wad> RenderBSP<'wad> {
        pub fn new(map: &Rc<Map<'wad>>, size: Vector2<i32>, offset: Vector2<i32>) -> Self {
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
            surface.draw_box_lb(&topleft, &bottomright, color);
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
            surface.draw_line_lb(&remapv1, &remapv2, color);
        }
    }

    impl crate::render::Render for RenderBSP<'_> {
        fn draw<'wad>(&mut self, doom: &mut Doom<'wad>, _last_frame_time: f64, _blending_factor: f64) {
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
                            let seg = doom.map.segs[sector_id as usize];
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
    // Render 2D bsp
    
    #[derive(Clone)]
    pub struct RenderCamera<'wad> {
        map: Rc<Map<'wad>>,
        bounds: [Vector2<i16>; 2],
        size: Vector2<i32>,
        offset: Vector2<i32>,
        camera: Camera
    }

    impl<'wad> RenderCamera<'wad> {
        pub fn new(map: &Rc<Map<'wad>>, size: Vector2<i32>, offset: Vector2<i32>, configure: &configure::Camera) -> Self {
            let bounds = utils::bound_from_vertices(&map.vertices);
            RenderCamera {
                map: map.clone(),
                bounds: bounds,
                size: size,
                offset: offset,
                camera: Camera::new(configure.fov, size.width().try_into().unwrap()),
            }
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
            surface.draw_line_lb(&remapv1, &remapv2, color);
        }
    }

    impl crate::render::Render for RenderCamera<'_> {
        fn draw<'wad>(&mut self, doom: &mut Doom<'wad>, _last_frame_time: f64, _blending_factor: f64) {
            // Ref to bsp
            let bsp = &mut doom.bsp;
            let surface = doom.surface.clone();
            let render = self;
            // Draw player 1
            match doom.actors.iter().find(|&actor| actor.borrow().type_id() == 1) {
                Some(actor) => {
                    bsp.visit(
                     &actor.borrow().position(),
                     render, 
                     |subsector_id, render| -> bool {
                        let subsector = render.map.sub_sectors[subsector_id as usize];
                        for sector_id in subsector.iter() {
                            let seg = render.map.segs[sector_id as usize];
                            let vertex1 = render.map.vertices[seg.start_vertex_id as usize];
                            let vertex2 = render.map.vertices[seg.end_vertex_id as usize];
                            if render.camera.is_segment_in_frustum(actor.borrow().as_ref(), &vertex1, &vertex2) {
                                render.draw_line(&mut surface.borrow_mut(), &vertex1, &vertex2, &[0x00,0x00, 0xFF, 0xFF]);
                            }
                        }
                        let position = Vector2::<f32>::from(&actor.borrow().position());
                        let angle_left = math::radians(actor.borrow().angle() as f32 + render.camera.half_fov);
                        let angle_right = math::radians(actor.borrow().angle() as f32 - render.camera.half_fov);
                        let left = position + Vector2::new(angle_left.cos(), angle_left.sin()) * (render.size.height() as f32) * 8.0;
                        let right = position + Vector2::new(angle_right.cos(), angle_right.sin()) * (render.size.height() as f32) * 8.0;
                        render.draw_line(
                            &mut surface.borrow_mut(),
                            &Vector2::<i16>::from(&position), 
                            &Vector2::<i16>::from(&left), 
                            &[0xFF,0xFF, 0xFF, 0xFF]
                        );
                        render.draw_line(
                            &mut surface.borrow_mut(),
                            &Vector2::<i16>::from(&position), 
                            &Vector2::<i16>::from(&right), 
                            &[0xFF,0xFF, 0xFF, 0xFF]
                        );
                        return true;
                    },|node_box, render| { 
                        render.camera.is_box_in_frustum(actor.borrow().as_ref(), &node_box)
                    });
                },
                None => ()
            } 
        }

    }

        // Render 2D map
    #[derive(Clone)]
    pub struct RenderFlats<'wad> {
        data_textures: Rc<DataTextures<'wad>>,
        flat_id: usize,
        flat_update: f64,
        size: Vector2<i32>,
        offset: Vector2<i32>,
    }

    impl<'wad> RenderFlats<'wad> {
        pub fn new(data_textures:&Rc<DataTextures<'wad>>, size: Vector2<i32>, offset: Vector2<i32>) -> Self {
            RenderFlats {
                data_textures: data_textures.clone(),
                flat_id: 0,
                flat_update: 0.0,
                size: size,
                offset: offset,
            }
        }
        fn draw_texture(&self, surface: &mut DoomSurface, texture: &Texture<3>) {
            let start_y = self.offset.y as usize;
            let end_y = ((self.offset.y + self.size.y) as usize).min(texture.size.height() as usize);
            
            let start_x = self.offset.x as usize;
            let end_x = ((self.offset.x + self.size.x) as usize).min(texture.size.width() as usize);
            
            for y in start_y..end_y {
                for x in start_x..end_x {
                    let texture_x = x - start_x;
                    let texture_y = y - start_y;
                    surface.draw_lt(
                        &Vector2::new(x, y), 
                        &texture.colors[texture_y * texture.size.width() as usize + texture_x]
                    );
                }
            }
        }
     }

     impl crate::render::Render for RenderFlats<'_> {
        fn draw<'wad>(&mut self, doom: &mut Doom<'wad>, last_frame_time: f64, _blending_factor: f64) {
            // Test
            if self.data_textures.flats.is_empty() {
                return;
            }
            // Update
            self.flat_update += last_frame_time;
            // Change Texture
            if self.flat_update >= 1.0  {
                self.flat_id += 1;
                self.flat_update = 0.0;
                if  self.flat_id >= self.data_textures.flats.len() {
                    self.flat_id = 0;
                }
            }
            // Draw
            self.draw_texture(&mut doom.surface.borrow_mut(), &self.data_textures.flats[self.flat_id]);
        }
    }

}

pub mod render_3d {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};
    use std::rc::Rc;
    use crate::actors::Actor;
    // Use engine
    use crate::camera::Camera;
    use crate::{configure, math};
    use crate::doom::Doom;
    use crate::map::{Map, Seg};
    use crate::math::Vector2;
    use crate::shape::Size;
    use crate::window::DoomSurface;

    mod consts {
        pub const VOID_TEXTURE : [u8; 8] = ['-' as u8,0,0,0, 0,0,0,0];
        pub const MAX_SCALE : f32  = 64.0;
        pub const MIN_SCALE : f32 = 0.00390625;
    }

    enum WallType<'a> {
        SolidWall(&'a Seg),
        PortalWall(&'a Seg)
    }

    // Render 3D bsp
    #[derive(Clone)]
    pub struct RenderSoftware<'wad> {
        map: Rc<Map<'wad>>,
        size: Vector2<i32>,
        offset: Vector2<i32>,
        camera: Camera,
        screen_range: Vec<bool>,
        upper_clip: Box<Vec<i32>>,
        lower_clip: Box<Vec<i32>>,
    }


    impl<'wad> RenderSoftware<'wad> {
        pub fn new(map: &Rc<Map<'wad>>, size: Vector2<i32>, offset: Vector2<i32>, configure: &configure::Camera) -> Self {
            RenderSoftware {
                map: map.clone(),
                size: size,
                offset: offset,
                camera: Camera::new(configure.fov, size.width().try_into().unwrap()),
                screen_range: vec![false; size.width() as usize],
                upper_clip: Box::new(vec![0; size.width() as usize]),
                lower_clip: Box::new(vec![size.height(); size.width() as usize]),
            }
        }
        
        fn reset(&mut self) {
            self.screen_range.fill(true);
            self.upper_clip.fill(0);
            self.lower_clip.fill(self.size.height());
        }

        fn name_to_color(array: &[u8; 8], mut light_level: &f32) -> [u8; 4] {
            light_level = math::clamp(light_level, &0.1, &1.0);
            if  *array == consts::VOID_TEXTURE {
                return 
                [
                    (255.0 * light_level) as u8,
                    (255.0 * light_level) as u8,
                    (255.0 * light_level) as u8,
                    0xFF
                ]
            }
            let mut hasher = DefaultHasher::new();
            array.hash(&mut hasher);
            let hash = hasher.finish();
            let r = math::clamp((hash >> 16 & 0xFF) as f32, 32.0, 255.0);
            let g = math::clamp((hash >>  8 & 0xFF) as f32, 32.0, 255.0);
            let b = math::clamp((hash >>  0 & 0xFF) as f32, 32.0, 255.0);
            return 
            [
                (r * light_level) as u8,
                (g * light_level) as u8,
                (b * light_level) as u8,
                0xFF
            ]
        }

        fn classify_segment(&self, seg: &'wad Seg, start: u32, end: u32) -> Option<WallType<'wad>> {
            if start == end {
                return None;
            }

            // Right is mandatory
            let right_sector = seg.right_sector(&self.map)?;
            
            // Left only if it is a portal
            if let Some(left_sector) = seg.left_sector(&self.map) {
                
                // Wall with window
                if right_sector.floor_height != left_sector.floor_height 
                || right_sector.ceiling_height != left_sector.ceiling_height {
                   return Some(WallType::PortalWall(&seg));
                }

                // Reject empty lines used for triggers and special events.
                // identical floor and ceiling on both sides, identical
                // light levels on both sides, and no middle texture.
                if right_sector.ceiling_texture == left_sector.ceiling_texture 
                && right_sector.floor_texture == left_sector.floor_texture
                && right_sector.light_level == left_sector.light_level
                && seg.line_defs(&self.map).right_side(&self.map)?.middle_texture == consts::VOID_TEXTURE {
                    return None;
                }

                // Borders with different light levels and/or textures
                return Some(WallType::PortalWall(&seg));

            } else {
                return Some(WallType::SolidWall(&seg));
            }
        }

        fn draw_line(&self, surface: &mut DoomSurface, x: i32, mut y1: i32, mut y2: i32, color: &[u8]) {
            y1 = math::clamp(y1, 0, self.size.y);
            y2 = math::clamp(y2, 0, self.size.y);
            if x < self.size.x && y1 < y2 {
                let start = Vector2::new(x as i32, y1) + self.offset;
                let end = Vector2::new(x as i32, y2) + self.offset;
                surface.draw_line_lt(&start, &end, &color);
            }
        }

        fn draw_wall(&mut self, actor: &Box<dyn Actor>, surface: &mut DoomSurface, wtype: &WallType<'wad>, start: u32, end: u32, wall_angle: f32) {
            match wtype {
                WallType::SolidWall(ref_seg) => {
                    // Alias
                    let seg = *ref_seg;
                    let line = seg.line_defs(&self.map);
                    let side = line.right_side(&self.map).unwrap();
                    let sector = seg.right_sector(&self.map).unwrap();
                    let angle = actor.angle() as f32;
                    let position = Vector2::<f32>::from( actor.position() );
                    let start_vertex = Vector2::<f32>::from( seg.start_vertex(&self.map) );
                    let half_height = self.size.height() as f32 / 2.0;
                    // Texture
                    let wall_texture = side.middle_texture;
                    let floor_texture = sector.floor_texture;
                    let ceiling_texture = sector.ceiling_texture;
                    let light_level = math::clamp( sector.light_level as f32 / 255.0, 0.0, 1.0);
                    // Height of wall w/ rispect to player
                    let wall_floor = sector.floor_height - *actor.height();
                    let wall_ceiling = sector.ceiling_height - *actor.height();
                    // What to draw
                    let b_draw_wall = side.middle_texture != consts::VOID_TEXTURE;
                    let b_draw_ceiling = wall_ceiling > 0;
                    let b_draw_floor = wall_floor < 0;
                    // Test
                    if !b_draw_wall && !b_draw_ceiling && !b_draw_floor {
                        return;
                    }
                    // Calculate the scaling factors of the left and right edges of the wall range
                    let wall_normal_angle = seg.float_degrees_angle() + 90.0;
                    let offset_angle = wall_normal_angle - wall_angle;
                    let hypotenuse = position.distance(&start_vertex);
                    let wall_distance = hypotenuse * math::radians(offset_angle).cos();
                    // Compute scale
                    let wall_scale_1 = math::clamp(
                        self.camera.scale_from_global_angle(start, wall_normal_angle, wall_distance, angle), 
                        consts::MIN_SCALE, 
                        consts::MAX_SCALE
                    );
                    let wall_scale_step = {
                        if start < end {
                            let wall_scale_2 = math::clamp(
                                self.camera.scale_from_global_angle(end, wall_normal_angle, wall_distance, angle), 
                                consts::MIN_SCALE, 
                                consts::MAX_SCALE
                            );
                            (wall_scale_2 - wall_scale_1) / (end - start) as f32
                        } else {
                            0.0
                        }
                    };
                    // Determine where on the screen the wall is drawn
                    // Top wall
                    let mut wall_y1 = half_height - wall_ceiling as f32 * wall_scale_1;
                    let wall_y1_step = -wall_scale_step * wall_ceiling as f32;
                    // Bottom wall
                    let mut wall_y2 = half_height - wall_floor as f32 * wall_scale_1;
                    let wall_y2_step = -wall_scale_step * wall_floor as f32;

                    // Draw
                    for x in start..end {
                        let draw_wall_y1 = wall_y1 as i32;
                        let draw_wall_y2 = wall_y2 as i32;
                        if b_draw_ceiling {
                            let ceiling_wall_y1 = self.upper_clip[x as usize];
                            let ceiling_wall_y2 = math::min(draw_wall_y1, self.lower_clip[x as usize]);
                            self.draw_line(
                                surface, 
                                x as i32, 
                                ceiling_wall_y1, 
                                ceiling_wall_y2, 
                                &RenderSoftware::name_to_color(&ceiling_texture, &light_level)
                            );
                        }
                        if b_draw_wall {
                            let middle_wall_y1 = math::max(draw_wall_y1, self.upper_clip[x as usize]);
                            let middle_wall_y2 = math::min(draw_wall_y2, self.lower_clip[x as usize]);
                            self.draw_line(
                                surface, 
                                x as i32, 
                                middle_wall_y1, 
                                middle_wall_y2, 
                                &RenderSoftware::name_to_color(&wall_texture, &light_level)
                            );
                        }
                        if b_draw_floor {
                            let floor_wall_y1 = math::max(draw_wall_y2, self.upper_clip[x as usize]);
                            let floor_wall_y2 = self.lower_clip[x as usize];
                            self.draw_line(
                                surface, 
                                x as i32, 
                                floor_wall_y1, 
                                floor_wall_y2, 
                                &RenderSoftware::name_to_color(&floor_texture, &light_level)
                            );
                        }
                        // Next step
                        wall_y1 += wall_y1_step;
                        wall_y2 += wall_y2_step;
                    }
                },
                WallType::PortalWall(ref_seg) => {
                    // Alias
                    let seg = *ref_seg;
                    let line = seg.line_defs(&self.map);
                    let side = line.right_side(&self.map).unwrap();
                    let right_sector = seg.right_sector(&self.map).unwrap();
                    let left_sector = seg.left_sector(&self.map).unwrap();
                    let angle = actor.angle() as f32;
                    let position = Vector2::<f32>::from( actor.position() );
                    let start_vertex = Vector2::<f32>::from( seg.start_vertex(&self.map) );
                    let half_height = (self.size.height() / 2) as f32;
                    // Texture
                    let upper_texture = side.upper_texture;
                    let lower_texture = side.lower_texture;
                    let _wall_texture = side.middle_texture;
                    let floor_texture = right_sector.floor_texture;
                    let ceiling_texture = right_sector.ceiling_texture;
                    let light_level = math::clamp( right_sector.light_level as f32 / 255.0, 0.0, 1.0);
                    // Height of wall w/ rispect to player
                    let right_wall_floor = right_sector.floor_height - *actor.height();
                    let right_wall_ceiling = right_sector.ceiling_height - *actor.height();
                    let left_wall_floor = left_sector.floor_height - *actor.height();
                    let left_wall_ceiling = left_sector.ceiling_height - *actor.height();
                    // set what to draw
                    let mut b_draw_upper_wall = false;
                    let mut b_draw_ceiling = false;
                    let mut b_draw_floor = false;
                    let mut b_draw_lower_wall = false;
                    // What to draw
                    if right_wall_ceiling != left_wall_ceiling 
                    || right_sector.light_level != left_sector.light_level 
                    || right_sector.ceiling_texture != left_sector.ceiling_texture {
                        b_draw_upper_wall = upper_texture != consts::VOID_TEXTURE && left_wall_ceiling < right_wall_ceiling;
                        b_draw_ceiling = right_wall_ceiling >= 0;
                    }

                    if right_wall_floor != left_wall_floor 
                    || right_sector.light_level != left_sector.light_level 
                    || right_sector.floor_texture != left_sector.floor_texture {
                        b_draw_lower_wall = lower_texture != consts::VOID_TEXTURE && left_wall_floor > right_wall_floor;
                        b_draw_floor = right_wall_floor <= 0;
                    }
                    // Test
                    if !b_draw_upper_wall && !b_draw_ceiling && !b_draw_floor && !b_draw_lower_wall {
                        return;
                    }
                    // Calculate the scaling factors of the left and right edges of the wall range
                    let wall_normal_angle = seg.float_degrees_angle() + 90.0;
                    let offset_angle = wall_normal_angle - wall_angle;
                    let hypotenuse = position.distance(&start_vertex);
                    let wall_distance = hypotenuse * math::radians(offset_angle).cos();
                    // Compute scale
                    let wall_scale_1 = math::clamp(
                        self.camera.scale_from_global_angle(start, wall_normal_angle, wall_distance, angle), 
                        consts::MIN_SCALE, 
                        consts::MAX_SCALE
                    );
                    let wall_scale_step = {
                        if start < end {
                            let wall_scale_2 = math::clamp(
                                self.camera.scale_from_global_angle(end, wall_normal_angle, wall_distance, angle), 
                                consts::MIN_SCALE, 
                                consts::MAX_SCALE
                            );
                            (wall_scale_2 - wall_scale_1) / (end - start) as f32
                        } else {
                            0.0
                        }
                    };
                    // Determine where on the screen the wall is drawn
                    // Top wall
                    let mut wall_y1 = half_height - right_wall_ceiling as f32 * wall_scale_1;
                    let wall_y1_step = -wall_scale_step * right_wall_ceiling as f32;
                    // Bottom wall
                    let mut wall_y2 = half_height - right_wall_floor as f32 * wall_scale_1;
                    let wall_y2_step = -wall_scale_step * right_wall_floor as f32;
                    // Determinate y for the top and bottom walls
                    let mut portal_y1 = wall_y2;
                    let mut portal_y1_step = wall_y2_step;
                    if b_draw_upper_wall && left_wall_ceiling > right_wall_floor {
                            portal_y1 = half_height - left_wall_ceiling as f32 * wall_scale_1;
                            portal_y1_step = -wall_scale_step * left_wall_ceiling as f32;
                    }
                    let mut portal_y2 = wall_y1;
                    let mut portal_y2_step = wall_y1_step;
                    if b_draw_lower_wall && left_wall_floor < right_wall_ceiling {
                            portal_y2 = half_height - left_wall_floor as f32 * wall_scale_1;
                            portal_y2_step = -wall_scale_step * left_wall_floor as f32;
                    }

                    // Draw
                    for x in start..end {
                        let draw_wall_y1 = wall_y1 as i32;
                        let draw_wall_y2 = wall_y2 as i32;

                        if b_draw_upper_wall {
                            if b_draw_ceiling {
                                let ceiling_wall_y1 = self.upper_clip[x as usize];
                                let ceiling_wall_y2 = math::min(draw_wall_y1, self.lower_clip[x as usize]);
                                self.draw_line(
                                    surface, 
                                    x as i32, 
                                    ceiling_wall_y1, 
                                    ceiling_wall_y2, 
                                    &RenderSoftware::name_to_color(&ceiling_texture, &light_level)
                                );
                            }
                            let draw_upper_wall_y1 = wall_y1 as i32 - 1;
                            let draw_upper_wall_y2 = portal_y1 as i32;

                            let middle_wall_y1 = math::max(draw_upper_wall_y1, self.upper_clip[x as usize]);
                            let middle_wall_y2 = math::min(draw_upper_wall_y2, self.lower_clip[x as usize]);
                            self.draw_line(
                                surface, 
                                x as i32, 
                                middle_wall_y1, 
                                middle_wall_y2, 
                                &RenderSoftware::name_to_color(&upper_texture, &light_level)
                            );
                            if self.upper_clip[x as usize] < middle_wall_y2 {
                                self.upper_clip[x as usize] = middle_wall_y2
                            }
                            portal_y1 += portal_y1_step
                        }

                        if b_draw_ceiling {
                            let ceiling_wall_y1 = self.upper_clip[x as usize];
                            let ceiling_wall_y2 = math::min(draw_wall_y1, self.lower_clip[x as usize]);
                            self.draw_line(
                                surface, 
                                x as i32, 
                                ceiling_wall_y1, 
                                ceiling_wall_y2, 
                                &RenderSoftware::name_to_color(&ceiling_texture, &light_level)
                            );
                            if self.upper_clip[x as usize] < ceiling_wall_y2 {
                                self.upper_clip[x as usize] = ceiling_wall_y2
                            }
                        }

                        if b_draw_lower_wall {
                            if b_draw_floor {
                                let floor_wall_y1 = math::max(draw_wall_y2, self.upper_clip[x as usize]);
                                let floor_wall_y2 = self.lower_clip[x as usize];
                                self.draw_line(
                                    surface, 
                                    x as i32, 
                                    floor_wall_y1, 
                                    floor_wall_y2, 
                                    &RenderSoftware::name_to_color(&floor_texture, &light_level)
                                );
                            }
                            let draw_lower_wall_y1 = portal_y2 as i32 - 1;
                            let draw_lower_wall_y2 = wall_y2 as i32;

                            let middle_wall_y1 =  math::max(draw_lower_wall_y1, self.upper_clip[x as usize]);
                            let middle_wall_y2 = math::min(draw_lower_wall_y2, self.lower_clip[x as usize]);
                            self.draw_line(
                                surface, 
                                x as i32, 
                                middle_wall_y1, 
                                middle_wall_y2, 
                                &RenderSoftware::name_to_color(&lower_texture, &light_level)
                            );
                            if self.lower_clip[x as usize] > middle_wall_y1 {
                                self.lower_clip[x as usize] = middle_wall_y1
                            }
                            portal_y2 += portal_y2_step
                        }
                        
                        if b_draw_floor {
                            let floor_wall_y1 = math::max(draw_wall_y2, self.upper_clip[x as usize]);
                            let floor_wall_y2 = self.lower_clip[x as usize];
                            self.draw_line(
                                surface, 
                                x as i32, 
                                floor_wall_y1, 
                                floor_wall_y2, 
                                &RenderSoftware::name_to_color(&floor_texture, &light_level)
                            );
                            if self.lower_clip[x as usize] > draw_wall_y2 {
                                self.lower_clip[x as usize] = floor_wall_y1;
                            }
                        }
                        // Next step
                        wall_y1 += wall_y1_step;
                        wall_y2 += wall_y2_step;
                    }
                }
            }
        }

        fn draw_clip_walls(&mut self, actor: &Box<dyn Actor>, surface: &mut DoomSurface, wtype: &WallType<'wad>, wall_x_start: u32, wall_x_end: u32, raw_angle: f32) -> bool {
            let mut xs = wall_x_start;
            let end = math::min(wall_x_end, self.screen_range.len() as u32);

            while xs < end {         
                if !self.screen_range[xs as usize] {
                    xs += 1;
                    continue;
                }
                let mut xe = xs;
                while  xe < end && self.screen_range[xe as usize] {
                    if let WallType::SolidWall(_) = wtype {
                        self.screen_range[xe as usize] = false;
                    }
                    xe += 1;
                }
                if (xe - xs) > 0 {
                    self.draw_wall(actor, surface, wtype, xs, xe, raw_angle);
                    xs = xe + 1;
                } else {
                    break;
                }
            }
            return self.screen_range.contains(&true);
        }
    }

    impl crate::render::Render for RenderSoftware<'_> {
        fn draw<'wad>(&mut self, doom: &mut Doom<'wad>, _last_frame_time: f64, _blending_factor: f64) {
            // Clear
            self.reset();
            // Ref to bsp
            let bsp = &mut doom.bsp;
            let surface = doom.surface.clone();
            let render = self;
            // Draw player 1
            match doom.actors.iter().find(|&actor| actor.borrow().type_id() == 1) {
                Some(actor) => {
                    bsp.visit(
                        &actor.borrow().position(), 
                        render,
                        |subsector_id, render|{
                        let subsector = render.map.sub_sectors[subsector_id as usize];
                        for sector_id in subsector.iter() {
                            let seg = render.map.segs[sector_id as usize];
                            let vertex1 = render.map.vertices[seg.start_vertex_id as usize];
                            let vertex2 = render.map.vertices[seg.end_vertex_id as usize];
                            if let Some((x1,x2, raw_angle)) = render.camera.clip_segment_in_frustum(actor.borrow().as_ref(), &vertex1, &vertex2) {
                               if let Some(wtype) = render.classify_segment(&seg, x1, x2){
                                    render.draw_clip_walls(&actor.borrow(),&mut surface.borrow_mut(), &wtype, x1,  x2, raw_angle);
                               }
                            }                               
                        }
                        return  true;
                    },|node_box, render| { 
                        render.camera.is_box_in_frustum(actor.borrow().as_ref(), &node_box)
                    });
                },
                None => ()
            } 
        }

    }

}