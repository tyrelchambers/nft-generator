use std::path::{Path, PathBuf};

static BASE_DIR: &'static str = "build";

fn create_directories(paths: &[&Path]) {
    for path in paths {
        let path = Path::new(path);

        match path.is_dir() {
            true => {
                println!("{} already exists", path.display());
            }
            false => {
                println!("Creating {}", path.display());
                std::fs::create_dir(path).unwrap();
            }
        }
    }
}

pub fn build_setup() {
    let path = Path::new(BASE_DIR);
    let json_path = Path::new(&path).join("json");
    let images = Path::new(&path).join("images");

    match path.is_dir() {
        true => {
            println!("{} already exists - recreating.", BASE_DIR);
            std::fs::remove_dir_all(path).unwrap();
            create_directories(&[path, &json_path, &images]);
        }
        false => {
            println!("Creating build directories directories");
            create_directories(&[path, &json_path, &images]);
        }
    }
}
