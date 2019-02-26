use image::GenericImageView;
use pbr::ProgressBar;
use rand::Rng;
use rayon::prelude::*;
use std::borrow::Cow;
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;

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
    let buff = buff;

    let mut rand = rand::thread_rng();
    let format = image::guess_format(&buff).expect("Could not guess image format");
    if format != image::ImageFormat::JPEG {
        println!("This program has only been tested to work with JPG/JPEG");
        println!("Use at own risk");
    }
    let dimensions = image::load_from_memory_with_format(&buff, format)
        .expect("Could not load image")
        .dimensions();
    let dimensions = (dimensions.0 as u16, dimensions.1 as u16);

    // The gif ends up being about 8 times the size of the original image size
    // And we generate up to 120 frames
    // So we reserve a buffer of roughly that size so we don't have to resize later
    let initial_capacity = buff.len() * 120 * 8;
    let mut writer = Vec::with_capacity(initial_capacity);

    {
        let mut encoder = gif::Encoder::new(&mut writer, dimensions.0, dimensions.1, &[])
            .expect("Could not create gif encoder");

        println!("Corrupting the buffers");

        let mut frames = Vec::with_capacity(120);

        frames.push((0, buff.clone()));

        for _ in 0..119 {
            let index = rand.gen_range(0, buff.len());
            let value = rand.gen_range(0, u16::from(std::u8::MAX) + 1) as u8;
            let mut buff = buff.clone();
            buff[index] = value;
            frames.push((index, buff));
        }
        println!("Starting the generation of frames");

        let frames = frames
            .par_iter()
            .filter_map(|(index, buff)| {
                let image = match image::load_from_memory_with_format(&buff, format) {
                    Err(e) => {
                        println!(
                            "Could not load image, tried changing byte at {}: {:?}",
                            index, e
                        );
                        return None;
                    }
                    Ok(image) => image,
                };

                let rgba_image = image.to_rgba();
                let raw = rgba_image.into_raw();

                let mut frame = gif::Frame::default();
                frame.width = dimensions.0;
                frame.height = dimensions.1;
                frame.buffer = Cow::Owned(raw);
                frame.delay = 10;

                Some(frame)
            })
            .collect::<Vec<_>>();

        println!("Combining the frames into a gif, this can take a second");
        let mut pb = ProgressBar::new((frames.len() as u64).min(100));
        pb.format("[=> ]");
        for frame in frames.into_iter().take(100) {
            encoder.write_frame(&frame).expect("Could not add frame");
            pb.inc();
        }
        pb.finish_print("Done");
    }

    println!("Writing {} bytes to the output gif", writer.len());
    println!(
        "Len is {}% of the initial capacity ({} => {})",
        writer.len() * 100 / initial_capacity,
        initial_capacity,
        writer.len()
    );
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
