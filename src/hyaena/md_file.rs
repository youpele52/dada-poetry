use std::fs::File;
use std::io::Read;


pub fn read_md_file(filename: &str) -> std::io::Result<String> {
    let mut file: File = match File::open(filename) {
        Ok(file) => file,
        Err(err) => {
            println!("\n\nError opening file: {} \n{}\n", &filename, err);
            return Err(err);
        }
    };

    let mut contents: String = String::new();
    match file.read_to_string(&mut contents) {
        Ok(_) => Ok(contents),
        Err(err) => {
            println!("\n\nError reading file: {} \n{}\n", &filename, err);
            Err(err)
        }
    }
}
