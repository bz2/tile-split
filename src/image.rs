use image::io::Reader as ImageReader;
use image::GenericImageView;
use crate::{Config, Error};


fn save_view(img: &image::DynamicImage, x: u32, y: u32, tilesize: u32)  -> Result<(), Error> {
    let x1 = x * tilesize;
    let y1 = y * tilesize;
    let sub = img.view(x1, y1, tilesize, tilesize);
    sub.to_image().save(format!("0_{x}_{y}.png", x=x, y=y))?;
    Ok(())
}

pub fn tile_image(config: Config) -> Result<(), Error> {
    let img = ImageReader::open(config.filename)?.decode()?;

    // Needs to be a loop, for each possible tile in the image
    for x in 0..3 {
        for y in 0..2 {
            save_view(&img, x, y, config.tilesize)?;
        }
    }
    return Ok(());
}
