use walkdir::WalkDir;
use std::fs;

pub fn find_files_with_extension_and_count_lines(directory_path: &str, extension: &str) {
    for file in WalkDir::new(directory_path).into_iter().filter_map(|file| file.ok()) {
        if file.metadata().unwrap().is_file() {
            let file_ext = file.path().extension().unwrap();
            if file_ext == extension {
                let contents = fs::read_to_string(file.path()).expect("Should have been able to read the file");
                println!("File {:?} has {} lines", file.path().file_name().unwrap(),contents.lines().count());
            }
        }
    }
}