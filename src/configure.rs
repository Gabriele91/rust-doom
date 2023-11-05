
use ini::{Ini, Properties};
pub struct Resource {
    pub wad : String
}

pub struct Size<T> {
    pub width: T, 
    pub height: T, 
}

pub struct Screen {
    pub window: Size<f64>, 
    pub surface: Size<u32>, 
}

pub struct Camera {
    pub fov: f64,
}

#[readonly::make]
pub struct Configure {
    pub resource: Resource,
    pub screen: Screen,
    pub camera: Camera,
}

impl Resource {
    pub fn from(props: &Properties) -> Option<Self> {
        Some(Resource { 
            wad: String::from(props.get("wad")?),
        })
    }
}

impl<T> Size<T> where T: std::str::FromStr, <T as std::str::FromStr>::Err: std::fmt::Debug {
    pub fn from_str(s: &str) -> Option<Self> {
        let parts: Vec<&str> = s.split(',').collect();
        if parts.len() != 2 {
            return None;
        }
        Some(Size {
            width: parts[0].trim().parse().ok()?,
            height: parts[1].trim().parse().ok()?,
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

impl Screen {
    pub fn from(props: &Properties) -> Option<Self> {
        Some(Screen { 
            window: Size::<f64>::from_str(props.get("window")?)?,
            surface: Size::<u32>::from_str(props.get("surface")?)?,
        })
    }
}

impl Configure {
    pub fn load_from_file(filename: String) -> Option<Self> {
        if let Ok(ini) = Ini::load_from_file(filename) {
            return Some(Configure {
                resource : Resource::from(ini.section(Some("Resource"))?)?,
                screen : Screen::from(ini.section(Some("Screen"))?)?,
                camera : Camera::from(ini.section(Some("Camera"))?)?,
            });
        }
        None
    }
}