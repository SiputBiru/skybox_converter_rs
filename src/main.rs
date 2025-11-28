use clap::{Parser, ValueEnum};
use std::path::PathBuf;
use std::time::Instant;

use eq2c::{self, codecs::ToneMapType};

#[derive(Parser)]
#[command(
    author,
    version,
    about,
    after_help = "EXAMPLES:\n  \
        # Convert HDR to 6 Separate PNGs\n  \
        eq2c -i input.exr -o skybox.png --layout separate\n\n  \
        # Convert to Horizontal Strip\n  \
        eq2c -i input.hdr -o strip.png --layout strip-h"
)]
struct Cli {
    #[arg(short, long)]
    input: PathBuf,

    #[arg(short, long)]
    output: PathBuf,

    #[arg(short, long, value_enum, default_value_t = FormatArg::Png)]
    format: FormatArg,

    #[arg(short, long, value_enum, default_value_t = LayoutArg::Cross)]
    layout: LayoutArg,

    #[arg(short, long, value_enum, default_value_t = ToneMapType::Aces)]
    tonemap: ToneMapType,

    #[arg(short, long, default_value_t = 1.0)]
    exposure: f32,

    #[arg(short, long, default_value_t = 512)]
    size: u32,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum FormatArg {
    Png,
    Exr,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum LayoutArg {
    Cross,
    StripH,
    StripV,
    Separate,
}

fn main() {
    let args = Cli::parse();
    let start = Instant::now();

    let config = eq2c::Config {
        input: args.input,
        output: args.output,
        format: match args.format {
            FormatArg::Png => eq2c::OutputFormat::Png,
            FormatArg::Exr => eq2c::OutputFormat::Exr,
        },
        layout: match args.layout {
            LayoutArg::Cross => eq2c::LayoutType::Cross,
            LayoutArg::StripH => eq2c::LayoutType::StripHorizontal,
            LayoutArg::StripV => eq2c::LayoutType::StripVertical,
            LayoutArg::Separate => eq2c::LayoutType::Separate,
        },
        tonemap: args.tonemap,
        exposure: args.exposure,
        size: args.size,
    };

    if let Err(e) = eq2c::run(config) {
        eprintln!("Error: {e}");
        std::process::exit(1);
    }

    println!("Total Time: {:?}", start.elapsed());
}
