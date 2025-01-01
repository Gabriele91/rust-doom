#![allow(dead_code)]
// Using engine
use crate::doom::Doom;
// Trait
pub trait Render {
    fn draw<'wad>(&mut self, doom: &mut Doom<'wad>, last_frame_time: f64, blending_factor: f64);
}

pub mod render_2d {
    use std::cell::RefCell;
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
                        &actor.borrow().get_transform().position_as_int(), 
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
                    bsp.visit_debug(&actor.borrow().get_transform().position_as_int(),
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
    
    // Render 2D Camera
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
                     &actor.borrow().get_transform().position_as_int(),
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

        // Render 2D Camera
    #[derive(Clone)]
    pub struct RenderCollision<'wad> {
        map: Rc<Map<'wad>>,
        bounds: [Vector2<i16>; 2],
        size: Vector2<i32>,
        offset: Vector2<i32>
    }

    impl<'wad> RenderCollision<'wad> {
        pub fn new(map: &Rc<Map<'wad>>, size: Vector2<i32>, offset: Vector2<i32>) -> Self {
            let bounds = utils::bound_from_vertices(&map.vertices);
            RenderCollision {
                map: map.clone(),
                bounds: bounds,
                size: size,
                offset: offset,
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

    impl crate::render::Render for RenderCollision<'_> {
        fn draw<'wad>(&mut self, doom: &mut Doom<'wad>, _last_frame_time: f64, _blending_factor: f64) {
            // Ref to bsp
            let surface = doom.surface.clone();
            let render = self;
            // Draw player 1
            match doom.actors.iter().find(|&actor| actor.borrow().type_id() == 1) {
                Some(actor) => {
                    if let Some(map) = &doom.map.blockmaps {
                        let position= actor.borrow().get_transform().position_as_int();
                        if let Some(list_lines) = map.get(position.x, position.y) {
                            for line in list_lines.iter() {
                                let vertex1 = line.start_vertex(&doom.map);
                                let vertex2 = line.end_vertex(&doom.map);
                                render.draw_line(&mut surface.borrow_mut(), &vertex1, &vertex2, &[0xFF,0xC0, 0xCB, 0xFF]);
                            }
                        }
                    }
                },
                None => ()
            }
        }

    }

    // Render 2D textures/sprites/flats
    #[derive(Clone)]
    pub struct RenderTextures<'wad, const C : usize> {
        data_textures: Rc<DataTextures<'wad>>,
        textures: Rc<RefCell<Vec<Rc<Texture<C>>>>>,
        texture_id: usize,
        texture_update: f64,
        size: Vector2<i32>,
        offset: Vector2<i32>,
    }

    impl<'wad, const C : usize> RenderTextures<'wad, C> {
        pub fn new(data_textures:&Rc<DataTextures<'wad>>, textures: &Rc<RefCell<Vec<Rc<Texture<C>>>>>, size: Vector2<i32>, offset: Vector2<i32>) -> Self {
            RenderTextures {
                data_textures: data_textures.clone(),
                textures: textures.clone(),
                texture_id: 0,
                texture_update: 0.0,
                size: size,
                offset: offset,
            }
        }
        fn draw_texture(&self, surface: &mut DoomSurface, texture: &Texture<C>) {
            let start_y = self.offset.y as usize;
            let end_y = ((self.offset.y + self.size.height()) as usize).min(start_y + texture.size.height() as usize);
            
            let start_x = self.offset.x as usize;
            let end_x = ((self.offset.x + self.size.width()) as usize).min(start_x + texture.size.width() as usize);
            
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

    impl<const C : usize> crate::render::Render for RenderTextures<'_, C> {
        fn draw<'wad>(&mut self, doom: &mut Doom<'wad>, last_frame_time: f64, _blending_factor: f64) {
            // Test
            if self.textures.borrow().is_empty() {
                return;
            }
            // Update
            self.texture_update += last_frame_time;
            // Change Texture
            if self.texture_update >= 1.0  {
                self.texture_id += 1;
                self.texture_update = 0.0;
                if  self.texture_id >= self.textures.borrow().len() {
                    self.texture_id = 0;
                }
            }
            // Textures
            let textures: &Vec<Rc<Texture<C>>> = &*self.textures.borrow();
            // Draw
            self.draw_texture(&mut doom.surface.borrow_mut(), &textures[self.texture_id]);
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
    use crate::map::{Map, Seg, SideDef, LineDefFlags};
    use crate::math::{Vector2, radians};
    use crate::shape::Size;
    use crate::window::DoomSurface;
    use crate::data_textures::{Texture, DataTextures, is_sky_texture, remap_sky_texture};

    mod consts {
        pub const VOID_TEXTURE : [u8; 8] = ['-' as u8,0,0,0, 0,0,0,0];
        pub const MAX_SCALE : f32  = 64.0;
        pub const MIN_SCALE : f32 = 0.00390625;
        pub const SKY_SCALE : f32 = 160.0;
        pub const SKY_ALT : i16 = 100;
    }
    
    #[derive(Clone)]
    struct SegExtraData<'wad> {
        seg:&'wad Seg,
        ceiling_texture_id: Option<usize>,
        upper_texture_id: Option<usize>,
        wall_texture_id: Option<usize>,
        lower_texture_id: Option<usize>,
        floor_texture_id: Option<usize>,
        sky_texture_id: Option<usize>,
        light_level: f32
    }

