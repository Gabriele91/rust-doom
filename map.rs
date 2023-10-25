use std::mem;
use crate::math;
use crate::wad;

// Def a vertex
pub type Vertex = math::Vec2<i16>;

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

#[allow(dead_code)]
pub enum MAPLUMPSINDEX {
    THINGS = 1,
    LINEDEFS = 2,
    SIDEDDEFS = 3,
    VERTEXES = 4,
    SEAGS = 5,
    SSECTORS = 6,
    NODES = 7,
    SECTORS = 8,
    REJECT = 9,
    BLOCKMAP = 10,
    COUNT = 11
}

pub struct Map<'a> {
    pub line_defs : Vec<&'a LineDef>,
    pub vertices : Vec<&'a Vertex>,
}

impl<'a> Map<'a> {
    pub fn new(reader: &'a wad::Reader, name: &String) -> Option<Self> {
        if let Some(directories) = reader.directories() {
            if let Some(map_dir_id) = directories.index_of(&name) {
                return Some(Map {
                    line_defs: Map::extract::<LineDef>(&reader.buffer, &directories[map_dir_id + MAPLUMPSINDEX::LINEDEFS as usize]),
                    vertices: Map::extract::<Vertex>(&reader.buffer, &directories[map_dir_id + MAPLUMPSINDEX::VERTEXES as usize]),
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