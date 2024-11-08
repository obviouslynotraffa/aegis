pub mod encoder;
pub mod decoder;
pub mod cli;

use encoder::Encoder;
use decoder::Decoder;
use cli::Cli;
use clap::Parser;
use std::io;


fn main() -> io::Result<()> {

    let args = Cli::parse();

    if args.encode {
        let output_path = args.output_path.unwrap();
        let message = args.message.unwrap();
        let encoder = Encoder::new(&args.img_path, &output_path, &message);
        encoder.encode()?;
    } else if args.decode {
        let decoder = Decoder::new(&args.img_path);
        let decoded_message = decoder.decode();
        println!("Decoded message: {}", decoded_message);
    }
 
    Ok(())
}