    enum WallType<'wad> {
        SolidWall(&'wad SegExtraData<'wad>),
        PortalWall(&'wad SegExtraData<'wad>)
    }
    
    // Render 3D bsp
    #[derive(Clone)]
    pub struct RenderSoftware<'wad> {
        map: Rc<Map<'wad>>,
        seg_extra_data: Vec<Box<SegExtraData<'wad>>>,
        data_textures: Rc<DataTextures<'wad>>,
        size: Vector2<i32>,
        h_size: Vector2<f32>,
        offset: Vector2<i32>,
        camera: Camera,
        screen_range: Vec<bool>,
        upper_clip: Box<Vec<i32>>,
        lower_clip: Box<Vec<i32>>,
        sky_inv_scale: f32,
        sky_texture_alt: i16
    }

    fn circual_tex(value:f32, size: u16) -> u16 {
        let mod_value = value % size as f32;
        if mod_value < 0.0 {
            (mod_value + size as f32 - 1.0) as u16
        } else {
            mod_value as u16
        }
    }

    impl<'wad> RenderSoftware<'wad> {
        pub fn new(map: &Rc<Map<'wad>>, data_textures: &Rc<DataTextures<'wad>>, size: Vector2<i32>, offset: Vector2<i32>, configure: &configure::Camera) -> Self {
            RenderSoftware {
                map: map.clone(),
                seg_extra_data: vec![],
                data_textures: data_textures.clone(),
                size: size,
                h_size: Vector2::<f32>::from(&size) * 0.5,
                offset: offset,
                camera: Camera::new(configure.fov, size.width().try_into().unwrap()),
                screen_range: vec![false; size.width() as usize],
                upper_clip: Box::new(vec![0; size.width() as usize]),
                lower_clip: Box::new(vec![size.height(); size.width() as usize]),
                sky_inv_scale : consts::SKY_SCALE / size.width() as f32,
                sky_texture_alt : consts::SKY_ALT
            }.preprocessing()
        }

        fn preprocessing(mut self) -> Self
        {
            for seg in &self.map.segs {
                self.seg_extra_data.push(Box::new(SegExtraData {
                    seg: &seg,
                    ceiling_texture_id: seg
                                        .front_sector(&self.map)
                                        .and_then(|sector| self.data_textures.flats_names.iter().position(|flats_name| *flats_name == sector.ceiling_texture)),
                    upper_texture_id: {
                        let line = seg.line_defs(&self.map);
                        let find_id_upper = |side: &SideDef| {
                            if side.upper_texture != consts::VOID_TEXTURE {  
                                self.data_textures.texture_maps.iter().position(|tex_map| tex_map.name == side.upper_texture)
                            } else {
                                None
                            }
                        };
                        line
                        .front_side(&self.map)
                        .and_then(find_id_upper)
                        .or( line.back_side(&self.map).and_then(find_id_upper))
                    },
                    wall_texture_id: {
                        let line = seg.line_defs(&self.map);
                        line
                        .front_side(&self.map)
                        .and_then(|side| { 
                            if side.middle_texture != consts::VOID_TEXTURE {  
                                self.data_textures.texture_maps.iter().position(|tex_map| tex_map.name == side.middle_texture)  
                            } else {
                                None
                            }
                        })
                    },
                    lower_texture_id: {
                        let line = seg.line_defs(&self.map);
                        let find_id_lower = |side: &SideDef| {
                            if side.lower_texture != consts::VOID_TEXTURE {  
                                self.data_textures.texture_maps.iter().position(|tex_map| tex_map.name == side.lower_texture)
                            } else {
                                None
                            }
                        };
                        line
                        .front_side(&self.map)
                        .and_then(find_id_lower)
                        .or( line.back_side(&self.map).and_then(find_id_lower))
                    },
                    floor_texture_id: seg
                                      .front_sector(&self.map)
                                      .and_then(|sector| {
                                        if sector.floor_texture != consts::VOID_TEXTURE {
                                            self.data_textures.flats_names.iter().position(|flats_name| *flats_name == sector.floor_texture)
                                        } else {
                                            None
                                        }
                                      }),
                    sky_texture_id: seg
                                    .front_sector(&self.map)
                                    .and_then(|sector| {
                                        if sector.ceiling_texture != consts::VOID_TEXTURE && is_sky_texture(&sector.ceiling_texture) {
                                            let sky_name = &remap_sky_texture(&sector.ceiling_texture);
                                            self.data_textures.texture_maps.iter().position(|tex_map| tex_map.name == *sky_name)
                                        } else {
                                            None
                                        }
                                    }),
                    light_level : seg
                                  .front_sector(&self.map)
                                  .and_then(|sector| Some(math::clamp(sector.light_level as f32 / 255.0, 0.0, 1.0)))
                                  .unwrap_or(0.0 as f32),
                }));
            }
            // Returns itself
            self
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

        fn apply_light_to_color<'a, const C: usize>(rgba: &'a mut [u8; C], light_level: f32) -> &'a [u8] {
            match C {
                1 => {
                    rgba[0] = (rgba[0] as f32 * light_level) as u8;
                },
                2 => {
                    rgba[0] = (rgba[0] as f32 * light_level) as u8;
                    rgba[1] = (rgba[1] as f32 * light_level) as u8;
                },
                3 | 4 => {
                    rgba[0] = (rgba[0] as f32 * light_level) as u8;
                    rgba[1] = (rgba[1] as f32 * light_level) as u8;
                    rgba[2] = (rgba[2] as f32 * light_level) as u8;
                },
                _ => panic!("Unsupported"),
            }
            rgba
        }
        
        fn classify_segment<'a>(&self, seg_ex: &'a SegExtraData<'wad>, start: u32, end: u32) -> Option<WallType<'a>> {
            if start == end {
                return None;
            }

            // Seg reference
            let seg = seg_ex.seg;

            // Right is mandatory
            let right_sector = seg.right_sector(&self.map)?;
            
            // Left only if it is a portal
            if let Some(left_sector) = seg.left_sector(&self.map) {
                
                // Wall with window
                if right_sector.floor_height != left_sector.floor_height 
                || right_sector.ceiling_height != left_sector.ceiling_height {
                   return Some(WallType::PortalWall(&seg_ex));
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
                return Some(WallType::PortalWall(&seg_ex));

            } else {
                return Some(WallType::SolidWall(&seg_ex));
            }
        }

        fn draw_line(&self, surface: &mut DoomSurface, x: i32, mut y1: i32, mut y2: i32, color: &[u8]) {
            y1 = math::clamp(y1, 0, self.size.height());
            y2 = math::clamp(y2, 0, self.size.height());
            if x < self.size.width() && y1 < y2 {
                let start = Vector2::new(x as i32, y1) + self.offset;
                let end = Vector2::new(x as i32, y2) + self.offset;
                surface.draw_line_lt(&start, &end, &color);
            }
        }

        fn draw_line_texture<const C: usize>(
            &self, 
            surface: &mut DoomSurface, 
            mut x: i32, 
            mut y1: i32,
            mut y2: i32, 
            column: f32, 
            texture_alt: i16, 
            inv_scale: f32,
            tex: &Texture<C>,
            light_level: f32) 
        {
            let u = circual_tex(column, tex.size.width());
            if x < self.size.width() && y1 < y2 {
                x += self.offset.x;
                y1 += self.offset.y;
                y2 += self.offset.y;
                let mut v: f32 = texture_alt as f32 + ((y1 as f32 - self.h_size.height()) * inv_scale);
                for y in y1..y2 {
                    let mut color = tex.get(u, circual_tex(v, tex.size.height())).clone();
                    surface.draw_lt(
                    &Vector2::new(x as usize, y as usize), 
                    RenderSoftware::apply_light_to_color(&mut color, light_level)
                    );
                    v += inv_scale;
                }
            }
        }

        fn draw_flat<const C: usize>(
            &self,
            surface: &mut DoomSurface,
            x: i32,
            y1: i32,
            y2: i32,
            world_z: i16,
            player_angle: f32,
            player_pos: &Vector2<f32>,
            tex: &Texture<C>,
            light_level: f32
        ) {
            if x < self.size.width() && y1 < y2 {
                let player_anglese_rad =  radians(player_angle);
                let player_dir_x = player_anglese_rad.cos();
                let player_dir_y = player_anglese_rad.sin();
                let world_z_float = world_z as f32;
        
                for iy in y1..y2 {
                    let z = self.h_size.width() * world_z_float / (self.h_size.height() - iy as f32);
        
                    let px = player_dir_x * z + player_pos.x;
                    let py = player_dir_y * z + player_pos.y;
        
                    let left_x = -player_dir_y * z + px;
                    let left_y = player_dir_x * z + py;
                    let right_x = player_dir_y * z + px;
                    let right_y = -player_dir_x * z + py;
        
                    let dx = (right_x - left_x) / self.size.width() as f32;
                    let dy = (right_y - left_y) / self.size.width() as f32;
        
                    let tx = circual_tex(left_x + dx * x as f32, tex.size.width());
                    let ty = circual_tex(left_y + dy * x as f32, tex.size.height());
        
                    let mut color = tex.get(tx, ty).clone();

                    surface.draw_lt(
                        &Vector2::new(x as usize, iy as usize), 
                        RenderSoftware::apply_light_to_color(&mut color, light_level)
                    );
                }
            }
        }

        fn draw_wall<'wall>(&mut self, actor: &Box<dyn Actor>, surface: &mut DoomSurface, wtype: &WallType<'wall>, start: u32, end: u32, wall_angle: f32) {
            match wtype {
                WallType::SolidWall(seg_ex) => {
                    // Alias
                    let seg = seg_ex.seg;
                    let line = seg.line_defs(&self.map);
                    let side = line.front_side(&self.map).unwrap();
                    let sector = seg.front_sector(&self.map).unwrap();
                    let angle = actor.angle();
                    let position = actor.position();
                    let height = actor.height();
                    let start_vertex = Vector2::<f32>::from( seg.start_vertex(&self.map) );
                    let half_height = self.h_size.height();
                    // Texture
                    let light_level = seg_ex.light_level;
                    // Get texture
                    let ceiling_texture = seg_ex.ceiling_texture_id.and_then(|id| self.data_textures.flat_id(id));
                    let wall_texture = seg_ex.wall_texture_id.and_then(|id| self.data_textures.texture_id(id));
                    let floor_texture = seg_ex.floor_texture_id.and_then(|id| self.data_textures.flat_id(id));
                    let sky_texture = seg_ex.sky_texture_id.and_then(|id| self.data_textures.texture_id(id));
                    // Height of wall w/ rispect to player
                    let wall_ceiling = sector.ceiling_height - height;
                    let wall_floor = sector.floor_height - height;
                    // What to draw
                    let b_ceiling_is_sky = sky_texture.is_some();
                    let b_draw_ceiling = wall_ceiling > 0 || b_ceiling_is_sky;
                    let b_draw_wall = wall_texture.is_some();
                    let b_draw_floor = wall_floor < 0;
                    // Calculate the scaling factors of the left and right edges of the wall range
                    let wall_normal_angle = seg.float_degrees_angle() + 90.0;
                    let offset_angle = wall_normal_angle - wall_angle;
                    // Wall distance
                    let hypotenuse = position.distance(&start_vertex);
                    let wall_distance = hypotenuse * math::radians(offset_angle).cos();
                    // Test
                    if !b_draw_wall && !b_draw_ceiling && !b_draw_floor {
                        return;
                    }
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
                    //////////////////////////////////////////////////////////////////////////////
                    // Determine how the wall textures are horizontally aligned
                    let mut wall_offset = hypotenuse * math::radians(offset_angle).sin();
                    wall_offset += seg.offset as f32 + side.offset.x as f32;
                    let wall_center_angle = wall_normal_angle - angle;
                    // Texture height
                    let middle_texture_alt = {
                        if let Some(ref texture) = wall_texture {
                            if line.has_flag(LineDefFlags::DontPegBottom) { 
                                wall_floor + texture.size.y as i16 + side.offset.y  
                            } else {
                                wall_ceiling + side.offset.y  
                            }
                        } else {
                            0
                        }
                    };
                    // Texture scale
                    let mut wall_tex_y_scale = wall_scale_1;
                    //////////////////////////////////////////////////////////////////////////////
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
                            if ceiling_wall_y1 < ceiling_wall_y2 {
                                if b_ceiling_is_sky {
                                    if let Some(ref texture) = sky_texture {
                                        let texture_column = 2.2 * (angle + self.camera.x_to_angle(x));
                                        self.draw_line_texture(
                                            surface, 
                                            x as i32,
                                            ceiling_wall_y1,
                                            ceiling_wall_y2, 
                                            texture_column, 
                                            self.sky_texture_alt, 
                                            self.sky_inv_scale, 
                                            texture.as_ref(), 
                                            1.0
                                        );
                                    }
                                }
                                else {
                                    if let Some(ref texture) = ceiling_texture {
                                        self.draw_flat(
                                            surface, 
                                            x as i32, 
                                            ceiling_wall_y1, 
                                            ceiling_wall_y2, 
                                            wall_ceiling, 
                                            angle, 
                                            position, 
                                            texture.as_ref(), 
                                            light_level
                                        );
                                    }
                                }
                            }
                        }
                        if b_draw_wall {
                            let middle_wall_y1 = math::max(draw_wall_y1, self.upper_clip[x as usize]);
                            let middle_wall_y2 = math::min(draw_wall_y2, self.lower_clip[x as usize]);
                            if middle_wall_y1 < middle_wall_y2 {
                                if let Some(ref texture) = wall_texture { 
                                    let wall_angle = wall_center_angle - self.camera.x_to_angle(x);
                                    let texture_column = wall_distance * radians(wall_angle).tan() - wall_offset;
                                    let inv_scale =  1.0 / wall_tex_y_scale;
                                    self.draw_line_texture(
                                        surface, 
                                        x as i32, 
                                        middle_wall_y1, 
                                        middle_wall_y2, 
                                        texture_column, 
                                        middle_texture_alt, 
                                        inv_scale, 
                                    texture.as_ref(), 
                                        light_level
                                    );
                                }
                            }
                            
                        }
                        if b_draw_floor {
                            let floor_wall_y1 = math::max(draw_wall_y2, self.upper_clip[x as usize]);
                            let floor_wall_y2 = self.lower_clip[x as usize];
                            if floor_wall_y1 < floor_wall_y2 {
                                if let Some(ref texture) = floor_texture { 
                                    self.draw_flat(
                                        surface, 
                                        x as i32, 
                                        floor_wall_y1, 
                                        floor_wall_y2, 
                                        wall_floor, 
                                        angle, 
                                        position, 
                                        texture.as_ref(), 
                                        light_level
                                    );
                                }
                            }
                        }
                        // Next step
                        wall_tex_y_scale += wall_scale_step;
                        wall_y1 += wall_y1_step;
                        wall_y2 += wall_y2_step;
                    }
                },
                WallType::PortalWall(seg_ex) => {
                    // Alias
                    let seg = seg_ex.seg;
                    let line = seg.line_defs(&self.map);
                    let side = line.front_side(&self.map).unwrap();
                    let front_sector = seg.front_sector(&self.map).unwrap();
                    let back_sector = seg.back_sector(&self.map).unwrap();
                    let angle = actor.angle();
                    let position = actor.position();
                    let height = actor.height();
                    let start_vertex = Vector2::<f32>::from( seg.start_vertex(&self.map) );
                    let half_height = self.h_size.height();
                    // Get texture
                    let ceiling_texture = seg_ex.ceiling_texture_id.and_then(|id| self.data_textures.flat_id(id));
                    let upper_texture = seg_ex.upper_texture_id.and_then(|id| self.data_textures.texture_id(id));
                    let lower_texture = seg_ex.lower_texture_id.and_then(|id| self.data_textures.texture_id(id));
                    let floor_texture = seg_ex.floor_texture_id.and_then(|id| self.data_textures.flat_id(id));
                    let sky_texture = seg_ex.sky_texture_id.and_then(|id| self.data_textures.texture_id(id));
                    let light_level = seg_ex.light_level;
                    // Height of wall w/ rispect to player
                    let front_wall_floor = front_sector.floor_height - height;
                    let mut front_wall_ceiling = front_sector.ceiling_height - height;
                    let back_wall_floor = back_sector.floor_height - height;
                    let back_wall_ceiling = back_sector.ceiling_height - height;
                    // set what to draw
                    let mut b_draw_upper_wall = false;
                    let mut b_draw_ceiling = false;
                    let mut b_draw_floor = false;
                    let mut b_draw_lower_wall = false;
                    let b_ceiling_is_sky= sky_texture.is_some();
                    // Test if the upper is a sky
                    if  front_sector.ceiling_texture == back_sector.ceiling_texture && b_ceiling_is_sky {
                        front_wall_ceiling = back_wall_ceiling;
                    }
                    // What to draw
                    if front_wall_ceiling != back_wall_ceiling 
                    || front_sector.light_level != back_sector.light_level 
                    || front_sector.ceiling_texture != back_sector.ceiling_texture {
                        b_draw_upper_wall = upper_texture.is_some() && back_wall_ceiling < front_wall_ceiling;
                        b_draw_ceiling = front_wall_ceiling >= 0 || b_ceiling_is_sky;
                    }

                    if front_wall_floor != back_wall_floor 
                    || front_sector.light_level != back_sector.light_level 
                    || front_sector.floor_texture != back_sector.floor_texture {
                        b_draw_lower_wall = floor_texture.is_some() && back_wall_floor > front_wall_floor;
                        b_draw_floor = front_wall_floor <= 0;
                    }
                    // Test
                    if !b_draw_upper_wall && !b_draw_ceiling && !b_draw_floor && !b_draw_lower_wall {
                        return;
                    }
                    // Calculate the scaling factors of the left and right edges of the wall range
                    let wall_normal_angle = seg.float_degrees_angle() + 90.0;
                    let offset_angle = wall_normal_angle - wall_angle;
                    // Wall distance
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
                    //////////////////////////////////////////////////////////////////////////////
                    // Determine how the wall textures are horizontally aligned
                    struct TextureDrawData 
                    {
                        wall_offset: f32,
                        wall_center_angle: f32,
                        upper_texture_alt: i16,
                        lower_texture_alt: i16
                    }
                    let texture_draw_data = {
                        if !b_draw_upper_wall && !b_draw_lower_wall {
                            TextureDrawData {
                                wall_offset: 0.0,
                                wall_center_angle: 0.0,
                                upper_texture_alt: 0,
                                lower_texture_alt: 0
                            }
                        }
                        else {
                            TextureDrawData {                                
                                wall_offset: hypotenuse * math::radians(offset_angle).sin() + seg.offset as f32 + side.offset.x as f32,
                                wall_center_angle: wall_normal_angle - angle,
                                upper_texture_alt: {
                                    if b_draw_upper_wall {
                                        if line.has_flag(LineDefFlags::DontPegTop) { 
                                            front_wall_ceiling + side.offset.y
                                        } else {
                                            back_wall_ceiling + upper_texture.clone().unwrap().size.y as i16 + side.offset.y
                                        }
                                    } else {
                                        0i16
                                    }
                                },
                                lower_texture_alt: {
                                    if b_draw_lower_wall {
                                        if line.has_flag(LineDefFlags::DontPegBottom) {
                                            front_wall_ceiling + side.offset.y
                                        } else {
                                            back_wall_floor + side.offset.y  
                                        }
                                    } else {
                                        0i16
                                    }
                                }
                            }
                        }
                    };
                    // Texture scale
                    let mut wall_tex_y_scale = wall_scale_1;
                    // Determine where on the screen the wall is drawn
                    // Top wall
                    let mut wall_y1 = half_height - front_wall_ceiling as f32 * wall_scale_1;
                    let wall_y1_step = -wall_scale_step * front_wall_ceiling as f32;
                    // Bottom wall
                    let mut wall_y2 = half_height - front_wall_floor as f32 * wall_scale_1;
                    let wall_y2_step = -wall_scale_step * front_wall_floor as f32;
                    // Determinate y for the top and bottom walls
                    let mut portal_y1 = wall_y2;
                    let mut portal_y1_step = wall_y2_step;
                    if b_draw_upper_wall && back_wall_ceiling > front_wall_floor {
                            portal_y1 = half_height - back_wall_ceiling as f32 * wall_scale_1;
                            portal_y1_step = -wall_scale_step * back_wall_ceiling as f32;
                    }
                    let mut portal_y2 = wall_y1;
                    let mut portal_y2_step = wall_y1_step;
                    if b_draw_lower_wall && back_wall_floor < front_wall_ceiling {
                            portal_y2 = half_height - back_wall_floor as f32 * wall_scale_1;
                            portal_y2_step = -wall_scale_step * back_wall_floor as f32;
                    }

                    // Draw
                    for x in start..end {
                        let draw_wall_y1 = wall_y1 as i32;
                        let draw_wall_y2 = wall_y2 as i32;
                        let (draw_texture_column, inv_tex_scale) = {
                            if b_draw_upper_wall || b_draw_lower_wall {
                                let wall_angle = texture_draw_data.wall_center_angle - self.camera.x_to_angle(x);
                                let draw_texture_column = wall_distance * radians(wall_angle).tan() - texture_draw_data.wall_offset;
                                (draw_texture_column, 1.0 / wall_tex_y_scale)
                            } else {
                                (0.0,0.0)
                            }
                        };


                        if b_draw_upper_wall {
                            if b_draw_ceiling {
                                let ceiling_wall_y1 = self.upper_clip[x as usize];
                                let ceiling_wall_y2 = math::min(draw_wall_y1, self.lower_clip[x as usize]);
                                if ceiling_wall_y1 < ceiling_wall_y2 {
                                    if b_ceiling_is_sky {
                                        if let Some(ref texture) = sky_texture { 
                                            let texture_column = 2.2 * (angle + self.camera.x_to_angle(x));
                                            self.draw_line_texture(
                                                surface, 
                                                x as i32,
                                                ceiling_wall_y1,
                                                ceiling_wall_y2, 
                                                texture_column, 
                                                self.sky_texture_alt, 
                                                self.sky_inv_scale, 
                                                texture.as_ref(), 
                                                1.0
                                            );
                                        }
                                    } else {
                                        if let Some(ref texture) = ceiling_texture { 
                                            self.draw_flat(
                                                surface, 
                                                x as i32, 
                                                ceiling_wall_y1, 
                                                ceiling_wall_y2, 
                                                front_wall_ceiling, 
                                                angle, 
                                                position, 
                                                texture.as_ref(), 
                                                light_level
                                            );
                                        }
                                    }
                                }
                            }
                            let draw_upper_wall_y1 = wall_y1 as i32 - 1;
                            let draw_upper_wall_y2 = portal_y1 as i32;

                            let middle_wall_y1 = math::max(draw_upper_wall_y1, self.upper_clip[x as usize]);
                            let middle_wall_y2 = math::min(draw_upper_wall_y2, self.lower_clip[x as usize]);

                            if middle_wall_y1 < middle_wall_y2 {
                                if let Some(ref texture) = upper_texture {                                
                                    self.draw_line_texture(
                                        surface, 
                                        x as i32, 
                                        middle_wall_y1, 
                                        middle_wall_y2, 
                                        draw_texture_column, 
                                        texture_draw_data.upper_texture_alt, 
                                        inv_tex_scale, 
                                        texture.as_ref(), 
                                        light_level
                                    );
                                }
                            }
                            if self.upper_clip[x as usize] < middle_wall_y2 {
                                self.upper_clip[x as usize] = middle_wall_y2
                            }
                            portal_y1 += portal_y1_step
                        }

                        if b_draw_ceiling {
                            let ceiling_wall_y1 = self.upper_clip[x as usize];
                            let ceiling_wall_y2 = math::min(draw_wall_y1, self.lower_clip[x as usize]);
                            if ceiling_wall_y1 < ceiling_wall_y2 {
                                if b_ceiling_is_sky {
                                    if let Some(ref texture) = sky_texture { 
                                        let texture_column = 2.2 * (angle + self.camera.x_to_angle(x));
                                        self.draw_line_texture(
                                            surface, 
                                            x as i32,
                                            ceiling_wall_y1,
                                            ceiling_wall_y2, 
                                            texture_column,
                                            self.sky_texture_alt, 
                                            self.sky_inv_scale, 
                                            texture.as_ref(), 
                                            1.0
                                        );
                                    }
                                } else {
                                    if let Some(ref texture) = ceiling_texture { 
                                        self.draw_flat(
                                            surface, 
                                            x as i32, 
                                            ceiling_wall_y1, 
                                            ceiling_wall_y2, 
                                            front_wall_ceiling, 
                                            angle, 
                                            position, 
                                            texture.as_ref(), 
                                            light_level
                                        );
                                    }
                                }
                            }
                            if self.upper_clip[x as usize] < ceiling_wall_y2 {
                                self.upper_clip[x as usize] = ceiling_wall_y2
                            }
                        }

                        if b_draw_lower_wall {
                            if b_draw_floor {
                                let floor_wall_y1 = math::max(draw_wall_y2, self.upper_clip[x as usize]);
                                let floor_wall_y2 = self.lower_clip[x as usize];
                                if floor_wall_y1 < floor_wall_y2 {
                                    if let Some(ref texture) = floor_texture { 
                                        self.draw_flat(
                                            surface, 
                                            x as i32, 
                                            floor_wall_y1, 
                                            floor_wall_y2, 
                                            front_wall_floor, 
                                            angle, 
                                            position, 
                                            texture.as_ref(), 
                                            light_level
                                        );
                                    }
                                }
                            }
                            let draw_lower_wall_y1 = portal_y2 as i32 - 1;
                            let draw_lower_wall_y2 = wall_y2 as i32;

                            let middle_wall_y1 = math::max(draw_lower_wall_y1, self.upper_clip[x as usize]);
                            let middle_wall_y2 = math::min(draw_lower_wall_y2, self.lower_clip[x as usize]);
                            if middle_wall_y1 < middle_wall_y2 {
                                if let Some(ref texture) = lower_texture {                                
                                    self.draw_line_texture(
                                        surface, 
                                        x as i32, 
                                        middle_wall_y1, 
                                        middle_wall_y2, 
                                        draw_texture_column, 
                                        texture_draw_data.lower_texture_alt, 
                                        inv_tex_scale, 
                                        texture.as_ref(), 
                                        light_level
                                    );
                                }
                            }
                            if self.lower_clip[x as usize] > middle_wall_y1 {
                                self.lower_clip[x as usize] = middle_wall_y1
                            }
                            portal_y2 += portal_y2_step
                        }
                        
                        if b_draw_floor {
                            let floor_wall_y1 = math::max(draw_wall_y2, self.upper_clip[x as usize]);
                            let floor_wall_y2 = self.lower_clip[x as usize];
                            if floor_wall_y1 < floor_wall_y2 {
                                if let Some(ref texture) = floor_texture { 
                                    self.draw_flat(
                                        surface, 
                                        x as i32, 
                                        floor_wall_y1, 
                                        floor_wall_y2, 
                                        front_wall_floor, 
                                        angle, 
                                        position, 
                                        texture.as_ref(), 
                                        light_level
                                    );
                                }
                            }
                            if self.lower_clip[x as usize] > draw_wall_y2 {
                                self.lower_clip[x as usize] = floor_wall_y1;
                            }
                        }
                        // Next step
                        wall_tex_y_scale += wall_scale_step;
                        wall_y1 += wall_y1_step;
                        wall_y2 += wall_y2_step;
                    }
                }
            }
        }

        fn draw_clip_walls<'wall>(&mut self, actor: &Box<dyn Actor>, surface: &mut DoomSurface, wtype: &WallType<'wall>, wall_x_start: u32, wall_x_end: u32, wall_angle: f32) -> bool {
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
                    self.draw_wall(actor, surface, wtype, xs, xe, wall_angle);
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
            match doom.actors.iter().find(|&actor| actor.as_ref().borrow().type_id() == 1) {
                Some(actor) => {
                    bsp.visit(
                        &actor.as_ref().borrow().get_transform().position_as_int(), 
                        render,
                        |subsector_id, render|{
                        let subsector = render.map.sub_sectors[subsector_id as usize];
                        for sector_id in subsector.iter() {
                            let seg_ex = render.seg_extra_data[sector_id as usize].clone();
                            let vertex1= render.map.vertices[seg_ex.seg.start_vertex_id as usize];
                            let vertex2= render.map.vertices[seg_ex.seg.end_vertex_id as usize];
                            if let Some((x1,x2, wall_angle)) = render.camera.clip_segment_in_frustum(actor.as_ref().borrow().as_ref(), &vertex1, &vertex2) {
                               if let Some(wtype) = render.classify_segment(&seg_ex, x1, x2){
                                    render.draw_clip_walls(&actor.as_ref().borrow(), &mut surface.borrow_mut(), &wtype, x1,  x2, wall_angle);
                               }
                            }                               
                        }
                        return  true;
                    },|node_box, render| { 
                        render.camera.is_box_in_frustum(actor.as_ref().borrow().as_ref(), &node_box)
                    });
                },
                None => ()
            } 
        }

    }

}