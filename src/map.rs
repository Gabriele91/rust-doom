#![allow(dead_code)]
use std::ops::Range;
use std::{mem, vec};
use std::rc::Rc;
use crate::configure;
use crate::math::{Vector2, Vector4};
use crate::wad::{self, Directory};

// Consts
const DOOM_BAM_SCALE : f64 = 360.0 / 4294967296.0; // 8.38190317e-8

#[repr(packed)]
#[allow(dead_code)]
#[derive(Debug)]
#[readonly::make]
#[repr(C)]
pub struct Thing {
    pub position: Vector2<i16>,
    pub angle: u16,
    pub type_id: u16, 
    pub flags: u16,
}

// Def a Line
#[repr(packed)]
#[allow(dead_code)]
#[derive(Debug)]
#[readonly::make]
#[repr(C)]
pub struct LineDef {
    pub start_vertex_id: u16,
    pub end_vertex_id: u16,
    pub flag: u16, 
    pub line_type: u16, 
    pub sector_tag :u16,
    pub right_sidedef_id :u16,
    pub left_sidedef_id :u16,
}

// Def of a side
#[repr(packed)]
#[allow(dead_code)]
#[derive(Debug)]
#[readonly::make]
#[repr(C)]
pub struct SideDef {
    pub offset: Vector2<i16>,
    pub upper_texture: [u8; 8],
    pub lower_texture: [u8; 8],
    pub middle_texture: [u8; 8],
    pub sector_id: i16 
}

// Def a vertex
pub type Vertex = Vector2<i16>;

#[repr(packed)]
#[allow(dead_code)]
#[derive(Debug)]
#[readonly::make]
#[repr(C)]
pub struct SubSector {
    pub seg_count: u16,
    pub first_seg_id: u16,
}

#[repr(packed)]
#[allow(dead_code)]
#[derive(Debug)]
#[readonly::make]
#[repr(C)]
pub struct Seg {
    pub start_vertex_id: u16, 
    pub end_vertex_id: u16, 
    pub angle: u16,
    pub line_def_id: u16,
    pub direction: u16, // 0 same as linedef, 1 opposite of linedef
    pub offset: u16,    // distance along linedef to start of seg
}

// Def a NodeBox
pub type NodeBox = Vector4<i16>;

#[repr(packed)]
#[allow(dead_code)]
#[derive(Debug)]
#[readonly::make]
#[repr(C)]
pub struct Node {
    pub partition: Vector2<i16>,
    pub change_partition: Vector2<i16>,
    pub right_box: NodeBox,
    pub left_box: NodeBox,
    pub right_child_id: u16,
    pub left_child_id:  u16,
}

#[repr(packed)]
#[allow(dead_code)]
#[derive(Debug)]
#[readonly::make]
#[repr(C)]
pub struct Sector {
    pub floor_height: i16,
    pub ceiling_height: i16,
    pub floor_texture: [u8; 8],
    pub ceiling_texture: [u8; 8],
    pub light_level: i16,
    pub special_type: i16,
    pub tag_number: i16
}

#[repr(packed)]
#[allow(dead_code)]
#[derive(Debug)]
#[readonly::make]
#[repr(C)]
pub struct BlockmapsHeader {
    pub x: i16,
    pub y: i16,
    pub columns: u16,
    pub rows: u16
}

#[allow(dead_code)]
#[derive(Debug)]
#[readonly::make]
pub struct Blockmaps<'a> {
    pub header: &'a BlockmapsHeader,
    pub metrix_lines: Vec<Rc<Vec<&'a LineDef>>>
}

#[allow(dead_code)]
#[repr(usize)]
pub enum MapLumpsIndex {
    Things = 1,
    LineDefs = 2,
    SideDefs = 3,
    Vertexes = 4,
    Segs = 5,
    SubSectors = 6,
    Nodes = 7,
    Sectors = 8,
    Reject = 9,
    Blockmap = 10,
    MaxSize = 11
}

pub struct MapLumpIndexs {
    things: Option<usize>,
    linedefs: Option<usize>,
    sideddefs: Option<usize>,
    vertexes: Option<usize>,
    segs: Option<usize>,
    ssectors: Option<usize>,
    nodes: Option<usize>,
    sectors: Option<usize>,
    reject: Option<usize>,
    blockmap: Option<usize>,
}

#[repr(u16)]
#[warn(non_camel_case_types)]
pub enum LineDefFlags {
    Blocking = 1, 
    BlockMonsters = 2, 
    TwoSided = 4, 
    DontPegTop = 8,
    DontPegBottom = 16, 
    Secret = 32,
    SoundBlock = 64,
    DontDraw = 128, 
    Mapped = 256
}

