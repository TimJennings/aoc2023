use std::fs;

pub fn read_file_to_vec(file_path: &str) -> Vec<String> {
    let contents = fs::read_to_string(file_path).expect("error reading file");

    let lines: Vec<String> = contents.split('\n').map(|s| String::from(s)).collect();

    lines
}

pub fn read_file_to_string(file_path: &str) -> String {
    fs::read_to_string(file_path).expect("error reading file")
}
