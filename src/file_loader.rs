use std::fs;

pub fn load_file(name: &str) -> String {
    let path: String = "resources/".to_owned();
    let file = path + name;
    println!("Reading from file: {:?}", file);

    let contents: String = fs::read_to_string(file)
        .expect("Something went wrong reading the file");

    return contents;
}