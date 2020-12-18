pub mod helpers {

    use std::{fs::File, path::Path, os};
    use std::io::BufReader;
    use std::io::prelude::*;

    pub fn load_input(filename: String) -> String {
        let file = File::open(Path::new("D:/Programming/rust/aoc2020/src/input/").join(filename)).expect("File not found");
        let mut buf_reader = BufReader::new(file);
        let mut contents = String::new();
        buf_reader.read_to_string(&mut contents).expect("Cant read file");
        
        return contents;
    }

}
