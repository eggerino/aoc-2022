use std::{env, fs};

pub fn get_file_content() -> String {
    let filename = env::args()
        .nth(1)
        .expect("Provide a file to read the input from");
    fs::read_to_string(filename).expect("Cannot read the given file")
}
