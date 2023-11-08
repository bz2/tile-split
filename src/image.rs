use std::fs::File;
use std::io::Write;
use crate::{Config, Error};
use image::io::Reader as ImageReader;
use image::{DynamicImage, GenericImageView};

fn save_view(img: &DynamicImage, x: u32, y: u32, config: &Config) -> Result<(), Error> {
    let x1 = x * config.tilesize;
    let y1 = y * config.tilesize;
    let sub = img.view(x1, y1, config.tilesize, config.tilesize);
    let path = format!("{p}/{z}_{x}_{y}.png", p=config.folder, z=config.zoomlevel, x = x, y = y);
    let png = oxipng::RawImage::new(
        config.tilesize,
        config.tilesize,
        oxipng::ColorType::RGBA,
        oxipng::BitDepth::Eight,
        sub.to_image().into_raw()
    )?.create_optimized_png(&oxipng::Options::from_preset(2))?;
    let mut file = File::create(path)?;
    file.write_all(&png)?;

    Ok(())
}

pub fn tile_image(config: Config) -> Result<(), Error> {
    let img = ImageReader::open(config.filename)?.decode()?;

    // Needs to be a loop, for each possible tile in the image
    for x in 0..(img.width() / config.tilesize) {
        for y in 0..(img.height() / config.tilesize) {
            save_view(&img, x, y, &config)?;
        }
    }
    return Ok(());
}
