pub mod encoder;
pub mod decoder;

use encoder::Encoder;
use decoder::Decoder;
use std::io;


fn main() -> io::Result<()> {

    let img_path = "some_path.png";
    let output_path = "modified_img.png";
    let message = "Super secret message";

    let encoder = Encoder::new(img_path, output_path, message);
    encoder.encode()?;

    let decoder = Decoder::new(output_path);
    let decoded_message = decoder.decode();

    println!("Decoded message: {}", decoded_message);

    Ok(())
}
