use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[arg(short = 'e', long = "encode", requires_all = ["output_path", "message"])]
    pub encode: bool,

    #[arg(short = 'd', long = "decode")]
    pub decode: bool,

    pub img_path: String,

    #[arg(required_if_eq("encode", "true"))]
    pub output_path: Option<String>,

    #[arg(required_if_eq("encode", "true"))]
    pub message: Option<String>,
}
