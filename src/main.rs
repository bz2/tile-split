use std::ops::RangeInclusive;
use clap::Parser;
use tile_split::{Config, Error, TileImage};

fn parse_zoomrange(arg: &str) -> Result<RangeInclusive<u8>, Error> {
    match arg.splitn(2, &['-', ' ']).map(str::parse).collect::<Result<Vec<_>, _>>()?[..] {
        [a] => Ok(RangeInclusive::new(a, a)),
        [a, b] => Ok(RangeInclusive::new(a, b)),
        _ => unreachable!(),
    }
}

/// Split input image files into sets of tiles.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Input PNG filename.
    filename: std::path::PathBuf,

    /// Zoomlevel of input PNG file
    #[arg(short='l', long, env)]
    zoomlevel: u8,

    /// Zoomrange to slice tiles for, currently unused.
    #[arg(short='r', long, required(false), value_parser = parse_zoomrange)]
    zoomrange: RangeInclusive<u8>,

    /// Location to write output tiles to.
    #[arg(short, long, env, required(false), default_value("out"))]
    output_dir: std::path::PathBuf,

    /// Dimension of output tiles, in pixels.
    #[arg(short='s', long, required(false), default_value("256"))]
    tilesize: u32,

    /// Type of output tiles, currently unused.
    #[arg(short='f', long, env, required(false), default_value("png"))]
    tileformat: String,
}

fn main() {
    let args = Args::parse();

    let config = Config {
            tilesize: args.tilesize,
            filename: &args.filename,
            zoomlevel: args.zoomlevel,
            folder: &args.output_dir,
            tileformat: &args.tileformat,
    };
    
    // create output folder
    std::fs::create_dir_all(&config.folder).unwrap();

    let zoom = config.zoomlevel;
    // instantiate TileImage
    let tile_image = TileImage{
        config: &config,
    };
    // save each sliced image
    // TODO: this is too long and unreadable
    tile_image.iter(&tile_image.create_img().unwrap()).for_each(|(img, x, y)| img.to_image().save(config.folder.join(format!("{z}_{x}_{y}.png", z=zoom, x = x, y = y))).unwrap());
}
