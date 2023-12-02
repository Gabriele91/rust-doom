// Global
use std::{mem, mem::size_of, rc::Rc, ops::Index};

// Engine
use crate::{wad, math::Vector2};

#[repr(packed)]
#[allow(dead_code)]
#[derive(Debug)]
#[readonly::make]
pub struct Palette([[u8; 3]; 256]);

#[repr(packed)]
#[allow(dead_code)]
#[readonly::make]
pub struct PatchHeader {
    pub size: [u16;2],
    pub offset: [u16;2],
}

#[allow(dead_code)]
#[derive(Debug)]
#[readonly::make]
pub struct PatchContent<'a>(&'a [u32]);

#[repr(packed)]
#[allow(dead_code)]
#[derive(Clone, Debug)]
#[readonly::make]
pub struct PatchColumnHeaderData {
    pub y_offset: u8, 
    pub length: u8, 
    _padding_: u8, 
}

#[repr(packed)]
#[allow(dead_code)]
#[derive(Clone, Debug)]
#[readonly::make]
pub struct PatchColumnData<'a> {
    pub header: &'a PatchColumnHeaderData,
    pub data: Option<&'a [u8]>
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
#[readonly::make]
pub struct PatchColumn<'a>(Vec<PatchColumnData<'a>>);

#[allow(dead_code)]
#[readonly::make]
pub struct Patch<'a> {
    pub name: [u8; 8],
    pub header: &'a PatchHeader,
    pub content: PatchContent<'a>,
    pub columns: Vec<PatchColumn<'a>>,
    pub is_sky: bool
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
    pub patch_names: Vec<&'a [u8;8]>,
    pub sprite_patches: Vec<Patch<'a>>,
    pub texture_patches: Vec<Patch<'a>>,
    // Processed
    pub sprites: Vec<Texture<4>>,
    pub textures: Vec<Texture<4>>,
}

// PatchName
fn u8str_to_string<'a>(str: &'a [u8;8]) -> Result<String, std::string::FromUtf8Error> {
    match String::from_utf8(str.to_vec())  {
        Ok(str) => Ok(str.trim_end_matches('\0').to_string()),
        Err(error) => Err(error)
    }
}

// PatchColumn
impl<'a> PatchColumn<'a> {
    // Slices method
    pub fn slices(&self) -> &Vec<PatchColumnData<'a>> {
        &self.0
    }
}

