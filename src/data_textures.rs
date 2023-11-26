// Clobal
use std::{mem, vec, rc::Rc, u8, ops::Index};
// Engine
use crate::{wad, math::Vector2};

#[repr(packed)]
#[allow(dead_code)]
#[derive(Debug)]
#[readonly::make]
pub struct Palette([[u8; 3]; 256]);

#[repr(packed)]
#[allow(dead_code)]
#[derive(Debug)]
#[readonly::make]
pub struct PatchHeader {
    pub size: Vector2<u16>,
    pub offset: Vector2<u16>,
}

#[repr(packed)]
#[allow(dead_code)]
#[readonly::make]
pub struct PatchContent {
    pub colums: Vec<u8>
}

#[repr(packed)]
#[allow(dead_code)]
#[derive(Debug)]
#[readonly::make]
pub struct PatchColumn {
    pub y_offset: u8, 
    pub length: u8, 
    _padding_: u8, 
}

#[repr(packed)]
#[allow(dead_code)]
#[derive(Debug)]
#[readonly::make]
pub struct PatchName([u8; 8]);

pub struct Patch {
    pub name: PatchName,
    pub is_sky: bool,
    pub header: PatchHeader,
    pub content: PatchContent,
    pub colums: Vec<PatchColumn>
}

#[repr(packed)]
#[allow(dead_code)]
#[derive(Debug)]
#[readonly::make]
pub struct RawFlats([u8; 64 * 64]);

pub struct Texture<const C : usize> {
    pub size: Vector2<u16>,
    pub colors: Vec<[u8; C]>
}

pub struct DataTextures<'a> {
        reader: Rc<wad::Reader>,
    // Palette
    pub palettes: Vec<&'a Palette>,
    // Top/Bottom textures
    pub raw_flats: Vec<&'a RawFlats>,
    pub flats: Vec< Texture<3> >,
    // Patch and relative sprite
    pub patch_names: Vec<&'a PatchName>,
    pub patches: Vec<Patch>,
    pub sprites: Vec< Texture<4> >
}

// Palette
impl Palette {
    // Slices method
    pub fn slices(&self) -> &[[u8; 3]; 256] {
        &self.0
    }
}

impl<'a> IntoIterator for &'a Palette {
    type Item = &'a [u8; 3];
    type IntoIter = std::slice::Iter<'a, [u8; 3]>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}

// Implementing Indexing
impl Index<usize> for Palette {
    type Output = [u8; 3];

    fn index(&self, idx: usize) -> &Self::Output {
        &self.0[idx]
    }
}

// Implement Palette
impl RawFlats {
    // Slices method
    pub fn slices(&self) -> &[u8] {
        &self.0
    }
}

impl<'a> IntoIterator for &'a RawFlats {
    type Item = &'a u8;
    type IntoIter = std::slice::Iter<'a, u8>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}

// Implement DataTextures
impl<'a> DataTextures<'a> {
    pub fn new(reader: &Rc<wad::Reader>) -> Option<Self> {
        if let Some(directories) = reader.directories() {
            if let Some(palettes_id) = directories.index_of(&String::from("PLAYPAL")) {
                let mut data_textures = DataTextures {
                    reader: reader.clone(),
                    palettes: vec![], 

                    raw_flats: vec![], 
                    flats: vec![], 
                    
                    patch_names: vec![], 
                    patches: vec![], 
                    sprites: vec![], 
                };
                data_textures.palettes = data_textures.extract::<Palette>(&directories[palettes_id]);
                data_textures.raw_flats = data_textures.extract_a_set(&directories, String::from("F_START"), String::from("F_END"));
                data_textures.patch_names = data_textures.extract_from_name::<PatchName>(&directories, String::from("PNAMES"));

                // Build images:
                if !data_textures.palettes.is_empty() {
                    data_textures.build_flats(&data_textures.palettes[0]);
                }
                
                return Some(data_textures)
            }
        }
        return None
    }   

    fn extract<T>(&self, directory: &wad::Directory) -> Vec<&'a T> {
        let buffer = &self.reader.buffer;
        let mut vec_t = vec![];   
        for chunk_offset in directory.data::<T>() {
            let value: &'a T = unsafe { mem::transmute(&buffer[chunk_offset]) };
            vec_t.push(value);
        }
        return vec_t;
    }
     
    fn extract_from_name<T>(&self, directories: &wad::DirectoryList, name: String) -> Vec<&'a T> {
        let mut vec_t = vec![];   
        if let Some(directory_id) = directories.index_of(&name) {
            let buffer = &self.reader.buffer;
            for chunk_offset in directories[directory_id].data::<T>() {
                let value: &'a T = unsafe { mem::transmute(&buffer[chunk_offset]) };
                vec_t.push(value);
            }
        }
        return vec_t;
    }

    fn extract_a_set<T>(&self, directories: &wad::DirectoryList, start: String, end: String) -> Vec<&'a T> {
        let mut vec_t = vec![];   
        if let Some(start_id) = directories.index_of(&start) {
            if let Some(end_id) = directories.index_of(&end) {
                for id in start_id..=end_id {
                    vec_t.extend(self.extract::<T>(&directories[id]));
                }
            }
        }
        vec_t
    }

    fn build_flats(&mut self, palette: &Palette) {
        self.flats.clear();
        self.flats.reserve(self.raw_flats.len());
        for ptexture in &self.raw_flats {
            self.flats.push(
                Texture {
                    size: Vector2::new(64, 64),
                    colors : {
                        ptexture.slices().iter().map(|id| (*palette)[(*id) as usize]).collect()
                    }
                }
            )
        }
    }

    fn patch_is_sky(pname: &PatchName) -> bool {
        match pname {
            PatchName(value) => match value {
                // DOOM 1&2
                b"SKY\0\0\0\0\0" => true,
                b"SKY1\0\0\0\0" => true,
                b"SKY2\0\0\0\0" => true,
                b"SKY3\0\0\0\0" => true,
                b"SKY4\0\0\0\0" => true,
                // Heretic
                b"SKY5\0\0\0\0" => true,
                // Hexen
                b"SKYFOG\0\0" => true,
                b"SKYFOG2\0" => true,
                b"SKYWALL\0" => true,
                b"SKYWALL2" => true,
                _ => false
            },
            _ => false
        }
    }
    
    /*
    fn extract_patch(&self, directory: &wad::Directory) -> Vec<&'a Patch> {
        let buffer = &self.reader.buffer;
        let mut vec_t = vec![];   
        for chunk_offset in directory.data_by_step::<T>(0, size_of(PatchHeader)) {
            let value: &'a T = unsafe { mem::transmute(&buffer[chunk_offset]) };
            vec_t.push(value);
        }
        return vec_t;
    }
    */

}