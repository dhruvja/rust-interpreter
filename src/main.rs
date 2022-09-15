
pub mod bytecode_interpreter;
pub mod bytecode_types;
pub mod file_line_count;
pub mod tests;

use file_line_count::find_files_with_extension_and_count_lines;

fn main() {
    let directory_path = "./test";
    let extension = "txt";

    find_files_with_extension_and_count_lines(directory_path, extension);
}
