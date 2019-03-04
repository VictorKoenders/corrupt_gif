use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;
use corrupted_gif;

fn main() {
    let arg = match std::env::args().nth(1) {
        Some(f) => f,
        None => {
            print_help();
            return;
        }
    };

    let out_file_name = Path::new(&arg).with_extension("gif");
    println!("Creating GIF for {}", arg);

    let mut file = match File::open(arg) {
        Ok(f) => f,
        Err(e) => {
            println!("{:?}", e);
            print_help();
            return;
        }
    };

    let mut buff = Vec::new();
    file.read_to_end(&mut buff).expect("Could not read file");

    let writer = corrupted_gif::generate(&buff);
    File::create(&out_file_name)
        .expect("Could not create output gif")
        .write_all(&writer)
        .unwrap();
}

fn print_help() {
    println!(
        "Usage: {} <filename>",
        std::env::args()
            .nth(0)
            .unwrap_or_else(|| String::from("corrupted_gif"))
    );
}
