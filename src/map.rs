use std::mem;
use crate::math;
use crate::math::Vec2;
use crate::wad;

#[repr(packed)]
#[allow(dead_code)]
#[derive(Debug)]
pub struct Thing {
    position: Vec2<i16>,
    angle: u16,
    thing_type: u16, 
    flags: u16,
}

// Def a Line
#[repr(packed)]
#[allow(dead_code)]
#[derive(Debug)]
pub struct LineDef {
    start_vertex_: u16,
    end_vertex: u16,
    flag: u16, 
    line_type: u16, 
    sector_tag :u16,
    right_sidedef :u16,
    left_sidedef :u16,
}

// Def a vertex
pub type Vertex = math::Vec2<i16>;

#[repr(packed)]
#[allow(dead_code)]
#[derive(Debug)]
pub struct SubSector {
    seg_count: u16,
    first_seg_id: u16,
}

#[repr(packed)]
#[allow(dead_code)]
#[derive(Debug)]
pub struct Seg {
    start_vertex_id: u16, 
    end_vertex_id: u16, 
    angle: u16,
    line_def_id: u16,
    direction: u16, // 0 same as linedef, 1 opposite of linedef
    offset: u16,    // distance along linedef to start of seg
}

#[repr(packed)]
#[allow(dead_code)]
#[derive(Clone, Copy, Debug)]
pub struct NodeBox {
    top: i16,
    bottom: i16,
    left: i16,
    right: i16
}

#[repr(packed)]
#[allow(dead_code)]
#[derive(Debug)]
pub struct Node {
    partition: Vec2<i16>,
    change_partition: Vec2<i16>,
    right_box: NodeBox,
    left_box: NodeBox,
    right_child_id: u16,
    left_child_id:  u16,
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

#[derive(Debug)]
pub struct Map<'a> {
    pub things : Vec<&'a Thing>,
    pub line_defs : Vec<&'a LineDef>,
    pub vertices : Vec<&'a Vertex>,
    pub sectors : Vec<&'a Seg>,
    pub sub_sectors : Vec<&'a SubSector>,
    pub nodes : Vec<&'a Node>,
}

impl<'a> Map<'a> {
    pub fn new(reader: &'a wad::Reader, name: &String) -> Option<Self> {
        if let Some(directories) = reader.directories() {
            if let Some(map_dir_id) = directories.index_of(&name) {
                return Some(Map {
                    things: Map::extract::<Thing>(&reader.buffer, &directories[map_dir_id + MAPLUMPSINDEX::THINGS as usize]), 
                    line_defs: Map::extract::<LineDef>(&reader.buffer, &directories[map_dir_id + MAPLUMPSINDEX::LINEDEFS as usize]),
                    vertices: Map::extract::<Vertex>(&reader.buffer, &directories[map_dir_id + MAPLUMPSINDEX::VERTEXES as usize]),
                    sectors: Map::extract::<Seg>(&reader.buffer, &directories[map_dir_id + MAPLUMPSINDEX::SEGS as usize]),
                    sub_sectors: Map::extract::<SubSector>(&reader.buffer, &directories[map_dir_id + MAPLUMPSINDEX::SSECTORS as usize]),
                    nodes: Map::extract::<Node>(&reader.buffer, &directories[map_dir_id + MAPLUMPSINDEX::NODES as usize]),
                })
            }
        }
        return None
    }

    fn extract<T>(buffer: &'a [u8], directory: &'a wad::Directory) -> Vec<&'a T> {
        let mut vec_t = vec![];   
        for chunk_offset in directory.data::<T>() {
            let value: &'a T = unsafe { mem::transmute(&buffer[chunk_offset]) };
            vec_t.push(value);
        }
        return vec_t;
    }
}