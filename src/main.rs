extern crate gif;
extern crate rand;
extern crate image;

use image::GenericImage;
use std::io::Read;
use std::fs::File;
use std::path::Path;
use rand::Rng;


fn main() {
    let arg = match std::env::args().nth(1) {
        Some(f) => f,
        None => {
            print_help();
            return;
        }
    };

    let out_file_name = Path::new(&arg).with_extension("gif");

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

    let mut out_file = File::create(out_file_name).expect("Could not create output gif");
    let mut rand = rand::thread_rng();
    println!("{:?}", &buff[..10]);
    let format = image::guess_format(&buff).expect("Could not guess image format");
    if format != image::ImageFormat::JPEG {
        println!("This program has only been tested to work with JPG/JPEG");
        println!("Use at own risk");
    }
    let dimensions = image::load_from_memory_with_format(&buff, format).expect("Could not load image").dimensions();
    let dimensions = (dimensions.0 as u16, dimensions.1 as u16);
    
    let mut encoder = gif::Encoder::new(&mut out_file, dimensions.0, dimensions.1, &[]).expect("Could not create gif encoder");
    add_frame(&buff, format, &mut encoder, &dimensions).expect("Could not write first frame");

    for i in 0..100 {
        loop {
            // corrupt a random pixel
            let index = rand.gen_range(0, buff.len());
            buff[index] = 0;

            if let Ok(()) = add_frame(&buff, format, &mut encoder, &dimensions) {
                break;
            }
            println!("Could not clear pixel at index {}", index);
        }
        println!("{}%", i + 1);
    }
}

// Create image from the in-memory buffer
fn add_frame(buff: &[u8], format: image::ImageFormat, encoder: &mut gif::Encoder<&mut File>, dimensions: &(u16, u16)) -> Result<(), ()> {
    let image = image::load_from_memory_with_format(&buff, format).map_err(|_| ())?;
    let rgba_image = image.to_rgba();
    let mut raw = rgba_image.into_raw();

    // Add the image to the gif encoder
    let mut frame = gif::Frame::from_rgba(dimensions.0, dimensions.1, &mut raw);
    frame.delay = 10;
    encoder.write_frame(&frame).expect("Could not add frame");

    Ok(())
}

fn print_help(){
    println!("Usage: {} <filename>", std::env::args().nth(0).unwrap_or_else(|| String::from("corrupted_gif")));
}