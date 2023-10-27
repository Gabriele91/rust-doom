// Define
mod wad;
mod map;
mod math;

fn main() {
    if let Some(doom1) = wad::Reader::new(String::from("assets/DOOM1.WAD")) {
        if let Some(map) = map::Map::new(&doom1, &String::from("E1M1")) {  
            println!("{:?}", map);
        }
    }
}