mod render;
mod termctl;

use clap::Parser;
use opencv::Result;

/// Encode video into ascii animation
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Video input, either a path "~/test.avi" or a camera id "0/1/..."
    #[clap(short, long, default_value = "0")]
    input: String,
    /// Colorized or not
    #[clap(short, long)]
    colored: bool,
    /// Brightness scale represented with a ASCII string
    #[clap(short, long, default_value = r#" .:=+*#%@"#)]
    scale: String,
    /// Width of output animation
    #[clap(short, long)]
    width: Option<u32>,
    /// Height of output animation
    #[clap(short, long)]
    height: Option<u32>,
}

fn main() -> Result<()> {
    let args = Args::parse();
    let mut buf = String::new();

    if let Ok(img) = opencv::imgcodecs::imread(&args.input, 0) {
        if let Ok(img) = render::resize(&img, args.width, args.height) {
            if let Ok(img) = render::bgr2rgb(img) {
                buf = render::render_ascii(&img, args.colored, &args.scale);
            }
        }
    }
    
    if buf == "" {

    } else {
        print!("{}", buf);
        termctl::clear_screen();
    }
    
    Ok(())
}
