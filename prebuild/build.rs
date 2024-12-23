mod textures;

mod consts {
    use lazy_static::lazy_static;
    use std::env;

    // Constant for the prebuild output path
    pub const PREBUILD_OUTPUT: &'static str = "src/prebuild";

    // A vector of tuples (path, build_fn) where:
    // `str` is the output path
    // `build_fn` is a lambda function used to build assets
    pub const PREBUILD_DIRECTORIES: &'static [(&'static str, fn(&String))] = &[
        ("textures", |output| crate::textures::build_textures(output)),
    ];

    // Lazy initialization for the base project path
    lazy_static! {
        pub static ref ENV_PATH: String = env::var("CARGO_MANIFEST_DIR").unwrap_or_else(|_| String::from(""));

        // Compute the full paths for each prebuild directory
        pub static ref PREBUILD_PATHS: Vec<String> = {
            let base_path = format!("{}/{}", *ENV_PATH, PREBUILD_OUTPUT);
            PREBUILD_DIRECTORIES.iter().map(|&(dir, _)| format!("{}/{}", base_path, dir)).collect()
        };
    }
}

mod utils {
    use std::fs;
    use std::path::Path;

    // Function to ensure the directory exists, creating it if it doesn't
    fn ensure_path_exists(path: &str) -> Result<bool, std::io::Error> {
        let path = Path::new(path);
        if !path.exists() {
            fs::create_dir_all(path)?;
            Ok(true)
        } else {
            Ok(false)
        }
    }

    // Function to build all required prebuild directories
    pub fn build_dirs() -> Result<(), std::io::Error> {
        for dir in crate::consts::PREBUILD_PATHS.iter() {
            if let Err(e) = ensure_path_exists(&dir) {
                eprintln!("Error creating directory {}: {}", dir, e);
                return Err(e);
            } else {
                println!("Directory created: {}", dir);
            }
        }
        Ok(())
    }
}

fn main() {
    println!("Running build.rs...");

    // Build the necessary directories
    if let Err(e) = utils::build_dirs() {
        eprintln!("Error building directories: {}", e);
        std::process::exit(1);
    }

    // Execute the build logic for each resource
    for &(path, build_fn) in crate::consts::PREBUILD_DIRECTORIES {
        let resource_path = &crate::consts::PREBUILD_PATHS.iter()
            .find(|&p| p.contains(path))
            .expect("Path not found");
        // Call the build function (lambda) for the resource
        build_fn(resource_path);
    }
}
