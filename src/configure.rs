
#![allow(dead_code)]
use ini::{Ini, Properties};
use crate::math::{Vector2, Vector4};

pub struct Resource {
    pub wad : String
}

pub struct Screen {
    pub title: String,
    pub window: Vector2<f64>, 
    pub surface: Vector2<u32>, 
    pub frame_rate: u32,
    pub vsync: bool
}

pub struct Camera {
    pub fov: f32,
}

pub struct Player {
    pub speed: f32,
    pub angle_speed: f32,
    pub height: i16,
    pub jump: i16,
    pub jump_speed: i16
}

pub struct Map {
    pub name: String,
}

pub struct Render {
    pub map_2d: Option<Vector4<i32>>, 
    pub bsp_2d: Option<Vector4<i32>>, 
    pub camera_2d: Option<Vector4<i32>>, 
    pub flat_2d: Option<Vector4<i32>>, 
    pub sprite_2d: Option<Vector4<i32>>, 
    pub texture_2d: Option<Vector4<i32>>, 
    pub software_3d: Option<Vector4<i32>>, 
}

#[readonly::make]
pub struct Configure {
    pub resource: Resource,
    pub screen: Screen,
    pub camera: Camera,
    pub player: Player,
    pub map: Map,
    pub render: Option<Render>,
}

impl<T> Vector2<T> where T: std::str::FromStr, <T as std::str::FromStr>::Err: std::fmt::Debug {
    pub fn from_str(s: &str) -> Option<Self> {
        let parts: Vec<&str> = s.split(',').collect();
        if parts.len() != 2 {
            return None;
        }
        Some(Vector2 {
            x: parts[0].trim().parse().ok()?,
            y: parts[1].trim().parse().ok()?,
        })
    }

    pub fn from_optional_str(opt_s: Option<&str>) -> Option<Self> {
        match opt_s {
            Some(s) => Vector2::<T>::from_str(s),
            _ => None
        }
    }
}

impl<T> Vector4<T> where T: std::str::FromStr, <T as std::str::FromStr>::Err: std::fmt::Debug {
    pub fn from_str(s: &str) -> Option<Self> {
        let parts: Vec<&str> = s.split(',').collect();
        if parts.len() != 4 {
            return None;
        }
        Some(Vector4 {
            x: parts[0].trim().parse().ok()?,
            y: parts[1].trim().parse().ok()?,
            z: parts[2].trim().parse().ok()?,
            w: parts[3].trim().parse().ok()?,
        })
    }

    pub fn from_optional_str(opt_s: Option<&str>) -> Option<Self> {
        match opt_s {
            Some(s) => Vector4::<T>::from_str(s),
            _ => None
        }
    }
}

fn bool_from_str(value: Option<&str>) -> Option<bool> {
    match value?.trim() {
        "true" => Some(true),        
        "yes" => Some(true),
        "t" => Some(true),
        "y" => Some(true),
        "1" => Some(true),
        "false" => Some(false),
        "no" => Some(false),
        "f" => Some(false),
        "n" => Some(false),
        "0" => Some(false),
        _ => None
    }
}

impl Resource {
    pub fn from(props: &Properties) -> Option<Self> {
        Some(Resource { 
            wad: String::from(props.get("wad")?),
        })
    }
}

impl Camera {
    pub fn from(props: &Properties) -> Option<Self> {
        Some(Camera { 
            fov: props.get("fov")?.parse().ok()?,
        })
    }
}

impl Player {
    pub fn from(props: &Properties) -> Option<Self> {
        Some(Player { 
            speed: props.get("speed")?.parse().ok()?,
            angle_speed: props.get("angle_speed")?.parse().ok()?,
            height: props.get("height")?.parse().ok()?,
            jump: props.get("jump")?.parse().ok()?,
            jump_speed: props.get("jump_speed")?.parse().ok()?,
        })
    }
}

impl Map {
    pub fn from(props: &Properties) -> Option<Self> {
        Some(Map { 
            name: String::from(props.get("name")?),
        })
    }
}

impl Screen {
    pub fn from(props: &Properties) -> Option<Self> {
        Some(Screen { 
            title: String::from(props.get("title")?),
            window: Vector2::<f64>::from_str(props.get("window")?)?,
            surface: Vector2::<u32>::from_str(props.get("surface")?)?,
            frame_rate: props.get("frame_rate")?.parse().ok()?,
            vsync: bool_from_str(props.get("vsync")).unwrap_or(false),
        })
    }
}

impl Render {
    pub fn from(props: Option<&Properties>) -> Option<Self> {
        match props {
            Some(props) => Some(Render { 
                map_2d: Vector4::<i32>::from_optional_str(props.get("map_2d")),
                bsp_2d: Vector4::<i32>::from_optional_str(props.get("bsp_2d")),
                camera_2d: Vector4::<i32>::from_optional_str(props.get("camera_2d")),
                flat_2d: Vector4::<i32>::from_optional_str(props.get("flat_2d")),
                sprite_2d: Vector4::<i32>::from_optional_str(props.get("sprite_2d")),
                texture_2d: Vector4::<i32>::from_optional_str(props.get("texture_2d")),
                software_3d: Vector4::<i32>::from_optional_str(props.get("software_3d")),
            }),
            _ => None  
        }
    }
}

impl Configure {
    pub fn load_from_file(filename: String) -> Option<Self> {
        if let Ok(ini) = Ini::load_from_file(filename) {
            return Some(Configure {
                resource : Resource::from(ini.section(Some("Resource"))?)?,
                screen : Screen::from(ini.section(Some("Screen"))?)?,
                camera : Camera::from(ini.section(Some("Camera"))?)?,
                player : Player::from(ini.section(Some("Player"))?)?,
                map : Map::from(ini.section(Some("Map"))?)?,
                render : Render::from(ini.section(Some("Render"))),
            });
        }
        None
    }
}