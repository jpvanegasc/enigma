pub fn parse_file(file_path: &str) -> () {
    match fs::read_to_string(file_path) {
        Ok(contents) => println!("{}", contents),
        Err(e) => eprintln!("Error reading file {}: {}", file_path, e),
    }
}
