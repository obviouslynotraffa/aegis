use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// Encode a message into an image
    #[arg(short = 'e', long = "encode", requires = "message")]
    pub encode: bool,

    /// Decode a message from an image
    #[arg(short = 'd', long = "decode")]
    pub decode: bool,

    /// Path to the input image
    pub img_path: String,

    /// Path and name of the output image (for encoding only)
    #[arg(short = 'o', long, requires = "encode")]
    pub output_path: Option<String>,

    /// Message to encode (required for encoding)
    #[arg(short = 'm', long, requires = "encode")]
    pub message: Option<String>,
}