// PatchContent
impl<'a> PatchContent<'a> {
    // Slices method
    pub fn slices(&self) -> &'a [u32] {
        &self.0
    }
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
        let mut data_textures: DataTextures<'a> = DataTextures {
            reader: reader.clone(),
            palettes: vec![], 

            raw_flats: vec![], 
            flats: vec![], 
            
            patch_names: vec![], 
            sprite_patches: vec![], 
            texture_patches: vec![], 

            sprites: vec![], 
            textures: vec![], 
        };
        if let Some(directories) = data_textures.reader.directories() {
            if let Some(palettes_id) = directories.index_of(&String::from("PLAYPAL")) {
                data_textures.palettes = data_textures.extract::<Palette>(&directories[palettes_id]);
                data_textures.raw_flats = data_textures.extract_a_set(&directories, String::from("F_START"), String::from("F_END"));
                data_textures.patch_names = data_textures.extract_from_name::<[u8;8]>(&directories, String::from("PNAMES"));
                data_textures.sprite_patches = data_textures.extract_sprite_patches(&directories, String::from("S_START"), String::from("S_END"));
                data_textures.texture_patches = data_textures.extract_texture_patches(&directories);
                // Build images:
                if !data_textures.palettes.is_empty() {
                    data_textures.build_flats(&data_textures.palettes[0]);
                    data_textures.build_sprites(&data_textures.palettes[0]);
                }
                
                return Some(data_textures);
            }
        }
        return None;
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

    fn patch_is_sky(pname: &[u8;8]) -> bool {
        match pname {
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
        }
    }
    
    fn extract_patch(&self, directories: &wad::DirectoryList, name: &[u8; 8]) -> Option<Patch<'a>> {
        if let Some(patch_id) = directories.index_of(&u8str_to_string(&name).ok().unwrap_or_default()) {
            let buffer: &Vec<u8> = &self.reader.buffer;
            let directory = &directories[patch_id];
            let lump_offset = directory.start();
            // Read
            let header: &'a PatchHeader = unsafe { mem::transmute(&buffer[lump_offset]) };
            let content_offset = lump_offset + size_of::<PatchHeader>();
            let width = header.size[0] as usize;
            let content: PatchContent<'a> = PatchContent(unsafe {
                std::slice::from_raw_parts(buffer[content_offset..].as_ptr() as *const u32, width)
            });
            // Create patch
            let patch: Patch<'a> = Patch {    
                columns : {
                    let mut columns_t = vec![];
                    for column_offset in content.slices() {
                        let datas = {
                            let mut data_t: Vec<PatchColumnData<'_>> = vec![];
                            let mut offset = lump_offset + *column_offset as usize;
                            loop {
                                let column_header: &'a PatchColumnHeaderData = unsafe { mem::transmute(&buffer[offset]) };
                                //println!("column_header: {:?}", &column_header);
                                if column_header.y_offset == 0xFF { break; }
                                let data_size = column_header.length as usize;
                                let data_start = offset + size_of::<PatchColumnHeaderData>();                       
                                let data_start_end = data_start + data_size; 
                                let column_buffer:&'a [u8] =  unsafe { mem::transmute(&buffer[data_start..data_start_end]) }; 
                                let patch_col_data: PatchColumnData<'a> = PatchColumnData { 
                                    header: column_header,
                                    data: Some(column_buffer)
                                };
                                data_t.push(patch_col_data);
                                offset = data_start_end + 1;
                            }
                            data_t
                        };
                        columns_t.push(PatchColumn(datas));
                    }
                    columns_t
                },        
                name: name.clone(), 
                header: header, 
                content: content, 
                is_sky: DataTextures::patch_is_sky(&name),
            };
            return Some(patch);
        }            
        return None;
    }
    
    fn extract_sprite_patches(&self, directories: &wad::DirectoryList, start: String, end: String) ->  Vec<Patch<'a>>  {
        let mut vec_t = vec![];   
        if let Some(start_id) = directories.index_of(&start) {
            if let Some(end_id) = directories.index_of(&end) {
                for id in start_id+1..end_id {
                    let name = &directories[id].lump_name;
                    if let Some(patch) = self.extract_patch(directories, &name) {
                        vec_t.push(patch);
                    }
                }
            }
        }
        vec_t
    }

    fn extract_texture_patches(&self, directories: &wad::DirectoryList) ->  Vec<Patch<'a>>  {
        let mut vec_t = vec![];   
        for name in &self.patch_names {
            if let Some(patch) = self.extract_patch(directories, name) {
                vec_t.push(patch);
            }
        }
        vec_t
    }

    fn build_patch_as_texture(&self, patch: &Patch, palette: &Palette) -> Texture<4> {
        let texture = Texture {
            size: Vector2::new(patch.header.size[0], patch.header.size[1]),
            colors: {
                let width = patch.header.size[0] as usize;
                let height = patch.header.size[1] as usize;
                let size = width * height;
                let mut texture_data: Vec<[u8; 4]> = vec![[0,0,0,0]; size];
                for x in 0..width {
                    let column = &patch.columns[x];
                    for column_data in column.slices().iter() {
                        let data = column_data.data as Option<&'_ [u8]>;
                        if let Some(values) = data {
                            let mut y = column_data.header.y_offset as usize;
                            for pidx in values {
                                let palette_id = *pidx as usize;
                                texture_data[y * width + x][0..3].copy_from_slice(&(*palette)[palette_id]);
                                texture_data[y * width + x][3] = 0xFF;
                                y += 1;
                            }
                        }
                    }
                }
                texture_data
            }
        };
        texture
    }

    fn build_sprites(&mut self, palette: &Palette) {
        self.sprites.reserve(self.sprite_patches.len());
        for patch in &self.sprite_patches {
            self.sprites.push(self.build_patch_as_texture(patch, palette));
        }
    }

}