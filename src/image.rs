use image::io::Reader as ImageReader;
use image::GenericImage;
use crate::{Config, Error};


fn save_sub_image(img: &mut image::DynamicImage, x: u32, y: u32, tilesize: u32)  -> Result<(), Error> {
    let x1 = x * tilesize;
    let y1 = y * tilesize;
    let sub = img.sub_image(x1, y1, tilesize, tilesize);
    sub.to_image().save(format!("0_{x}_{y}.png", x=x, y=y))?;
    Ok(())
}

pub fn tile_image(config: Config) -> Result<(), Error> {
    let mut img = ImageReader::open(config.filename)?.decode()?;

    // Needs to be a loop, for each possible tile in the image
    for x in 0..3 {
        for y in 0..2 {
            save_sub_image(&mut img, x, y, config.tilesize)?;
        }
    }
    return Ok(());
}
