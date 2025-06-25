use clap::Parser;
use image::{GenericImageView, GrayImage, Luma, imageops::FilterType};
use std::fs;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    input: String,
    #[arg(short, long, default_value = "output.txt")]
    output: String,
    #[arg(short, long, default_value_t = 80)]
    width: u32,
}

const ASCII_CHARS: &[u8] = b"$@B%8&WM#*oahkbdpqwmZO0QLCJUYXzcvunxrjft/()1{}[]?-_+~<>i!lI;:,\"^`'. ";

fn main() {
    let args = Args::parse();

    let img = image::open(&args.input).expect("Failed to open image");
    let (orig_width, orig_height) = img.dimensions();

    let aspect_ratio = orig_height as f32 / orig_width as f32;
    let target_height = (args.width as f32 * aspect_ratio * 0.5) as u32;

    let gray: GrayImage = img.to_luma8();
    let resized = image::imageops::resize(&gray, args.width, target_height, FilterType::Nearest);

    let mut ascii_output = String::new();

    for y in 0..resized.height() {
        for x in 0..resized.width() {
            let pixel: Luma<u8> = *resized.get_pixel(x, y);
            let brightness = pixel[0] as f32 / 255.0;
            let idx = (brightness * (ASCII_CHARS.len() - 1) as f32).round() as usize;
            ascii_output.push(ASCII_CHARS[idx] as char);
        }
        ascii_output.push('\n');
    }

    fs::write(&args.output, ascii_output).expect("Failed to write output file");

    println!("ASCII art saved to '{}'", args.output);
}
