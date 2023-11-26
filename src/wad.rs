#![allow(dead_code)]
#![allow(unused_imports)]
use std::iter::StepBy;
use std::path::Path;
use std::mem;
use std::fs;
use std::option::Option;
use std::collections::HashMap;
use std::str::FromStr;


#[repr(packed)]
#[derive(Debug)]
pub struct Header {
    pub wad_type : [u8; 4],
    pub directory_count : u32,
    pub directory_offset : u32,
}

impl std::fmt::Display for Header {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let directory_count = self.directory_count;
        let directory_offset = self.directory_offset;
        write!(f, "Header {{ type: {}, directory_count: {}, directory_offset: {} }}", 
            String::from_utf8_lossy(&self.wad_type),
            directory_count, 
            directory_offset,
        )
    }
}

impl Header {
    pub fn valid(&self) -> bool {
        if let Ok(name) = self.name() {
            return name == "IWAD" || name == "PWAD";
        }
        false
    }

    pub fn name(&self) -> Result<String, std::string::FromUtf8Error> {
        match String::from_utf8(self.wad_type.to_vec())  {
            Ok(str) => Ok(str.trim_end_matches('\0').to_string()),
            Err(error) => Err(error)
        }
    }
}

#[repr(packed)]
#[derive(Debug)]
pub struct Directory {
    pub lump_offset : u32,
    pub lump_size : u32,
    pub lump_name : [u8; 8],
}

impl std::fmt::Display for Directory {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let lump_offset = self.lump_offset;
        let lump_size = self.lump_offset;
        write!(f, "Directory {{ offset: {}, size: {}, name: {} }}", 
            lump_offset, 
            lump_size,
            String::from_utf8_lossy(&self.lump_name)
        )
    }
}

impl Directory {
    pub fn data<'a,T>(&self) -> core::iter::StepBy<std::ops::Range<usize>> {
        let start = self.lump_offset as usize;
        let end = start + self.lump_size as usize;
        let chunk_size = mem::size_of::<T>() as usize;
        (start..end).step_by(chunk_size)
    }

    pub fn data_by_step<'a,T>(&self, offset: usize, step: usize) -> core::iter::StepBy<std::ops::Range<usize>> {
        let start = self.lump_offset as usize + offset;
        let end = start + self.lump_size as usize;
        (start..end).step_by(step)
    }
    
    pub fn name(&self) -> Result<String, std::string::FromUtf8Error> {
        match String::from_utf8(self.lump_name.to_vec())  {
            Ok(str) => Ok(str.trim_end_matches('\0').to_string()),
            Err(error) => Err(error)
        }
    }
}

pub struct DirectoryList<'a> {
    pub directories: Vec<&'a Directory>,
}

impl<'a> DirectoryList<'a> {
    pub fn new(capacity: usize) -> Self {
        let mut directories = vec![];
        directories.reserve(capacity);
        DirectoryList {
            directories
        }
    }

    pub fn push(&mut self, directory: &'a Directory) {
        self.directories.push(directory);
    }

    pub fn index_of(&self, name: &String) -> Option<usize> {
        for dir in self.into_iter().enumerate() {
            if let Ok(dir_name) = dir.1.name() {
                if (*name) == dir_name {
                    return Some(dir.0);
                }
            }
        }
        return None;
    }
}

impl<'a> IntoIterator for &'a DirectoryList<'a> {
    type Item = &'a &'a Directory;
    type IntoIter = std::slice::Iter<'a, &'a Directory>;

    fn into_iter(self) -> Self::IntoIter {
        self.directories.iter()
    }
}

impl<'a> std::ops::Index<usize> for DirectoryList<'a> {
    type Output = &'a Directory;

    fn index(&self, index: usize) -> &Self::Output {
        &self.directories[index]
    }
}

pub struct Reader {
    pub pathfile : String,
    pub buffer : Vec<u8>,
}

impl  Reader {

    pub fn new(wadfile: &String) -> Option<Self> {
        if let Ok(file_buffer) = fs::read(&wadfile) {
            return {
                let reader = Reader {
                    pathfile: wadfile.clone(),
                    buffer: file_buffer,
                };
                if let Some(header) = reader.header() {
                    if header.valid() {
                        return Some(reader);
                    }
                }
                None
            };
        }
        None
    } 

    pub fn header(&self) -> Option<&Header> {
        if mem::size_of::<Header>() <= self.buffer.len() {
            let header: &Header = unsafe { mem::transmute(&self.buffer[0]) };
            return Some(header);
        }
        return None
    }

    pub fn directories<'a>(&'a self) -> Option< DirectoryList<'a> > {
        if  let Some(&ref header) = self.header() {
            let size_of_header_dir = mem::size_of::<Directory>() as usize;
            let start = header.directory_offset as usize;
            let end = start + header.directory_count as usize * size_of_header_dir;
            let mut out: DirectoryList<'a> = DirectoryList::new(header.directory_offset as usize);
            for dir_offset in (start..end).step_by(size_of_header_dir){
                let directory: &'a Directory = unsafe { mem::transmute(&self.buffer[dir_offset]) };
                out.push(directory);
            }
            return Some(out);
        } else {
            None
        }
    }

}
