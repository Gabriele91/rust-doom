#![allow(dead_code)]
use std::ops::Range;
use std::{mem, vec};
use std::rc::Rc;
use lazy_static::lazy_static;

use crate::math::{Vector2, Vector4};
use crate::wad;

// Consts
const DOOM_BAM_SCALE : f64 = 360.0 / 4294967296.0; // 8.38190317e-8

#[repr(packed)]
#[allow(dead_code)]
#[derive(Debug)]
#[readonly::make]
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
pub struct SubSector {
    pub seg_count: u16,
    pub first_seg_id: u16,
}

#[repr(packed)]
#[allow(dead_code)]
#[derive(Debug)]
#[readonly::make]
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
pub struct Blockmaps {
    pub x: i16,
    pub y: i16,
    pub columns: i16,
    pub row: i16
}

#[allow(dead_code)]
#[repr(usize)]
pub enum MAPLUMPSINDEX {
    THINGS = 1,
    LINEDEFS = 2,
    SIDEDDEFS = 3,
    VERTEXES = 4,
    SEGS = 5,
    SSECTORS = 6,
    NODES = 7,
    SECTORS = 8,
    REJECT = 9,
    BLOCKMAP = 10,
    COUNT = 11
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
pub enum LINEDEF_FLAGS {
    BLOCKING = 1, 
    BLOCK_MONSTERS = 2, 
    TWO_SIDED = 4, 
    DONT_PEG_TOP = 8,
    DONT_PEG_BOTTOM = 16, 
    SECRET = 32, 
    SOUND_BLOCK = 64, 
    DONT_DRAW = 128, 
    MAPPED = 256
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
}

impl LINEDEF_FLAGS {
    pub fn value(self) -> u16 {
        self as u16
    }
}

impl MAPLUMPSINDEX {
    pub fn value(self) -> usize {
        self as usize
    }

    pub fn as_name(&self) -> &[u8; 8] {
        match self {
            MAPLUMPSINDEX::THINGS    => &b"THINGS\0\0",
            MAPLUMPSINDEX::LINEDEFS  => &b"LINEDEFS",
            MAPLUMPSINDEX::SIDEDDEFS => &b"SIDEDEFS",
            MAPLUMPSINDEX::VERTEXES  => &b"VERTEXES",
            MAPLUMPSINDEX::SEGS      => &b"SEGS\0\0\0\0",
            MAPLUMPSINDEX::SSECTORS  => &b"SSECTORS",
            MAPLUMPSINDEX::NODES     => &b"NODES\0\0\0",
            MAPLUMPSINDEX::SECTORS   => &b"SECTORS\0",
            MAPLUMPSINDEX::REJECT    => &b"REJECT\0\0",
            MAPLUMPSINDEX::BLOCKMAP  => &b"BLOCKMAP",
            MAPLUMPSINDEX::COUNT     => &b"--------",
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
            things: map_lump_indexs_get_id(&directories, map_lump_id, MAPLUMPSINDEX::THINGS.as_name()),
            linedefs: map_lump_indexs_get_id(&directories, map_lump_id, MAPLUMPSINDEX::LINEDEFS.as_name()),
            sideddefs: map_lump_indexs_get_id(&directories, map_lump_id, MAPLUMPSINDEX::SIDEDDEFS.as_name()),
            vertexes: map_lump_indexs_get_id(&directories, map_lump_id, MAPLUMPSINDEX::VERTEXES.as_name()),
            segs: map_lump_indexs_get_id(&directories, map_lump_id, MAPLUMPSINDEX::SEGS.as_name()),
            ssectors: map_lump_indexs_get_id(&directories, map_lump_id, MAPLUMPSINDEX::SSECTORS.as_name()),
            nodes: map_lump_indexs_get_id(&directories, map_lump_id, MAPLUMPSINDEX::NODES.as_name()),
            sectors: map_lump_indexs_get_id(&directories, map_lump_id, MAPLUMPSINDEX::SECTORS.as_name()),
            reject: map_lump_indexs_get_id(&directories, map_lump_id, MAPLUMPSINDEX::REJECT.as_name()),
            blockmap: map_lump_indexs_get_id(&directories, map_lump_id, MAPLUMPSINDEX::BLOCKMAP.as_name()),
        }
    }
}

impl LineDef {
    pub fn right_side<'a>(&'a self, map: &Map<'a>) -> Option<&'a SideDef> {
        if self.right_sidedef_id != 0xFFFF {
            return Some(map.side_defs[self.right_sidedef_id as usize]);
        }
        return None;
    }
    
    pub fn front_side<'a>(&'a self, map: &Map<'a>) -> Option<&'a SideDef> {
        return self.right_side(&map);
    }
    
    pub fn left_side<'a>(&'a self, map: &Map<'a>) -> Option<&'a SideDef> {
        if self.left_sidedef_id != 0xFFFF {
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

impl<'a> Map<'a> {
    pub fn new(reader: &Rc<wad::Reader>, name: &String) -> Option<Self> {
        if let Some(directories) = reader.directories() {
            if let Some(map_dir_id) = directories.index_of(&name) {
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