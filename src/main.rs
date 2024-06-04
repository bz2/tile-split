use clap::Parser;
use image::{DynamicImage, ImageResult, SubImage};
use rayon::prelude::*;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use std::str::FromStr;
use std::{ops::RangeInclusive, path::Path};
use tile_split::{Config, Error, TileImage};

fn save_subimage_oxi(
    sub: &SubImage<&DynamicImage>,
    x: &u32,
    y: &u32,
    z: u8,
    folder: &Path,
    config: &Config,
    preset: u8,
) -> Result<(), Error> {
    let path = folder.join(format!("{z}-{x}-{y}.png", z = z, x = x, y = y));
    let png = oxipng::RawImage::new(
        config.tilesize,
        config.tilesize,
        oxipng::ColorType::RGBA,
        oxipng::BitDepth::Eight,
        sub.to_image().into_raw(),
    )?
    .create_optimized_png(&oxipng::Options::from_preset(preset))?;
    let mut file = File::create(path)?;
    file.write_all(&png)?;

    Ok(())
}

fn save_subimage(
    sub: &SubImage<&DynamicImage>,
    x: &u32,
    y: &u32,
    z: u8,
    folder: &Path,
    format: &str,
) -> Result<(), Error> {
    let path = folder.join(format!(
        "{z}-{x}-{y}.{fmt}",
        z = z,
        x = x,
        y = y,
        fmt = format
    ));
    sub.to_image().save(path)?;

    Ok(())
}

fn save_image(img: &DynamicImage, z: u8, folder: &Path, tileformat: &str) -> ImageResult<()> {
    img.save(folder.join(format!("{z}.{fmt}", z = z, fmt = tileformat)))
}

fn parse_range<T>(arg: &str) -> Result<RangeInclusive<T>, <T as FromStr>::Err>
where
    T: FromStr,
{
    let parts: Vec<&str> = arg.splitn(2, &['-', ' ']).collect::<Vec<&str>>();

    match parts.as_slice() {
        [a] => Ok(RangeInclusive::new(a.parse()?, a.parse()?)),
        [a, b] => Ok(RangeInclusive::new(a.parse()?, b.parse()?)),
        _ => unreachable!(),
    }
}

/// Split input image files into sets of tiles.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Input PNG filename.
    filename: PathBuf,

    /// Zoomlevel of input PNG file.
    #[arg(short = 'l', long, env)]
    zoomlevel: u8,

    /// Zoomrange to slice tiles for.
    #[arg(short='r', long, required(false), value_parser = parse_range::<u8>)]
    zoomrange: Option<RangeInclusive<u8>>,

    /// Location to write output tiles to.
    #[arg(short, long, env, required(false), default_value("out"))]
    output_dir: PathBuf,

    /// Dimension of output tiles, in pixels.
    #[arg(long, required(false), default_value("256"))]
    tilesize: u32,

    /// Type of output tiles.
    #[arg(long, env, required(false), default_value("png"), value_parser = Format::from_str)]
    tileformat: Format,

    /// Subset morton range of tiles to slice.
    #[arg(short='t', long, required(false), value_parser = parse_range::<u32>)]
    targetrange: Option<RangeInclusive<u32>>,

    /// PNG compression preset.
    #[arg(long, env, default_value_if("tileformat", "png", "2"), value_parser(clap::value_parser!(u8).range(0..7)))]
    preset: Option<u8>,

    /// Save the resized files
    #[arg(long, env, action)]
    save_resize: bool,
}

fn main() {
    let args = Args::parse();

    if args.preset.is_some() && &args.tileformat != "png" {
        eprintln!(
            "Error: The --preset argument cannot be used with --tileformat set to '{}'",
            &args.tileformat
        );
        std::process::exit(2);
    }

    // create output folder
    std::fs::create_dir_all(&args.output_dir).unwrap();

    let config = Config::new(
        &args.filename,
        args.tilesize,
        args.zoomlevel,
        args.zoomrange,
        args.targetrange,
    );

    // instantiate and load image
    let image = TileImage::new(&config);

    // resize (and save)
    let resized_images =
        RangeInclusive::new(config.startzoomrangetoslice, config.endzoomrangetoslice)
            .into_par_iter()
            .map(|x: u8| {
                let t_size = config.tilesize << x;
                (image.resize(t_size, t_size), x)
            });

    if args.save_resize {
        resized_images
            .for_each(|(img, z)| save_image(&img, z, &args.output_dir, &args.tileformat).unwrap())
    } else {
        // save each sliced image
        resized_images.for_each(|(img, z)| {
            let mut targetrangetoslice: Option<RangeInclusive<u32>> = None;
            // if startzoomrangetoslice is the same as endzoomrangetoslice,
            // then tiles to be sliced in this function are from same zoom level
            if config.startzoomrangetoslice == config.endzoomrangetoslice {
                if z == config.endzoomrangetoslice {
                    targetrangetoslice = Some(config.starttargetrange..=config.endtargetrange);
                }
            // otherwise, the start zoom level should slice tiles from starttargetrange to end,
            // the end zoom level should slice tiles from 0 to endtargetrange
            } else if z == config.startzoomrangetoslice {
                if 1 << (z * 2) > 1 {
                    targetrangetoslice = Some(config.starttargetrange..=(1 << (z * 2)) - 1);
                }
            } else if z == config.endzoomrangetoslice {
                targetrangetoslice = Some(0..=config.endtargetrange);
            }
            image
                .iter_tiles(&img, targetrangetoslice)
                .collect::<Vec<(SubImage<&DynamicImage>, u32, u32)>>()
                .par_iter()
                .for_each(|(sub_img, x, y)| {
                    if &args.tileformat == "png" {
                        save_subimage_oxi(
                            sub_img,
                            x,
                            y,
                            z,
                            &args.output_dir,
                            &config,
                            args.preset.unwrap(),
                        )
                        .unwrap()
                    } else {
                        save_subimage(sub_img, x, y, z, &args.output_dir, &args.tileformat).unwrap()
                    }
                });
        });
    }
}

#[cfg(test)]
mod main_tests;