#[derive(Clone)]
pub struct Map<'a> {
        reader: Rc<wad::Reader>,
    pub things: Vec<&'a Thing>,
    pub line_defs: Vec<&'a LineDef>,
    pub side_defs: Vec<&'a SideDef>,
    pub vertices: Vec<&'a Vertex>,
    pub segs: Vec<&'a Seg>,
    pub sub_sectors: Vec<&'a SubSector>,
    pub nodes: Vec<&'a Node>,
    pub sectors: Vec<&'a Sector>,
    pub blockmaps: Option<Rc< Blockmaps<'a> >>
}

impl LineDefFlags {
    pub fn value(self) -> u16 {
        self as u16
    }
}

impl MapLumpsIndex {
    pub fn value(self) -> usize {
        self as usize
    }

    pub fn as_name(&self) -> &[u8; 8] {
        match self {
            MapLumpsIndex::Things    => &b"THINGS\0\0",
            MapLumpsIndex::LineDefs  => &b"LINEDEFS",
            MapLumpsIndex::SideDefs => &b"SIDEDEFS",
            MapLumpsIndex::Vertexes  => &b"VERTEXES",
            MapLumpsIndex::Segs      => &b"SEGS\0\0\0\0",
            MapLumpsIndex::SubSectors  => &b"SSECTORS",
            MapLumpsIndex::Nodes     => &b"NODES\0\0\0",
            MapLumpsIndex::Sectors   => &b"SECTORS\0",
            MapLumpsIndex::Reject    => &b"REJECT\0\0",
            MapLumpsIndex::Blockmap  => &b"BLOCKMAP",
            MapLumpsIndex::MaxSize     => &b"--------",
        }
    }
}

fn map_lump_indexs_get_id(directories: &wad::DirectoryList, map_lump_id: usize, name:&[u8;8]) -> Option<usize> {
    for lump_id in map_lump_id+1..directories.len() {
        // If a lump has size 0 or follows the map naming convention (e.g., MAPxx or ExMx),
        // it is a new map, so the following lumps are not associated with the input map.
        if directories[lump_id].lump_size == 0 && (
            directories[lump_id].lump_name.starts_with(b"MAP") || 
            (directories[lump_id].lump_name[0] == b'E' && directories[lump_id].lump_name[2] == b'M')
        ) {
            return None;
        }
        else if directories[lump_id].lump_name == *name {
            return Some(lump_id);
        }
    }
    return None;
}

impl MapLumpIndexs {
    pub fn new(directories: &wad::DirectoryList, map_lump_id: usize) -> Self {
        MapLumpIndexs {
            things: map_lump_indexs_get_id(&directories, map_lump_id, MapLumpsIndex::Things.as_name()),
            linedefs: map_lump_indexs_get_id(&directories, map_lump_id, MapLumpsIndex::LineDefs.as_name()),
            sideddefs: map_lump_indexs_get_id(&directories, map_lump_id, MapLumpsIndex::SideDefs.as_name()),
            vertexes: map_lump_indexs_get_id(&directories, map_lump_id, MapLumpsIndex::Vertexes.as_name()),
            segs: map_lump_indexs_get_id(&directories, map_lump_id, MapLumpsIndex::Segs.as_name()),
            ssectors: map_lump_indexs_get_id(&directories, map_lump_id, MapLumpsIndex::SubSectors.as_name()),
            nodes: map_lump_indexs_get_id(&directories, map_lump_id, MapLumpsIndex::Nodes.as_name()),
            sectors: map_lump_indexs_get_id(&directories, map_lump_id, MapLumpsIndex::Sectors.as_name()),
            reject: map_lump_indexs_get_id(&directories, map_lump_id, MapLumpsIndex::Reject.as_name()),
            blockmap: map_lump_indexs_get_id(&directories, map_lump_id, MapLumpsIndex::Blockmap.as_name()),
        }
    }
}

impl LineDef {

    const LINEDEFNULL:u16 = 0xFFFF;

    pub fn right_side<'a>(&'a self, map: &Map<'a>) -> Option<&'a SideDef> {
        if self.right_sidedef_id != LineDef::LINEDEFNULL {
            return Some(map.side_defs[self.right_sidedef_id as usize]);
        }
        return None;
    }
    
    pub fn front_side<'a>(&'a self, map: &Map<'a>) -> Option<&'a SideDef> {
        return self.right_side(&map);
    }
    
    pub fn left_side<'a>(&'a self, map: &Map<'a>) -> Option<&'a SideDef> {
        if self.left_sidedef_id != LineDef::LINEDEFNULL {
            return Some(map.side_defs[self.left_sidedef_id as usize]);
        }
        return None;
    }
    
    pub fn back_side<'a>(&'a self, map: &Map<'a>) -> Option<&'a SideDef> {
        return self.left_side(&map);
    }
    
