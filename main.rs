// Define
mod wad;
mod map;
mod math;

fn main() {
    if let Some(doom1) = wad::Reader::new("assets/DOOM1.WAD") {
        if let Some(map) = map::Map::new(&doom1, &String::from("E1M1")) {  
            for vertex in map.vertices {
                println!("vertex: {:?}", &vertex);
            }
            for line_def in map.line_defs {
                println!("line_def: {:?}", &line_def);
            }
        }
    }
}