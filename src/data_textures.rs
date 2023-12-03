// Global
use std::{mem, mem::size_of, rc::Rc, ops::Index};

// Engine
use crate::wad::{self, Directory};
use crate::math::Vector2;

#[repr(packed)]
#[allow(dead_code)]
#[derive(Debug)]
#[readonly::make]
pub struct Palette([[u8; 3]; 256]);

#[repr(packed)]
#[allow(dead_code)]
#[readonly::make]
pub struct PNames {
    number_of_names: u32,
    names: [[u8; 8]; 0]
}

#[repr(packed)]
#[allow(dead_code)]
#[derive(Debug)]
#[readonly::make]
pub struct RawFlats([u8; 64 * 64]);

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
#[derive(Clone, Debug)]
#[readonly::make]
pub struct PatchMap {
    pub origin: [i16; 2],
    pub patch_id: u16, // Id of the patch in the PNAME
    pub stepdir: u16,  // Draw mode, normally or mirrored
    pub color_map: u16 // Pallete to be used
}

#[repr(packed)]
#[allow(dead_code)]
#[readonly::make]
pub struct TextureMap {
    pub name: [u8; 8],
    pub flags: u32, // C Boolean, aka a int
    pub size: [u16;2], // WxH, short integer
    __unusted__: u32, // Unused field, integer
    pub patch_map_count: u16, // number of patch map
    pub patch_maps: [PatchMap; 0]
}

#[repr(packed)]
#[allow(dead_code)]
#[readonly::make]
pub struct TextureHeader {
    pub number_of_textures: u32,
    pub offsets: [u32; 0],
}

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
    // Sprites
    pub sprite_patches: Vec<Patch<'a>>,
    pub sprites: Vec<Texture<4>>,
    // Texture (walls)
    pub texture_patch_names: Option<&'a PNames>,
    pub texture_patches: Vec<Patch<'a>>,
    pub texture_maps: Vec<&'a TextureMap>,
    pub textures: Vec<Texture<4>>,
}

// PatchName
fn u8str_to_string<'a>(str: &'a [u8;8]) -> Result<String, std::string::FromUtf8Error> {
    match String::from_utf8(str.to_vec())  {
        Ok(str) => Ok(str.trim_end_matches('\0').to_string()),
        Err(error) => Err(error)
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

// Pnames
impl PNames {
    pub fn iter(&self) -> std::slice::Iter<'_,[u8; 8]> {
        unsafe {
            // Convert this struct into a array buffer
            let names_ptr: *const [u8; 8] = mem::transmute(&self.names);
            // Convert and jump first element (number_of_textures)
            std::slice::from_raw_parts(
                names_ptr, 
                self.number_of_names as usize
            ).iter()
        }
    }
}

// Implement RawFlats
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

// Texture map
impl TextureHeader {
    pub fn size_of(&self) -> usize {
        size_of::<TextureHeader>() + (self.number_of_textures as usize) * size_of::<usize>()
    }
    
    pub fn iter<'a>(&'a self) -> std::slice::Iter<'_,u32> {
        let number_of_textures = self.number_of_textures;
        unsafe {
            // Convert this struct into a array buffer
            let offsets_ptr: *const u32 = mem::transmute(self);
            // Convert and jump first element (number_of_textures)
            std::slice::from_raw_parts(
                offsets_ptr, 
                (number_of_textures + 1) as usize
            )[1..].iter()
        }
    }
}

impl TextureMap {
    pub fn size_of(&self) -> usize {
        size_of::<TextureMap>() + (self.patch_map_count as usize) * size_of::<PatchMap>()
    }

    pub fn iter(&self) -> std::slice::Iter<'_,PatchMap> {
        unsafe {
            // Convert this struct into a array buffer
            let patch_maps_ptr: *const PatchMap = mem::transmute(&self.patch_maps);
            // Convert and jump first element (number_of_textures)
            std::slice::from_raw_parts(
                patch_maps_ptr, 
                self.patch_map_count as usize
            ).iter()
        }
    }
}

