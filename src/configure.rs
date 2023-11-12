
use ini::{Ini, Properties};
use crate::math::{Vector2, Vector4};

pub struct Resource {
    pub wad : String
}

pub struct Screen {
    pub window: Vector2<f64>, 
    pub surface: Vector2<u32>, 
}

pub struct Camera {
    pub fov: f64,
}

pub struct Debug {
    pub render_map_2d: Option<Vector4<u32>>, 
    pub render_bsp_2d: Option<Vector4<u32>>, 
}

#[readonly::make]
pub struct Configure {
    pub resource: Resource,
    pub screen: Screen,
    pub camera: Camera,
    pub debug: Option<Debug>,
}

impl Resource {
    pub fn from(props: &Properties) -> Option<Self> {
        Some(Resource { 
            wad: String::from(props.get("wad")?),
        })
    }
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

impl Camera {
    pub fn from(props: &Properties) -> Option<Self> {
        Some(Camera { 
            fov: props.get("fov")?.parse().ok()?,
        })
    }
}

impl Screen {
    pub fn from(props: &Properties) -> Option<Self> {
        Some(Screen { 
            window: Vector2::<f64>::from_str(props.get("window")?)?,
            surface: Vector2::<u32>::from_str(props.get("surface")?)?,
        })
    }
}

impl Debug {
    pub fn from(props: Option<&Properties>) -> Option<Self> {
        match props {
            Some(props) => Some(Debug { 
                render_map_2d: Vector4::<u32>::from_optional_str(props.get("render_map_2d")),
                render_bsp_2d: Vector4::<u32>::from_optional_str(props.get("render_bsp_2d")),
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
                debug : Debug::from(ini.section(Some("Debug"))),
            });
        }
        None
    }
}