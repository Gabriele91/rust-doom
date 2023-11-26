#![allow(dead_code)]
use std::ops::Range;
use std::{mem, vec};
use std::rc::Rc;
use crate::math::{Vector2, Vector4, normalize_degrees};
use crate::wad;

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

#[allow(dead_code)]
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

#[derive(Clone)]
pub struct Map<'a> {
        reader: Rc<wad::Reader>,
    pub things: Vec<&'a Thing>,
    pub line_defs: Vec<&'a LineDef>,
    pub side_defs: Vec<&'a SideDef>,
    pub vertices: Vec<&'a Vertex>,
    pub segs: Vec<&'a Seg>,
    pub sectors: Vec<&'a Sector>,
    pub sub_sectors: Vec<&'a SubSector>,
    pub nodes: Vec<&'a Node>
}

impl LineDef {
    pub fn right_side<'a>(&'a self, map: &Map<'a>) -> Option<&'a SideDef> {
        if self.right_sidedef_id != 0xFFFF {
            return Some(map.side_defs[self.right_sidedef_id as usize]);
        }
        return None;
    }
    
    pub fn left_side<'a>(&'a self, map: &Map<'a>) -> Option<&'a SideDef> {
        if self.left_sidedef_id != 0xFFFF {
            return Some(map.side_defs[self.left_sidedef_id as usize]);
        }
        return None;
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

    pub fn start_vertex<'a>(&'a self, map: &Map<'a>) -> &'a Vertex {
        return &map.vertices[self.start_vertex_id as usize];
    }

    pub fn end_vertex<'a>(&'a self, map: &Map<'a>) -> &'a Vertex {
        return &map.vertices[self.end_vertex_id as usize];
    }

    pub fn float_degrees_angle(&self) -> f32 {
        let angle = ((self.angle as u64) << 16) as f64 * 8.38190317e-8;
        normalize_degrees(angle) as f32
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
                    sectors: vec![], 
                    sub_sectors: vec![],  
                    nodes: vec![], 
                };
                map.things = map.extract::<Thing>(&directories[map_dir_id + MAPLUMPSINDEX::THINGS as usize]);
                map.line_defs = map.extract::<LineDef>(&directories[map_dir_id + MAPLUMPSINDEX::LINEDEFS as usize]);
                map.side_defs = map.extract::<SideDef>(&directories[map_dir_id + MAPLUMPSINDEX::SIDEDDEFS as usize]);
                map.vertices = map.extract::<Vertex>(&directories[map_dir_id + MAPLUMPSINDEX::VERTEXES as usize]);
                map.segs = map.extract::<Seg>(&directories[map_dir_id + MAPLUMPSINDEX::SEGS as usize]);
                map.sectors = map.extract::<Sector>(&directories[map_dir_id + MAPLUMPSINDEX::SECTORS as usize]);
                map.sub_sectors = map.extract::<SubSector>(&directories[map_dir_id + MAPLUMPSINDEX::SSECTORS as usize]);
                map.nodes = map.extract::<Node>(&directories[map_dir_id + MAPLUMPSINDEX::NODES as usize]);
                return Some(map)
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
}