    pub fn start_vertex<'a>(&'a self, map: &Map<'a>) -> &'a Vertex {
        return &map.vertices[self.start_vertex_id as usize];
    }

    pub fn end_vertex<'a>(&'a self, map: &Map<'a>) -> &'a Vertex {
        return &map.vertices[self.end_vertex_id as usize];
    }

    pub fn has_flag(&self, mask:LineDefFlags) -> bool {
        (self.flag & mask.value()) != 0
    }
}

impl SideDef {
    pub fn sector<'a>(&'a self, map: &Map<'a>) -> &'a Sector {
        return &map.sectors[self.sector_id as usize];
    }
}

impl SubSector {
    pub fn iter(&self) -> Range<u16> {
        self.first_seg_id..self.first_seg_id + self.seg_count
    }
}

impl Seg {
    pub fn line_defs<'a>(&'a self, map: &Map<'a>) -> &'a LineDef {
        return &map.line_defs[self.line_def_id as usize];
    }
    
    pub fn right_sector<'a>(&'a self, map: &Map<'a>) -> Option<&'a Sector> {
        let line_defs = self.line_defs(&map);
        if self.direction == 0 {
            if  let Some(side_def) =  line_defs.right_side(map) {
                return Some(side_def.sector(map));
            }
        } else {
            if  let Some(side_def) =  line_defs.left_side(map) {
                return Some(side_def.sector(map));
            }
        } 
        return None;
    }

    pub fn front_sector<'a>(&'a self, map: &Map<'a>) -> Option<&'a Sector> {
        return self.right_sector(&map);
    }

    pub fn left_sector<'a>(&'a self, map: &Map<'a>) -> Option<&'a Sector> {
        let line_defs = self.line_defs(&map);
        if self.direction == 0 {
            if  let Some(side_def) =  line_defs.left_side(map) {
                return Some(side_def.sector(map));
            }
        } else {
            if  let Some(side_def) =  line_defs.right_side(map) {
                return Some(side_def.sector(map));
            }
        } 
        return None;
    }

    pub fn back_sector<'a>(&'a self, map: &Map<'a>) -> Option<&'a Sector> {
        return self.left_sector(&map);
    }

    pub fn start_vertex<'a>(&'a self, map: &Map<'a>) -> &'a Vertex {
        return &map.vertices[self.start_vertex_id as usize];
    }

    pub fn end_vertex<'a>(&'a self, map: &Map<'a>) -> &'a Vertex {
        return &map.vertices[self.end_vertex_id as usize];
    }

    pub fn float_degrees_angle(&self) -> f32 {
        // Conver to i32
        let mut angle_i32: i32 = self.angle as i32;
        angle_i32 <<= 16;
        // Compute degrees angle
        let mut angle_f64 = angle_i32 as f64 * DOOM_BAM_SCALE;
        // [-180,180] -> [0,360]
        if angle_f64 < 0.0 {
            angle_f64 += 360.0;
        }
        // Returns
        angle_f64 as f32
    }
}

impl<'a> Blockmaps<'a> {

    const BLOCKSIZE:i32 = 128;
    const LINELISTSTART:u16 = 0x0;
    const LINELISTEND:u16 = 0xFFFF;

    fn new(directory: &Directory, line_defs: &Vec<&'a LineDef>, buffer:&Vec<u8>, no_first_line: bool) -> Result<Self,String> {
        // Test
        if directory.size() < std::mem::size_of::<BlockmapsHeader>() {
            return Err(String::from("Invalid blockmaps.header"));
        }
        //
        let mut offset = directory.start();
        // Get header
        let header: &'a BlockmapsHeader =  unsafe{ mem::transmute(&buffer[offset]) };
        // Cast values
        let columns = header.columns as usize;
        let rows = header.rows as usize;
        // Test header size
        assert!(std::mem::size_of::<BlockmapsHeader>() == 8);
        // Advance
        offset += std::mem::size_of::<BlockmapsHeader>();
        // Compute metrix size 
        let matrix_size = columns * rows;
        Ok(Blockmaps {
            header: header,
            metrix_lines: {
                let mut metrix: Vec<u16> = Vec::with_capacity(matrix_size);
                for _ in 0..matrix_size {
                    // Test
                    if offset >= directory.end() {
                        return Err(String::from("Invalid blockmaps.offsets"));
                    }
                    // Get offset
                    let list_offset: &u16 = unsafe{ mem::transmute(&buffer[offset]) };
                    metrix.push( *list_offset );
                    offset += std::mem::size_of::<u16>();
                }
                let mut metrix_lines: Vec<Rc<Vec<&'a LineDef>>> = vec![Rc::new(vec![]);matrix_size];
                for y in 0..rows {
                    for x in 0..columns {
                        let list_relative_offset =  metrix[columns * y + x] as usize * std::mem::size_of::<u16>();
                        let mut list_value_offset = directory.start() + list_relative_offset;
                        // List
                        let mut line_def_list: Vec<&'a LineDef> = Vec::new();
                        // Jump first index
                        if no_first_line {
                            // Test size
                            if list_value_offset >= directory.end() {
                                return Err(format!("Invalid blocklists at x: {}, y: {}", x, y));
                            }
                            // Get ID
                            let value:&u16 = unsafe{ mem::transmute(&buffer[list_value_offset]) };
                            if *value != Blockmaps::LINELISTSTART {
                                return Err(format!("Invalid blocklists start (not 0) at x: {}, y: {}", x, y));
                            }
                            // go ahead
                            list_value_offset += std::mem::size_of::<u16>();
                        }
                        // Loop until 0xFF
                        loop {
                            // Test size
                            if list_value_offset >= directory.end() {
                                return Err(format!("Invalid blocklists at x: {}, y: {}", x, y));
                            }
                            // Get ID
                            let value:&u16 = unsafe{ mem::transmute(&buffer[list_value_offset]) };
                            if *value == Blockmaps::LINELISTEND { 
                                break; 
                            }
                            let line_def_id = *value as usize;
                            // Test ID
                            if line_def_id as usize >= line_defs.len() {
                                return Err(format!("Invalid blocklist id: {} at x: {}, y: {}", line_def_id, x, y));
                            }
                            // Save and go ahead
                            line_def_list.push(line_defs[line_def_id]);
                            list_value_offset += std::mem::size_of::<u16>();
                        }
                        metrix_lines[columns * y + x] = Rc::new(line_def_list);
                    }
                }
                metrix_lines
            }
        })
    }

