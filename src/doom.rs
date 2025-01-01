// Using engine
use crate::actors::Actor;
use crate::bsp::BSP;
use crate::configure::Configure;
use crate::collision::CollisionSolver;
use crate::render::{
    render_2d::{RenderBSP, RenderCamera, RenderMap, RenderCollision, RenderTextures},
    render_3d::RenderSoftware,
    Render,
};
use crate::shape::Size;
use crate::wad::Reader;
use crate::window::DoomSurface;
use crate::{actors::Player, map::Map};
use crate::data_textures::DataTextures;
// Utils
use std::boxed::Box;
use std::cell::RefCell;
use std::rc::Rc;
use std::vec::Vec;
use winit::dpi::PhysicalSize;
use winit::window::Window;
use winit::{event::Event, keyboard::KeyCode};
use winit_input_helper::WinitInputHelper;

pub struct Doom<'wad> {
    pub wad: Rc<Reader>,

    pub input: WinitInputHelper,
    pub map: Rc<Map<'wad>>,
    pub data_textures: Rc<DataTextures<'wad>>,
    pub bsp: BSP<'wad>,
    pub actors: Vec<Rc<RefCell<Box<dyn Actor>>>>,

    pub surface: Rc<RefCell<DoomSurface>>,
    pub renders: Vec<Rc<RefCell<Box<dyn Render + 'wad>>>>,
    pub collider: Rc<RefCell<CollisionSolver<'wad>>>
}

macro_rules! crea_render {
    ($t:expr) => {
        Rc::new(RefCell::new(Box::new($t) as Box<dyn Render + 'wad>))
    };
}

impl<'wad> Doom<'wad> {
    pub fn new(window: &Window, configure: &Configure) -> Box<Self> {
        let wad = Rc::new(Reader::new(&configure.resource.wad).unwrap());
        let map = Rc::new(Map::new(&wad, &configure.map).unwrap());
        let data_textures = Rc::new(DataTextures::new(&wad).unwrap());
        let surface = Rc::new(RefCell::new(
            DoomSurface::new(
                PhysicalSize::<u32>::new(
                    configure.screen.surface.width(),
                    configure.screen.surface.height(),
                ),
                configure.screen.vsync,
                &window,
            )
            .unwrap(),
        ));
        Box::new(Doom {
            // Resource
            wad,
            // Logic
            input: WinitInputHelper::new(),
            map: map.clone(),
            data_textures: data_textures.clone(),
            bsp: BSP::new(&map),
            actors: Doom::create_actors(&map, &configure),
            // Render
            surface,
            renders: {
                let mut renders = vec![];
                if let Some(render) = &configure.render {
                    if let Some(software_3d) = &render.software_3d {
                        renders.push(crea_render!(RenderSoftware::new(
                            &map,
                            &data_textures,
                            software_3d.zw(),
                            software_3d.xy(),
                            &configure.camera
                        )));
                    }
                    if let Some(sprite_2d) = &render.texture_2d {
                        renders.push(crea_render!(RenderTextures::new(
                            &data_textures,
                            &data_textures.textures,
                            sprite_2d.zw(),
                            sprite_2d.xy(),
                        )));
                    }
                    if let Some(sprite_2d) = &render.sprite_2d {
                        renders.push(crea_render!(RenderTextures::new(
                            &data_textures,
                            &data_textures.sprites,
                            sprite_2d.zw(),
                            sprite_2d.xy(),
                        )));
                    }
                    if let Some(flat_2d) = &render.flat_2d {
                        renders.push(crea_render!(RenderTextures::new(
                            &data_textures,
                            &data_textures.flats,
                            flat_2d.zw(),
                            flat_2d.xy(),
                        )));
                    }
                    if let Some(map_2d) = &render.map_2d {
                        renders.push(crea_render!(RenderMap::new(
                            &map,
                            map_2d.zw(),
                            map_2d.xy()
                        )));
                    }
                    if let Some(bsp_2d) = &render.bsp_2d {
                        renders.push(crea_render!(RenderBSP::new(
                            &map,
                            bsp_2d.zw(),
                            bsp_2d.xy()
                        )));
                    }
                    if let Some(camera_2d) = &render.camera_2d {
                        renders.push(crea_render!(RenderCamera::new(
                            &map,
                            camera_2d.zw(),
                            camera_2d.xy(),
                            &configure.camera
                        )));
                    }
                    if let Some(collision_2d) = &render.collision_2d {
                        renders.push(crea_render!(RenderCollision::new(
                            &map,
                            collision_2d.zw(),
                            collision_2d.xy(),
                        )));
                    }
                }
                renders
            },
            collider: Rc::new(RefCell::new(CollisionSolver::new(&map)))
        })
    }

    pub fn update(&mut self, last_frame_time: f64, blending_factor: f64) {
        // Update actors
        for actor in &self.actors {
            actor.borrow_mut().update(self, last_frame_time, blending_factor);
        }
        // Update collisions
        self.collider.clone().borrow_mut().update(self, last_frame_time, blending_factor);
    }

    pub fn draw(&mut self, last_frame_time: f64, blending_factor: f64) {
        self.surface.borrow_mut().clear([0, 0, 0, 0xff]);
        for render_id in 0..self.renders.len() {
            let render = self.renders[render_id].clone();
            render.borrow_mut().draw(self, last_frame_time, blending_factor);
        }
        self.surface.borrow_mut().swap().unwrap();
    }

    pub fn control(&mut self, event: &Event<()>, last_frame_time: f64, blending_factor: f64) -> bool {
        // Input
        if self.input.update(&event) {
            // Close events
            if self.input.key_pressed(KeyCode::Escape) || self.input.close_requested() {
                return false;
            } else {
                for actor in &mut self.actors {
                    actor.borrow_mut().control(&self.input, last_frame_time, blending_factor);
                }
            }
        }
        return true;
    }

    fn create_actors(map: &Rc<Map<'wad>>, configure: &Configure) -> Vec<Rc<RefCell<Box<dyn Actor>>>> {
        let mut actors = vec![];
        for thing in &map.things {
            match thing.type_id {
                1 => actors.push(Rc::new(RefCell::new(Player::new(&thing, &configure)))),
                _ => (),
            }
        }
        return actors;
    }
}