// Implement DataTextures
impl<'a> DataTextures<'a> {
    pub fn new(reader: &Rc<wad::Reader>) -> Option<Self> {
        let mut data_textures: DataTextures<'a> = DataTextures {
            reader: reader.clone(),
            palettes: vec![], 
            // Flats (bottom, top textures)
            raw_flats: vec![], 
            flats: vec![], 
            // Sprites (player, gunes, items, etc...)
            sprite_patches: vec![], 
            sprites: vec![], 
            // Textures (walls)
            texture_patch_names: None, 
            texture_patches: vec![],            
            texture_maps: vec![],
            textures: vec![], 
        };
        if let Some(directories) = data_textures.reader.directories() {
            if let Some(palettes_id) = directories.index_of(&String::from("PLAYPAL")) {
                // Palettes
                data_textures.palettes = data_textures.extract_vec::<Palette>(&directories[palettes_id]);
                // Flats
                data_textures.raw_flats = data_textures.extract_a_set(&directories, String::from("F_START"), String::from("F_END"));
                // Sprites
                data_textures.sprite_patches = data_textures.extract_sprite_patches(&directories, String::from("S_START"), String::from("S_END"));
                // Textures
                data_textures.texture_patch_names = data_textures.extract::<PNames>(&directories, String::from("PNAMES"));
                data_textures.texture_patches = data_textures.extract_texture_patches(&directories);
                data_textures.texture_maps = data_textures.extract_texture_maps(&directories, String::from("TEXTURE1"));
                // Build images:
                if !data_textures.palettes.is_empty() {
                    data_textures.build_flats(&data_textures.palettes[0]);
                    data_textures.build_sprites(&data_textures.palettes[0]);
                    data_textures.build_textures();
                }
                
                return Some(data_textures);
            }
        }
        return None;
    }   

    // Basic
    fn extract<T>(&self, directories: &wad::DirectoryList, name: String) -> Option<&'a T> {
        if let Some(directory_id) = directories.index_of(&name) {
            let buffer = &self.reader.buffer;
            let directory = directories[directory_id];
            let value: &'a T = unsafe { mem::transmute(&buffer[directory.start()]) };
            return Some(value);
        }
        return None;
    }

    fn extract_vec<T>(&self, directory: &wad::Directory) -> Vec<&'a T> {
        let buffer = &self.reader.buffer;
        let mut vec_t = vec![];   
        for chunk_offset in directory.data::<T>() {
            let value: &'a T = unsafe { mem::transmute(&buffer[chunk_offset]) };
            vec_t.push(value);
        }
        return vec_t;
    }

    fn extract_vec_from_name<T>(&self, directories: &wad::DirectoryList, name: String) -> Vec<&'a T> {
        let mut vec_t = vec![];   
        if let Some(directory_id) = directories.index_of(&name) {
            let directory = directories[directory_id];
            let buffer = &self.reader.buffer;
            for chunk_offset in directory.data::<T>() {
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
                    vec_t.extend(self.extract_vec::<T>(&directories[id]));
                }
            }
        }
        vec_t
    }

    // Flats
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

    // Sprite & Texture
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
    
    // Sprites
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

    // Textures
    fn extract_patch(&self, directories: &wad::DirectoryList, name: &[u8; 8]) -> Option<Patch<'a>> {
        let str_name = u8str_to_string(&name).ok().unwrap_or_default();
        if let Some(patch_id) = directories.index_of(&str_name) {
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

    fn extract_texture_patches(&self, directories: &wad::DirectoryList) ->  Vec<Patch<'a>>  {
        let mut vec_t = vec![];   
        if let Some(pnames) = self.texture_patch_names {
            for name in pnames.iter() {
                if let Some(patch) = self.extract_patch(directories, name) {
                    vec_t.push(patch);
                }
            }
        }
        vec_t
    }

    fn extract_texture_maps(&self, directories: &wad::DirectoryList, texture_pack_name: String) -> Vec<&'a TextureMap> {
        // Output
        let mut vec_t = vec![];
        // Just if present
        if let Some(directory_id) = directories.index_of(&texture_pack_name) {
            let directory = &directories[directory_id];
            // Ref to buffer
            let buffer: &Vec<u8> = &self.reader.buffer;
            let lump_offset = directory.start();
            // Get all textures
            let header_texture_pack: &'a TextureHeader = unsafe { mem::transmute(&buffer[lump_offset]) };
            // For each texture read texture map
            for texture_map_offset in header_texture_pack.iter() {
                let texture_map : &'a TextureMap = unsafe { mem::transmute(&buffer[lump_offset + *texture_map_offset as usize]) };
                vec_t.push(texture_map);
            }
        }
        vec_t        
    }

    fn build_textures(&mut self) {
        for texture_map in &self.texture_maps {
            self.textures.push(Texture {
                size: Vector2::new(texture_map.size[0], texture_map.size[1]),
                colors : {
                    // Texture
                    let texture_width = texture_map.size[0] as i16;
                    let texture_height = texture_map.size[1] as i16;
                    let texture_size = (texture_width) as usize * texture_height as usize;
                    let mut texture_data: Vec<[u8; 4]> = vec![[0,0,0,0]; texture_size];
                    // Fill
                    for patch_map in texture_map.iter() {
                        let patch = &self.texture_patches[patch_map.patch_id as usize];
                        let palette = self.palettes[patch_map.color_map as usize];
                        let mut texture_data_x = patch_map.origin[0];
                        for column in &patch.columns {
                            if texture_data_x >= 0 {
                                for column_data in column.slices().iter() {
                                    let data = column_data.data as Option<&'_ [u8]>;
                                    if let Some(values) = data {
                                        let mut texture_data_y = column_data.header.y_offset as i16 + patch_map.origin[1];
                                        for pidx in values {
                                            if texture_data_y < 0 {
                                                texture_data_y += 1;
                                                continue;
                                            }
                                            else if texture_data_y >= texture_height {
                                                break;
                                            }
                                            let palette_id = *pidx as usize;
                                            let texture_idx = (texture_data_y * texture_width + texture_data_x) as usize;
                                            texture_data[texture_idx][0..3].copy_from_slice(&(*palette)[palette_id]);
                                            texture_data[texture_idx][3] = 0xFF;
                                            texture_data_y += 1;
                                        }
                                    }
                                }
                            }
                            texture_data_x += 1;
                            if texture_data_x >= texture_width as i16 {
                                break;
                            }
                        }
                    }
                    texture_data
                }
            });
        }
    }
}