    pub fn get(&self, x: i16, y: i16) -> Option<Rc<Vec<&'a LineDef>>> {
        // Calculate the offset relative to the blockmap origin
        let m_x = (x as i32 - self.header.x as i32) / Blockmaps::BLOCKSIZE;
        let m_y = (y as i32 - self.header.y as i32) / Blockmaps::BLOCKSIZE;
    
        // Check if the coordinates are within the blockmap bounds
        if m_x >= 0 && m_x < (self.header.columns as i32)
        && m_y >= 0 && m_y < (self.header.rows as i32)
        {
            // Calculate block indices based on the blockmap grid size (128x128 units per block)
            let i_x = m_x as usize; // Convert absolute units to block indices
            let i_y = m_y as usize;
    
            let columns = self.header.columns as usize;
            // Return the reference to the linedefs in the specified block
            Some(self.metrix_lines[columns * i_y + i_x].clone())
        } else {
            // Return None if the point is outside the blockmap boundaries
            None
        }
    }

}

impl<'a> Map<'a> {
    pub fn new(reader: &Rc<wad::Reader>, configure: &configure::Map) -> Option<Self> {
        if let Some(directories) = reader.directories() {
            if let Some(map_dir_id) = directories.index_of(&configure.name) {
                let mut map = Map {
                    reader: reader.clone(),
                    things: vec![], 
                    line_defs: vec![], 
                    side_defs: vec![], 
                    vertices: vec![], 
                    segs: vec![], 
                    sub_sectors: vec![],  
                    nodes: vec![], 
                    sectors: vec![], 
                    blockmaps: None
                };
    
                let indexes = MapLumpIndexs::new(&directories, map_dir_id);
    
                if let Some(index) = indexes.things {
                    map.things = map.extract::<Thing>(&directories[index]);
                }
    
                if let Some(index) = indexes.linedefs {
                    map.line_defs = map.extract::<LineDef>(&directories[index]);
                }
    
                if let Some(index) = indexes.sideddefs {
                    map.side_defs = map.extract::<SideDef>(&directories[index]);
                }
    
                if let Some(index) = indexes.vertexes {
                    map.vertices = map.extract::<Vertex>(&directories[index]);
                }
    
                if let Some(index) = indexes.segs {
                    map.segs = map.extract::<Seg>(&directories[index]);
                }

                if let Some(index) = indexes.ssectors {
                    map.sub_sectors = map.extract::<SubSector>(&directories[index]);
                }
    
                if let Some(index) = indexes.nodes {
                    map.nodes = map.extract::<Node>(&directories[index]);
                }
    
                if let Some(index) = indexes.sectors {
                    map.sectors = map.extract::<Sector>(&directories[index]);
                }

                if !map.line_defs.is_empty() {
                    if let Some(index) = indexes.blockmap {
                        match Blockmaps::new(&directories[index], &map.line_defs, &map.reader.buffer, configure.blockmap_no_first_line) {
                            Ok(blockmaps) => map.blockmaps = Some(Rc::new(blockmaps)),
                            Err(err) => eprintln!("{}",err),
                        }
                    }
                }
    
                return Some(map);
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
}