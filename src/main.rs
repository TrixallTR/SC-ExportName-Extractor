use std::fs::{self, File};
use std::io::{BufReader, BufWriter, Read, Write};
use std::path::Path;
use memchr::memmem;
use scexport::reader::Reader;

const PATTERN: &[u8] = &[0x1C, 0x00, 0x00, 0x00, 0x04, 0x00];

fn extract(path: &Path) -> String {
    let mut exports = String::new();
    let mut bytes = Vec::new();

    let file = File::open(path).expect("Failed to open file");
    let mut reader = BufReader::new(file);
    reader.read_to_end(&mut bytes).unwrap();

    let mut reader = Reader::new(&bytes);

    while let Some(index) = memmem::find(reader.read_remaining(), PATTERN) {
        reader.skip(index + 28);

        let export = reader.read_string();
        println!("{}", export);
        exports.push_str(&export);
        exports.push('\n');
    }

    exports
}

fn main() {
    let files = fs::read_dir("./").unwrap();
    for file in files {
        match file {
            Ok(entry) => {
                let path = entry.path();
                let file_name = path.file_name().unwrap().to_string_lossy();
                
                if file_name.ends_with(".sc") && !file_name.ends_with("_tex.sc") {
                    println!("{} \n", file_name);
                    let exports = extract(&path);
                    let output_file_name = format!("extracted_{}.txt", file_name);
                    let file = File::create(output_file_name).expect("Failed to create file");

                    let mut writer = BufWriter::new(file);
                    writer.write_all(exports.as_bytes()).unwrap();
                    writer.flush().unwrap();
                }
            }
            Err(e) => eprintln!("Failed to read directory: {}", e)
        }
    }
}
