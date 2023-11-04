use std::{mem, vec};
use std::rc::Rc;
use crate::math;
use crate::math::Vec2;
use crate::wad;

#[repr(packed)]
#[allow(dead_code)]
#[derive(Debug)]
#[readonly::make]
pub struct Thing {
    pub position: Vec2<i16>,
    pub angle: u16,
    pub thing_type: u16, 
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
    pub right_sidedef :u16,
    pub left_sidedef :u16,
}

// Def a vertex
pub type Vertex = math::Vec2<i16>;

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

#[repr(packed)]
#[allow(dead_code)]
#[derive(Clone, Copy, Debug)]
#[readonly::make]
pub struct NodeBox {
    pub top: i16,
    pub bottom: i16,
    pub left: i16,
    pub right: i16
}

#[repr(packed)]
#[allow(dead_code)]
#[derive(Debug)]
#[readonly::make]
pub struct Node {
    pub partition: Vec2<i16>,
    pub change_partition: Vec2<i16>,
    pub right_box: NodeBox,
    pub left_box: NodeBox,
    pub right_child_id: u16,
    pub left_child_id:  u16,
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
        reader : Rc<wad::Reader>,
    pub things : Vec<&'a Thing>,
    pub line_defs : Vec<&'a LineDef>,
    pub vertices : Vec<&'a Vertex>,
    pub sectors : Vec<&'a Seg>,
    pub sub_sectors : Vec<&'a SubSector>,
    pub nodes : Vec<&'a Node>,
}

impl<'a> Map<'a> {
    pub fn new(reader: &Rc<wad::Reader>, name: &String) -> Option<Self> {
        if let Some(directories) = reader.directories() {
            if let Some(map_dir_id) = directories.index_of(&name) {
                let mut map = Map {
                    reader: reader.clone(),
                    things: vec![], 
                    line_defs: vec![], 
                    vertices: vec![], 
                    sectors: vec![], 
                    sub_sectors: vec![],  
                    nodes: vec![], 
                };
                map.things = map.extract::<Thing>(&directories[map_dir_id + MAPLUMPSINDEX::THINGS as usize]);
                map.line_defs = map.extract::<LineDef>(&directories[map_dir_id + MAPLUMPSINDEX::LINEDEFS as usize]);
                map.vertices = map.extract::<Vertex>(&directories[map_dir_id + MAPLUMPSINDEX::VERTEXES as usize]);
                map.sectors = map.extract::<Seg>(&directories[map_dir_id + MAPLUMPSINDEX::SEGS as usize]);
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