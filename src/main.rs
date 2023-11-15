use clap::Parser;
use image::{DynamicImage, ImageResult, SubImage};
use std::num::ParseIntError;
use std::ops::RangeInclusive;
use std::path::PathBuf;
use tile_split::{Config, Resizer, TileImage};

type PseudoRange<T> = (Option<T>, Option<T>);

fn save_subimage(
    img: &SubImage<&DynamicImage>,
    x: u32,
    y: u32,
    z: u8,
    config: &Config,
) -> ImageResult<()> {
    img.to_image().save(config.folder.join(format!(
        "{z}-{x}-{y}.{fmt}",
        z = z,
        x = x,
        y = y,
        fmt = config.tileformat
    )))
}

fn save_image(img: &DynamicImage, z: u8, config: &Config) -> ImageResult<()> {
    img.save(
        config
            .folder
            .join(format!("{z}.{fmt}", z = z, fmt = config.tileformat)),
    )
}

fn maybe_parse<T: std::str::FromStr>(s: &str) -> Result<Option<T>, T::Err> {
    if s.is_empty() {
        Ok(None)
    } else {
        str::parse(s).map(Some)
    }
}

fn parse_zoomrange(arg: &str) -> Result<PseudoRange<u8>, ParseIntError> {
    Ok(
        match arg
            .splitn(2, &['-', ' '])
            .map(maybe_parse)
            .collect::<Result<Vec<_>, _>>()?[..]
        {
            [a] => (None, a),
            [a, b] => (a, b),
            _ => unreachable!(),
        },
    )
}

/// Split input image files into sets of tiles.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Input PNG filename.
    filename: PathBuf,

    /// Zoomlevel of input PNG file
    #[arg(short = 'l', long, env)]
    zoomlevel: u8,

    /// Zoomrange to slice tiles for, currently unused.
    #[arg(short='r', long, required(false), value_parser = parse_zoomrange)]
    zoomrange: PseudoRange<u8>,

    /// Location to write output tiles to.
    #[arg(short, long, env, required(false), default_value("out"))]
    output_dir: PathBuf,

    /// Dimension of output tiles, in pixels.
    #[arg(long, required(false), default_value("256"))]
    tilesize: u32,

    /// Type of output tiles, currently unused.
    #[arg(long, env, required(false), default_value("png"))]
    tileformat: String,

    /// Save the resized files
    #[arg(long, env, action)]
    save_resize: bool,
}

fn derive_range(args: &Args) -> RangeInclusive<u8> {
    match args.zoomrange {
        // TODO: Make the default 0..=max perhaps?
        (None, None) => args.zoomlevel..=args.zoomlevel,
        (Some(start), None) => start..=args.zoomlevel,
        (None, Some(end)) => 0..=end,
        (Some(start), Some(end)) => start..=end,
    }
}

fn main() {
    let args = Args::parse();

    let config = Config {
        tilesize: args.tilesize,
        filename: &args.filename,
        zoomlevel: args.zoomlevel,
        zoomrange: derive_range(&args),
        folder: &args.output_dir,
        tileformat: &args.tileformat,
    };
    let save_resized = args.save_resize;

    // create output folder
    std::fs::create_dir_all(config.folder).unwrap();

    // instantiate TileImage
    let tile_image = TileImage { config: &config };
    let image = &tile_image.open_img().unwrap();

    // resize (and save)
    let resized_images = config.resize_range(image);

    if save_resized {
        resized_images.for_each(|(img, z)| save_image(&img, z, &config).unwrap())
    } else {
        // save each sliced image
        resized_images.for_each(|(img, z)| {
            tile_image
                .iter(&img)
                .for_each(|(sub_img, x, y)| save_subimage(&sub_img, x, y, z, &config).unwrap());
        });
    }
}
