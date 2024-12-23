use std::fs::File;
use std::io::Write;

mod consts {
    pub const UNKNOWN_FLAT : &'static str = "unknown_flat.bin";
    pub const UNKNOWN_TEXTURE : &'static str = "unknown_texture.bin";
}

pub fn build_textures(output:&String) {
    build_flat_unknown( &format!("{}/{}", &output, &consts::UNKNOWN_FLAT));
    build_texture_unknown( &format!("{}/{}", &output, &consts::UNKNOWN_TEXTURE));
}

fn build_flat_unknown(output:&String) {
    let mut colors: Vec<[u8; 3]> = vec![[255, 165, 0]; 64 * 64];
    let start_x = 10;
    let start_y = 28;
    let font: [&[u8]; 7] = [
        b" #  #  #   # #   # #   #  ##  #     # #   #",
        b" #  #  ##  # #  #  ##  # #  # #     # ##  #",
        b" #  #  # # # # #   # # # #  # #     # # # #",
        b" #  #  # # # ##    # # # #  # #  #  # # # #",
        b" #  #  #  ## # #   #  ## #  # #  #  # #  ##",
        b" #  #  #   # #  #  #   # #  # #  #  # #   #",
        b"  ##   #   # #   # #   #  ##   ## ##  #   #",
    ];

    for x in 1..63 {
        colors[64 + x] = [255, 255, 255];
        colors[64 * 62 + x] = [255, 255, 255]
    }

    for y in 1..63 {
        colors[y * 64 + 1] = [255, 255, 255];
        colors[y * 64 + 62] = [255, 255, 255];
    }

    for (row, line) in font.iter().enumerate() {
        for (col, &byte) in line.iter().enumerate() {
            if byte == b'#' {
                let x = start_x + col;
                let y = start_y + row;
                if x < 64 && y < 64 {
                    colors[(64-y) * 64 + x] = [255, 255, 255];
                }
            }
        }
    }

    if let Ok(mut file) = File::create(&output) {
        for color in colors.iter() {
            file.write_all(color).unwrap();
        }
    }
}

fn build_texture_unknown(output:&String) {
    let mut colors: Vec<[u8; 4]> = vec![[255, 165, 0, 255]; 64 * 64];
    let start_x = 10;
    let start_y = 28;
    let font: [&[u8]; 7] = [
        b" #  #  #   # #   # #   #  ##  #     # #   #",
        b" #  #  ##  # #  #  ##  # #  # #     # ##  #",
        b" #  #  # # # # #   # # # #  # #     # # # #",
        b" #  #  # # # ##    # # # #  # #  #  # # # #",
        b" #  #  #  ## # #   #  ## #  # #  #  # #  ##",
        b" #  #  #   # #  #  #   # #  # #  #  # #   #",
        b"  ##   #   # #   # #   #  ##   ## ##  #   #",
    ];

    for x in 1..63 {
        colors[64 + x] = [255, 255, 255, 255];
        colors[64 * 62 + x] = [255, 255, 255, 255]
    }

    for y in 1..63 {
        colors[y * 64 + 1] = [255, 255, 255, 255];
        colors[y * 64 + 62] = [255, 255, 255, 255];
    }

    for (row, line) in font.iter().enumerate() {
        for (col, &byte) in line.iter().enumerate() {
            if byte == b'#' {
                let x = start_x + col;
                let y = start_y + row;
                if x < 64 && y < 64 {
                    colors[(64-y) * 64 + x] = [255, 255, 255, 255];
                }
            }
        }
    }

    if let Ok(mut file) = File::create(&output) {
        for color in colors.iter() {
            file.write_all(color).unwrap();
        }
    }
}
